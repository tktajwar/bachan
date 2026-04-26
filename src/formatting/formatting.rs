use regex::Regex;
use sanitize_html::{
    rules::predefined::DEFAULT,
    sanitize_str,
};
use std::collections::HashSet;

fn embold(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")'''(\S+?(?: +\S+?)*?)'''(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<b>$2</b>$3").to_string()
}

fn italicize(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")''(\S+?(?: +\S+?)*?)''(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<i>$2</i>$3").to_string()
}

fn underline(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")__(\S+?(?: +\S+?)*?)__(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<u>$2</u>$3").to_string()
}

fn strikethrough(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")~~(\S+?(?: +\S+?)*?)~~(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<s>$2</s>$3").to_string()
}

fn spoiler(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")\[spoiler\](\S+?(?: +\S+?)*?)\[/spoiler\](\s|,|\.|"|$)"#
    ).unwrap();
    let input = &re.replace_all(input, "$1<span class=\"spoiler\">$2</span>$3");

    let re = Regex::new(
	r#"(^|\s|,|\.|")\*\*(\S+?(?: +\S+?)*?)\*\*(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<span class=\"spoiler\">$2</span>$3").to_string()
}

fn redtext(input: &str) -> String {
    let re = Regex::new(
	r#"(^|\s|,|\.|")==(\S+?(?: +\S+?)*?)==(\s|,|\.|"|$)"#
    ).unwrap();
    re.replace_all(input, "$1<span class=\"redText\">$2</span>$3").to_string()
}

fn enquote(input: &str) -> String {
    let re = Regex::new(r"(?m)^(>|&gt;).*$").unwrap();
    re.replace_all(
	input,
	r#"<span class="quote">$0</span>"#
    ).to_string()
}

fn enref(input: &str) -> String {
    let re = Regex::new(
	r"(>|&gt;)(>|&gt;) *([0-9a-fA-F]{1,8})"
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

fn ref_board(input: &str) -> String {
    let re = Regex::new(
	r#"((?:>|&gt;){3}/([a-z]{1,12})/)(\s|,|\.|"|$)"#
    ).unwrap();

    re.replace_all(
	input,
	r#"<a class="boardRef" href="/$2/">$1</a>$3"#
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

pub fn sanitize(input: &str) -> String {
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
	r"\^\{(.+?)\}"
    ).unwrap();
    re.replace_all(
	input,
	r#"<sup>$1</sup>"#
    ).to_string()
}

fn sub(input: &str) -> String {
    let re = Regex::new(
	r"_\{(.+?)\}"
    ).unwrap();
    re.replace_all(
	input,
	r#"<sub>$1</sub>"#
    ).to_string()
}

pub fn format(input: &str) -> String {
    let input = sanitize(input);

    let formatted_input = ref_board(&enlink(&embed(
	&input
    )));

    let formatted_input = enref(&enquote(
	&redtext(&spoiler(&strikethrough(&underline(&italicize(&embold(
	    &formatted_input
	))))))
    ));

    let formatted_input = sub(&sup(
	&formatted_input
    ));

    formatted_input
}

pub fn references(input: &str) -> HashSet<i32> {
    let re = Regex::new (
	r"(>|&gt;)(>|&gt;) *([0-9a-fA-F]{1,8})"
    ).unwrap();

    let mut references = HashSet::new();
    for caps in re.captures_iter(input) {
	if let Some(id) = caps.get(3) {
	    if let Ok(n) = i32::from_str_radix(
		id.as_str(),
		16,
	    ) {
		references.insert(n);
	    }
	}
    }

    references
}
