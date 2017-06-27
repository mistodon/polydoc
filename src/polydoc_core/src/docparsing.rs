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


pub fn extract_docs(source: &str) -> Vec<DocItem>
{
    use regex::Regex;

    let mut docs = Vec::new();

    let mut line = 0;
    let mut line_start = 0;

    let delim_regex = Regex::new("/\\*\\*|\\*/|///|\n").expect("Bad regex");
    let source_str = source.as_ref();

    let mut delims = delim_regex.find_iter(source_str).peekable();

    while let Some(token) = parse_token(delims.peek())
    {
        match token.token_type
        {
            TokenType::Newline =>
            {
                line += 1;
                line_start = token.end;
            }

            TokenType::Single =>
            {
                // TODO: Pull out into function to reduce rightward drift
                {
                    let mut buffer = String::new();
                    let start_line = line;
                    let mut end_line = start_line;
                    let mut comment_start = None;

                    while let Some(token) = parse_token(delims.peek())
                    {
                        match (comment_start, token.token_type)
                        {
                            (Some(start_pos), TokenType::Newline) =>
                            {
                                let line_str = &source_str[start_pos..token.end];
                                buffer.push_str(line_str.trim());
                                buffer.push('\n');
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

                        if token.token_type == TokenType::Newline
                        {
                            line += 1;
                            line_start = token.end;
                        }

                        delims.next();
                    }

                    if let Some(start_pos) = comment_start
                    {
                        let line_str = &source_str[start_pos..];
                        buffer.push_str(line_str.trim());
                    }

                    docs.push(DocItem { start_line, end_line, text: buffer.trim().to_owned() });
                }

                continue;
            }

            TokenType::MultiOpen =>
            {
                // TODO: Pull out into function to reduce rightward drift
                {
                    let mut buffer = String::new();
                    let mut start_pos = token.end;
                    let mut block_indent = None;
                    let start_line = line;

                    while let Some(token) = parse_token(delims.peek())
                    {
                        match token.token_type
                        {
                            TokenType::Newline | TokenType::MultiClose =>
                            {
                                let line_str = &source_str[start_pos..token.start].trim_right();

                                let text_start = {
                                    let entire_line_str = &source_str[line_start..token.start].trim_right();
                                    line_start + (entire_line_str.len() - line_str.trim_left().len())
                                };
                                let line_indent = text_start - line_start;

                                let trimmed_line = match line_str.len()
                                {
                                    0 => line_str,
                                    _ => match block_indent
                                    {
                                        Some(indent) if line_indent >= indent => &line_str[indent..],
                                        _ =>
                                        {
                                            block_indent = Some(line_indent);
                                            line_str.trim_left()
                                        }
                                    }
                                };

                                buffer.push_str(trimmed_line);

                                match token.token_type
                                {
                                    TokenType::Newline =>
                                    {
                                        buffer.push('\n');
                                        start_pos = token.end;

                                        line += 1;
                                        line_start = token.end;
                                    }
                                    TokenType::MultiClose =>
                                    {
                                        let end_line = line;
                                        docs.push(DocItem { start_line, end_line, text: buffer.trim().to_owned() });
                                        break;
                                    }
                                    _ => unreachable!()
                                }
                            }
                            _ => ()
                        }

                        delims.next();
                    }
                }

                continue;
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
        test_extraction("///Comment", &vec![docitem(0, 0, "Comment")]);
    }

    #[test]
    fn single_line_comment_strips_leading_whitespace()
    {
        test_extraction("///      Comment", &vec![docitem(0, 0, "Comment")]);
    }

    #[test]
    fn single_line_comment_with_stripped_trailing_newline()
    {
        test_extraction("///Comment\n", &vec![docitem(0, 0, "Comment")]);
    }

    #[test]
    fn single_comment_block()
    {
        test_extraction("/// First line\n/// Second line\n", &vec![docitem(0, 1, "First line\nSecond line")]);
    }

    #[test]
    fn separate_single_comments()
    {
        test_extraction("/// First comment\n\n/// Second comment", &vec![docitem(0, 0, "First comment"), docitem(2, 2, "Second comment")])
    }

    #[test]
    fn indented_comment_block()
    {
        test_extraction(r#"
            /// First line
            /// Second line
        "#, &vec![docitem(1, 2, "First line\nSecond line")]);
    }

    #[test]
    fn badly_indented_comment_block()
    {
        test_extraction(r#"
            /// First line
        /// Second line
        "#, &vec![docitem(1, 2, "First line\nSecond line")]);
    }

    #[test]
    fn single_line_multiline_comment()
    {
        test_extraction("/**Comment*/", &vec![docitem(0, 0, "Comment")]);
    }

    #[test]
    fn single_line_multiline_comment_strips_leading_whitespace()
    {
        test_extraction("/**       Comment*/", &vec![docitem(0, 0, "Comment")]);
    }

    #[test]
    fn single_line_multiline_comment_with_stripped_trailing_newline()
    {
        test_extraction("/**Comment\n*/", &vec![docitem(0, 1, "Comment")]);
    }

    #[test]
    fn two_line_multiline_comment()
    {
        test_extraction("/**First line\nSecond line*/", &vec![docitem(0, 1, "First line\nSecond line")]);
    }

    #[test]
    fn two_line_multiline_comment_trailing_newline()
    {
        test_extraction("/**First line\nSecond line\n*/", &vec![docitem(0, 2, "First line\nSecond line")]);
    }

    #[test]
    fn two_multiline_comments()
    {
        test_extraction("/** First comment */\n/** Second comment */",
            &vec![
                docitem(0, 0, "First comment"),
                docitem(1, 1, "Second comment")]);
    }

    #[test]
    fn indented_multiline_comment()
    {
        test_extraction(r#"
        /**
            First line
                Second line
        */"#, &vec![docitem(1, 4, "First line\n    Second line")]);
    }

    #[test]
    fn indented_multiline_comment_without_trailing_newline()
    {
        test_extraction(r#"
        /**
            First line
                Second line */"#, &vec![docitem(1, 3, "First line\n    Second line")]);
    }

    #[test]
    fn badly_indented_multiline_comment()
    {
        test_extraction(r#"
        /** First line
        Second line
            Third line
        */"#, &vec![docitem(1, 4, "First line\nSecond line\n    Third line")]);
    }

    #[test]
    fn single_then_multi_comment()
    {
        test_extraction("/// First comment\n/** Second comment */",
            &vec![
                docitem(0, 0, "First comment"),
                docitem(1, 1, "Second comment")])
    }

    #[test]
    fn multi_then_single_comment()
    {
        test_extraction("/** First comment */\n/// Second comment",
            &vec![
                docitem(0, 0, "First comment"),
                docitem(1, 1, "Second comment")])
    }
}