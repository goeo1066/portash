pub fn parse_prompt(line: &str) -> Vec<String> {
    let mut in_quote = false;
    let mut quote_char = '"';
    let mut was_escape = false;

    let mut result: Vec<String> = vec![];
    let mut buffer: Vec<char> = vec![];
    for e in line.chars() {
        let c = e;
        if !was_escape {
            if !in_quote && (c == '\'' || c == '\"') {
                in_quote = true;
                quote_char = c;
                continue;
            }

            if in_quote && (c == quote_char) {
                in_quote = false;
                continue;
            }

            if c == '\\' {
                was_escape = true;
                continue;
            }
        }

        if !in_quote && c.is_whitespace() {
            result.push(buffer.iter().cloned().collect());
            buffer = vec![];
            continue;
        }

        buffer.push(c);
        was_escape = false;
    }

    if !buffer.is_empty() {
        result.push(buffer.iter().cloned().collect());
    }

    // remove whitespaces
    result
        .iter()
        .filter(|it| !it.trim().is_empty())
        .map(|it| it.into())
        .collect()
}
