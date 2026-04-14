use regex::Regex;
use sanitize_html::{
    rules::predefined::DEFAULT,
    sanitize_str,
};

fn embold(input: &str) -> String {
    let re = Regex::new(
	r"(\s|^)\*(\S+?(?: +\S+?)*)\*(\s|$)"
    ).unwrap();
    re.replace_all(input, "$1<b>$2</b>$3").to_string()
}

fn italicize(input: &str) -> String {
    let re = Regex::new(
	r"(\s|^)/(\S+?(?: +\S+?)*)/(\s|$)"
    ).unwrap();
    re.replace_all(input, "$1<i>$2</i>$3").to_string()
}

fn underline(input: &str) -> String {
    let re = Regex::new(
	r"(\s|^)_(\S+?(?: +\S+?)*)_(\s|$)"
    ).unwrap();
    re.replace_all(input, "$1<u>$2</u>$3").to_string()
}

fn enquote(input: &str) -> String {
    let re = Regex::new(r"(?m)^(>|&gt;)[ a-zA-Z0-9].*$").unwrap();
    re.replace_all(
	input,
	r#"<span class="quote">$0</span>"#
    ).to_string()
}

fn enref(input: &str) -> String {
    let re = Regex::new(
	r"(>|&gt;)(>|&gt;) *([0-9a-fA-F]{3,8})"
    ).unwrap();
    let input = &re.replace_all(
	input,
	r#"<a class="ref" href="/k/$3">$0</a>"#
    );

    let re = Regex::new(
	r"\[\[((?:প|প্রকাশনা|[Pp]ost|)[.: \-] *([0-9a-fA-F]{1,8}))\]\]"
    ).unwrap();
    re.replace_all(
	input,
	r#"<a class="ref" href="/k/$2">$1</a>"#
    ).to_string()
}

fn embed(input: &str) -> String {
    let re = Regex::new(concat!(
	r"\[",
	r"\[((?:http|https|ftp)://.+?(?i:(?:jpeg|jpg|png|webp|gif)))\]",
	r"(?:\[([^\n]+)?\])?",
	r"\]",
    )).unwrap();

    if re.find_iter(input).count() > 10 {
	return input.to_string()
    }

    re.replace_all(
	input,
	concat!(
	    r#"<figure class="fig-comment">"#,
	    r#"<img src="$1" alt="$1" class="img-comment">"#,
	    r#"<figcaption>$2</figcaption>"#,
	    r"</figure>",
	),
    ).to_string()
}

fn enlink(input: &str) -> String {
    let re = Regex::new(
	r"\[\[((?:http|https|ftp)://.+?)\](?:\[(.+?)?\])?\]"
    ).unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
	let href = caps[1].to_string();
	let text = caps.get(2).map_or(href.clone(),
				       |m| m.as_str().to_string()
	);

	format!(
	    r#"<a href="{}" target="_blank" onClick="return confirm('Enter {}?')">{}</a>"#,
	    href,
	    href,
	    text,
	)
    }).to_string()
}

fn sanitize(input: &str) -> String {
    let input_trimmed = input.trim();

    let input_escaped = input_trimmed.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;");

    sanitize_str(&DEFAULT, &input_escaped).unwrap()
}

fn sup(input: &str) -> String {
    let re = Regex::new(
	r"\^\{(.+)\}"
    ).unwrap();
    re.replace_all(
	input,
	r#"<sup>$1</sup>"#
    ).to_string()
}

fn sub(input: &str) -> String {
    let re = Regex::new(
	r"_\{(.+)\}"
    ).unwrap();
    re.replace_all(
	input,
	r#"<sub>$1</sub>"#
    ).to_string()
}

pub fn format(input: &str) -> String {
    let input = sanitize(input);

    let formatted_input = enref(&enquote(&underline(&italicize(&embold(
	&input
    )))));

    let formatted_input = enlink(&embed(
	&formatted_input
    ));

    let formatted_input = sub(&sup(
	&formatted_input
    ));

    formatted_input
}
