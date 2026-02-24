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
    let re = Regex::new(r"\*(\S+?)\*").unwrap();
    re.replace_all(input, "<b>$1</b>").to_string()
}

fn italicize(input: &str) -> String {
    let re = Regex::new(r"/(\S+?)/").unwrap();
    re.replace_all(input, "<i>$1</i>").to_string()
}

fn underline(input: &str) -> String {
    let re = Regex::new(r"_(\S+?)_").unwrap();
    re.replace_all(input, "<u>$1</u>").to_string()
}

fn enquote(input: &str) -> String {
    let re = Regex::new(r"^> +\S+ *$").unwrap();
    re.replace_all(
	input,
	"<span class=\"q-comment\">$0</span>"
    ).to_string()
}

fn enref(input: &str) -> String {
    let re = Regex::new(r"^>>\([0-9a-f]{3,8}\)$").unwrap();
    re.replace_all(
	input,
	"<span class=\"t-comment\" href=\"/k/$1\">$0</span>"
    ).to_string()
}

pub fn format(input: &str) -> String {
    enref(&enquote(&underline(&italicize(&embold(input)))))
}
