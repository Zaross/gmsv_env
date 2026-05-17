use std::collections::HashMap;

pub fn strip_inline_comment(line: &str) -> &str {
    let mut in_quotes: Option<char> = None;
    let mut is_escaped = false;
    let mut should_escape_end = false;

    for (byte_idx, ch) in line.char_indices() {
        if is_escaped {
            should_escape_end = true;
        }
        if ch == '\\' {
            is_escaped = true;
            should_escape_end = false;
        }
        if (ch == '"' || ch == '\'') && !is_escaped && in_quotes.is_none() {
            in_quotes = Some(ch);
        } else if Some(ch) == in_quotes && !is_escaped {
            in_quotes = None;
        }
        if should_escape_end {
            should_escape_end = false;
            is_escaped = false;
        }
        if ch == '#' && in_quotes.is_none() {
            return &line[..byte_idx];
        }
    }
    line
}

pub fn parse(body: &str) -> (HashMap<String, String>, Vec<String>) {
    let mut output: HashMap<String, String> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    let body = body.replace('\r', "");

    let mut cleaned = String::with_capacity(body.len());
    let mut prev_nl = false;
    for ch in body.chars() {
        if ch == '\n' {
            if !prev_nl {
                cleaned.push(ch);
            }
            prev_nl = true;
        } else {
            prev_nl = false;
            cleaned.push(ch);
        }
    }

    let body = cleaned.trim().to_string();
    let lines: Vec<&str> = body.lines().collect();

    for &line in lines.iter().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let line = strip_inline_comment(line);

        match line.find('=') {
            Some(eq_pos) => {
                let key = line[..eq_pos].trim().to_string();
                if key.is_empty() {
                    errors.push(format!("Invalid Key: {}", line));
                    continue;
                }

                let raw = line[eq_pos + 1..].trim();
                if raw.is_empty() {
                    output.remove(&key);
                    continue;
                }

                let value = if raw.len() >= 2
                    && ((raw.starts_with('"') && raw.ends_with('"'))
                        || (raw.starts_with('\'') && raw.ends_with('\'')))
                {
                    raw[1..raw.len() - 1].to_string()
                } else {
                    raw.to_string()
                };

                output.insert(key, value);
            }
            None => {
                errors.push(format!("Invalid Key: {}", line));
            }
        }
    }

    (output, errors)
}
