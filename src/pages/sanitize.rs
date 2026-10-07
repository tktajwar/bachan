use sanitize_html::{
    rules::predefined::DEFAULT,
    sanitize_str,
};

pub fn sanitize(input: &str) -> String {
    let input_trimmed = input.trim();

    let input_escaped = input_trimmed.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;");

    sanitize_str(&DEFAULT, &input_escaped).unwrap()
}
