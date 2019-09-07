pub fn split_response(text: &str) -> (&str, &str) {
    let s = text.to_string();
    let end = s.trim_end().len();
    match text.chars().position(|c| c == ',') {
        Some(pos) => {
            let code = &text[..pos];
            let data = &text[pos + 1 .. end];
            (code, data)
        },
        None => {
            ("", "")
        }
    }
}
