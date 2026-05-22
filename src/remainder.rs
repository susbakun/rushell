pub fn parse_args(remainder: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let (mut is_single_quote, mut is_double_quote) = (false, false);
    let mut backslash_seen = false;

    for ch in remainder.chars() {
        if backslash_seen {
            current.push(ch);
            backslash_seen = false;
            continue;
        }

        match ch {
            '\'' | '"' => {
                if is_double_quote {
                    if ch == '"' {
                        is_double_quote = !is_double_quote;
                    } else {
                        current.push(ch);
                    }
                } else if is_single_quote {
                    if ch == '\'' {
                        is_single_quote = !is_single_quote;
                    } else {
                        current.push(ch);
                    }
                } else {
                    if ch == '\'' {
                        is_single_quote = !is_single_quote;
                    } else {
                        is_double_quote = !is_double_quote;
                    }
                }
            }
            '\\' => {
                if is_single_quote {
                    current.push(ch);
                } else {
                    backslash_seen = true;
                }
            }
            ' ' if !is_single_quote && !is_double_quote => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

pub fn process_remainder(remainder: &str) -> String {
    let mut formated = String::new();
    let (mut is_single_quote, mut is_double_quote) = (false, false);
    let mut backslash_seen = false;
    let mut space_count = 0;

    for ch in remainder.chars() {
        if backslash_seen {
            formated.push(ch);
            backslash_seen = false;
        } else if ch == '\'' || ch == '"' {
            if is_double_quote {
                if ch == '"' {
                    is_double_quote = !is_double_quote;
                } else {
                    formated.push(ch);
                }
            } else if is_single_quote {
                if ch == '\'' {
                    is_single_quote = !is_single_quote;
                } else {
                    formated.push(ch);
                }
            } else {
                if ch == '\'' {
                    is_single_quote = !is_single_quote;
                } else {
                    is_double_quote = !is_double_quote;
                }
            }
        } else if ch == '\\' {
            if is_double_quote {
                backslash_seen = true;
            }
        } else if is_double_quote || is_single_quote {
            formated.push(ch);
        } else if ch == ' ' {
            space_count += 1;
            if space_count < 2 {
                formated.push(ch);
            }
        } else {
            formated.push(ch);
        }

        if ch != ' ' {
            space_count = 0;
        }
    }

    formated
}
