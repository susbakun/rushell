pub fn parse_args(remainder: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;

    for ch in remainder.chars() {
        match ch {
            '\'' | '"' => in_single_quote = !in_single_quote,
            ' ' if !in_single_quote => {
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
    let mut is_quote = false;
    let mut space_count = 0;

    for ch in remainder.chars() {
        if ch == '\'' || ch == '"' {
            is_quote = !is_quote;
        } else if is_quote {
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
