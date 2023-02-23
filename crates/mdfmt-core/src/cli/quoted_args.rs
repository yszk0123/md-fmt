pub fn parse_quoted_args(input: &str) -> Vec<String> {
    let mut chunks = vec![];
    let mut chunk = String::new();
    let mut chars = input.chars();
    let mut quoted = false;
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if quoted {
                    chunks.push(chunk);
                    chunk = String::new();
                }
                quoted = !quoted;
            },
            '\\' => {
                if let Some(n) = chars.next() {
                    chunk.push(n);
                }
            },
            ' ' => {
                if quoted {
                    chunk.push(c);
                } else if !chunk.is_empty() {
                    chunks.push(chunk);
                    chunk = String::new();
                }
            },
            v => {
                chunk.push(v);
            },
        }
    }
    if !chunk.is_empty() {
        chunks.push(chunk);
    }
    chunks
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(parse_quoted_args(""), Vec::<String>::new())
    }

    #[test]
    fn unquoted_string() {
        assert_eq!(parse_quoted_args("foo"), vec!["foo"])
    }

    #[test]
    fn quoted_string() {
        assert_eq!(parse_quoted_args("\"foo bar\""), vec!["foo bar"])
    }

    #[test]
    fn escaped_string() {
        assert_eq!(parse_quoted_args("\"fo\\\"o\""), vec!["fo\"o"])
    }

    #[test]
    fn multiple_unquoted_string() {
        assert_eq!(parse_quoted_args("foo bar"), vec!["foo", "bar"])
    }

    #[test]
    fn multiple_quoted_string() {
        assert_eq!(
            parse_quoted_args("\"foo bar\" \"bar baz\""),
            vec!["foo bar", "bar baz"]
        )
    }

    #[test]
    fn multiple_escaped_string() {
        assert_eq!(
            parse_quoted_args("\"foo b\\\"ar\" \"bar baz\""),
            vec!["foo b\"ar", "bar baz"]
        )
    }
}
