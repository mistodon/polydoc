use regex::Match;
use DocItem;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Token
{
    token_type: TokenType,
    start: usize,
    end: usize
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TokenType
{
    Single,
    MultiOpen,
    MultiClose,
    Newline
}


fn parse_token(delim: Option<&Match>) -> Option<Token>
{
    match delim
    {
        None => None,
        Some(delim) => match delim.as_str()
        {
            "///" => Some(Token { token_type: TokenType::Single, start: delim.start(), end: delim.end() }),
            "/**" => Some(Token { token_type: TokenType::MultiOpen, start: delim.start(), end: delim.end() }),
            "*/" => Some(Token { token_type: TokenType::MultiClose, start: delim.start(), end: delim.end() }),
            "\n" => Some(Token { token_type: TokenType::Newline, start: delim.start(), end: delim.end() }),
            _ => unreachable!()
        }
    }
}


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

    while let Some(token) = parse_token(delims.peek())
    {
        match token.token_type
        {
            TokenType::Newline => line += 1,
            TokenType::Single =>
            {
                // Parsing single-line comments

                let mut buffer = String::new();
                let start_line = line;
                let mut end_line = start_line;
                let mut comment_start = None;

                while let Some(token) = parse_token(delims.peek())
                {
                    if token.token_type == TokenType::Newline
                        { line += 1; }

                    match (comment_start, token.token_type)
                    {
                        (Some(start_pos), TokenType::Newline) =>
                        {
                            buffer.push_str(&source_str[start_pos..token.end]);
                            comment_start = None;
                        }
                        (None, TokenType::Single) =>
                        {
                            comment_start = Some(token.end);
                            end_line = line;
                        }
                        (None, _) => break,
                        _ => ()
                    }

                    delims.next();
                }

                if let Some(start_pos) = comment_start
                {
                    buffer.push_str(&source_str[start_pos..]);
                }

                docs.push(DocItem { start_line, end_line, text: buffer });
            }
            TokenType::MultiOpen =>
            {
                // Parsing multi-line comments
            }
            TokenType::MultiClose => ()
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