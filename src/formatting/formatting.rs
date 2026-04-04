use censor::*;
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
    let re = Regex::new(r"(?m)^(>|&gt;)(>|&gt;) *([0-9a-fA-F]{3,8})(\r)*$").unwrap();
    re.replace_all(
	input,
	r#"<a class="ref" href="/k/$1">$0</a>"#
    ).to_string()
}

fn embed(input: &str) -> String {
    let re = Regex::new(
	r"\[\[((?:http|https|ftp)://.+?(?i:(?:jpeg|jpg|png|webp|gif)))\]\]"
    ).unwrap();

    if re.find_iter(input).count() > 5 {
	return input.to_string()
    }

    re.replace_all(
	input,
	r#"<img class="img-comment" src="$1" alt="$1" />"#
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
	    r#"<a href="{}" onClick="return confirm('Enter {}?')">{}</a>"#,
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

fn redact_profane_words(input: &str) -> String {
    let censor = Censor::Standard + Censor::Sex
	+ "porn"
	+ "xhamster"
	+ "xvideo"
	+ "xnxx"
	+ "jav"
	+ "xhwide"
	+ "jizz"
	+ "redtube"
	+ "freeones"
	+ "motherless"
	+ "fatherless"
	+ "brazzer"
	+ "bang"
	+ "adult"
	+ "চোদ"
	+ "choda"
	+ "ভুষ্কি"
	+ "bhuski"
	;
    censor.censor(input)
}

pub fn format(input: &str) -> String {
    let input = redact_profane_words(&sanitize(input));

    let formatted_input = enref(&enquote(&underline(&italicize(&embold(
	&input
    )))));

    let formatted_input = enlink(&embed(
	&formatted_input
    ));

    formatted_input
}
