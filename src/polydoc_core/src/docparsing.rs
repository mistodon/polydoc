use DocItem;


pub fn extract_docs<S>(source: S) -> Vec<DocItem>
where
    S: AsRef<str>
{
    use regex::Regex;

    let mut docs = Vec::new();

    let mut line = 0;

    let delim_regex = Regex::new("/\\*\\*|\\*/|///|\n").expect("Bad regex");
    let source_str = source.as_ref();

    let mut delims = delim_regex.find_iter(source_str).peekable();

    #[derive(Debug)]
    enum Token
    {
        None,
        Single,
        MultiOpen,
        MultiClose,
        Newline
    }

    // Top-level parsing
    loop
    {
        let token = match delims.peek()
        {
            None => Token::None,
            Some(delim) => match delim.as_str()
            {
                "///" => Token::Single,
                "/**" => Token::MultiOpen,
                "*/" => Token::MultiClose,
                "\n" => Token::Newline,
                _ => unreachable!()
            }
        };

        match token
        {
            Token::Newline => line += 1,
            Token::Single =>
            {
                // Parsing single-line comments

                let mut buffer = String::new();
                let start_line = line;
                let mut end_line = start_line;
                let mut comment_start = None;

                loop
                {
                    let (token, _start, end) = match delims.peek()
                    {
                        None => (Token::None, 0, 0),
                        Some(delim) => match delim.as_str()
                        {
                            "///" => (Token::Single, delim.start(), delim.end()),
                            "/**" => (Token::MultiOpen, delim.start(), delim.end()),
                            "*/" => (Token::MultiClose, delim.start(), delim.end()),
                            "\n" => (Token::Newline, delim.start(), delim.end()),
                            _ => unreachable!()
                        }
                    };

                    if let Token::Newline = token
                    {
                        line += 1;
                    }

                    match (comment_start, token)
                    {
                        (Some(start_pos), Token::Newline) =>
                        {
                            buffer.push_str(&source_str[start_pos..end]);
                            comment_start = None;
                        }
                        (None, Token::Single) =>
                        {
                            comment_start = Some(end);
                            end_line = line;
                        }
                        (None, _) | (_, Token::None) => break,
                        _ => ()
                    }

                    delims.next();
                }

                // ^ Case (Some(start_pos), Token::None)
                if let Some(start_pos) = comment_start
                {
                    buffer.push_str(&source_str[start_pos..]);
                }

                docs.push(DocItem { start_line, end_line, text: buffer });
            }
            Token::MultiOpen =>
            {
                // Parsing multi-line comments
            }
            Token::MultiClose => (),
            Token::None => break
        }

        delims.next();
    }

    docs
}


#[cfg(test)]
mod tests
{
    use super::*;

    fn test_extraction(source: &str, expected: &Vec<DocItem>)
    {
        let docs = extract_docs(source);
        assert_eq!(&docs, expected);
    }

    fn docitem(start_line: u64, end_line: u64, text: &str) -> DocItem
    {
        DocItem
        {
            start_line,
            end_line,
            text: text.to_owned()
        }
    }

    #[test]
    fn single_line_comment()
    {
        test_extraction("/// Comment\n", &vec![docitem(0, 0, " Comment\n")]);
    }

    #[test]
    fn single_line_comment_without_newline()
    {
        test_extraction("/// Comment", &vec![docitem(0, 0, " Comment")]);
    }

    #[test]
    fn single_comment_block()
    {
        test_extraction("/// First line\n/// Second line\n", &vec![docitem(0, 1, " First line\n Second line\n")]);
    }

    #[test]
    fn separate_single_comments()
    {
        test_extraction("/// First comment\n\n/// Second comment", &vec![docitem(0, 0, " First comment\n"), docitem(2, 2, " Second comment")])
    }

    #[test]
    fn indented_comment_block()
    {
        test_extraction(r#"
            /// First line
            /// Second line
        "#, &vec![docitem(1, 2, " First line\n Second line\n")]);
    }

    #[test]
    fn badly_indented_comment_block()
    {
        test_extraction(r#"
            /// First line
        /// Second line
        "#, &vec![docitem(1, 2, " First line\n Second line\n")]);
    }
}