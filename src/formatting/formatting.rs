// use std::sync::LazyLock;
use regex::Regex;

// static RE_TAG_NUMBER: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(
// 	r"^\s*@([0-9]+\.?[0-9]*)\s*$"
//     ).unwrap());

// static RE_FLAG: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(
// 	r"#[0-9a-zA-Z_\-]+"
//     ).unwrap());

// static RE_FLAGS: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(
// 	r"^\s*(#[0-9a-zA-Z_\-]+(?:\s*#[0-9a-zA-Z_\-]+)*)\s*$"
//     ).unwrap());

// static RE_ATTRIBUTE: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(
// 	r"^\s*:(.+?)?:\s*(.+?)?\s*$"
//     ).unwrap());

fn embold(input: &str) -> String {
    let re = Regex::new(
	r"(?:\s|^)\*(\S+?)\*(?:\s|$)"
    ).unwrap();
    re.replace_all(input, "<b>$1</b>").to_string()
}

fn italicize(input: &str) -> String {
    let re = Regex::new(
	r"(?:\s|^)/(\S+?)/(?:\s|$)"
    ).unwrap();
    re.replace_all(input, "<i>$1</i>").to_string()
}

fn underline(input: &str) -> String {
    let re = Regex::new(
	r"(?:\s|^)_(\S+?)_(?:\s|$)"
    ).unwrap();
    re.replace_all(input, "<u>$1</u>").to_string()
}

fn enquote(input: &str) -> String {
    let re = Regex::new(r"(?m)^>[ a-zA-Z0-9].*$").unwrap();
    re.replace_all(
	input,
	r#"<span class="quote">$0</span>"#
    ).to_string()
}

fn enref(input: &str) -> String {
    let re = Regex::new(r"(?m)^>> *([0-9a-fA-F]{3,8})(\r)*$").unwrap();
    re.replace_all(
	input,
	r#"<a class="ref" href="/k/$1">$0</a>"#
    ).to_string()
}

pub fn format(input: &str) -> String {
    let formatted_input = enref(&enquote(&underline(&italicize(&embold(
	input
    )))));

    formatted_input
}
