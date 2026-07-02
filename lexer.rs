pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    // None = normal mode
    // Some('"') = inside double quotes
    // Some('\'') = inside single quotes
    let mut quote: Option<char> = None;

    for ch in input.chars() {
        match quote {
            // -------------------------
            // NORMAL MODE
            // -------------------------
            None => {
                match ch {
                    '"' | '\'' => {
                        quote = Some(ch);
                    }
                    '|' | '<' | '>' => {
                        if !current.is_empty() {
                            tokens.push(current.clone());
                            current.clear();
                        }

                        tokens.push(ch.to_string());
                    }
                    // Whitespace
                    c if c.is_whitespace() => {
                        if !current.is_empty() {
                            tokens.push(current.clone());
                            current.clear();
                        }
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            }
            // QUOTE MODE
            Some(q) => {
                if ch == q {
                    // Closing quote
                    quote = None;
                } else {
                    current.push(ch);
                }
            }
        }
    }
    if quote.is_some() {
        println!("Lexer Error: Unterminated quote");
    }
    // Push the last token
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}