const re_bold = /(?<=^|\s)'''(\S+?(?:\s+\S+)*?)'''(?=\s|$)/g;
const re_italic = /(?<=^|\s)''(\S+?(?:\s+\S+)*?)''(?=\s|$)/g;
const re_underline = /(?<=^|\s)__(\S+?(?:\s+\S+)*?)__(?=\s|$)/g;
const re_quote = /(?<=^)(>[^>].+)(?=$)/g;
const re_ref = /(?<=^|\s)>>\s*([0-9a-fA-F]+)(?=\s|$)/g;
const re_global_ref = /(?<=^|\s)>>>\s*\/(\w+?\/(?:[0-9a-fA-F]+)?)\/?(?=\s|$)/g;
const re_alt_ref = /(?<=^|\s)\[(?:প্রকাশনা|post)\s+?([0-9a-fA-F]+)\](?=\s|$)/g;
const re_spoiler = /\[spoiler\](.+?)\[\/spoiler\]/g;
const re_strikethrough = /(?<=^|\s)~~(\S+?(?:\s+\S+)*?)~~(?=\s|$)/g;
const re_redText = /(?<=^|\s)==(\S+?(?:\s+\S+)*?)==(?=\s|$)/g;
const re_embed = /\[\[((?:http|https):\/\/.+?(?:jpeg|jpg|png|webp|gif))\]\s*?(?:\[(.*?)\])?\]/g;
const re_hyper_link = /\[\[((?:http|https|ftp):\/\/.+?)\]\s*?\[(.+?)\]\]/g;
const re_link = /\[\[((?:http|https|ftp):\/\/.+?)\]\s*?(?:\[\])?\]/g;
const re_unformatted_link = /(?<=^|\s)(?:http|https|ftp):\/\/\S+(?=\s|$)/g;

function formatted(text) {
    return text.replaceAll(
	re_bold, '<b>$1</b>'
    ).replaceAll(
	re_italic, '<i>$1</i>'
    ).replaceAll(
	re_underline, '<u>$1</u>'
    ).replaceAll(
	re_quote, '<span class="quote">$1</span>'
    ).replaceAll(
	re_ref, '<a class="ref" href="$1">$&</a>'
    ).replaceAll(
	re_global_ref, '<a class="ref" href="/$1">$&</a>'
    ).replaceAll(
	re_alt_ref, '<a class="ref" href="$1">$&</a>'
    ).replaceAll(
	re_spoiler, '<span class="spoiler">$1</span>'
    ).replaceAll(
	re_strikethrough, '<s>$1</s>'
    ).replaceAll(
	re_redText, '<span class="redText">$1</span>'
    ).replaceAll(
	re_embed, `<figure class="fig-comment">
<img src="$1" alt="$1" class="img-comment" loading="lazy" />
<figcaption>$2</figcaption>
</figure>`
    ).replaceAll(
	re_hyper_link, '<a href="$1" target="_blank" ' +
	    'title="$1"' +
	    'onclick="return confirm(\'আপনি  [$1] লিংকটিতে প্রবেশ করছেন\')" ' +
	    '>' +
	    '$2' +
	    '</a>'
    ).replaceAll(
	re_link, '<a href="$1" target="_blank">$1</a>'
    ).replaceAll(
	re_unformatted_link, '<a href="$&" target="_blank">$&</a>'
    )
}

(function() {
    const comments = document.querySelectorAll(
	'.thread-container .thread .thread-body .thread-comment,' +
	'.thread-container #thread-replies .reply .reply-body .reply-comment'
    );

    comments.forEach(comment => {
	comment.innerHTML = formatted(comment.textContent);
    });
})();

(function() {
    const trs = document.querySelectorAll('#formatting-guide table tbody tr');

    trs.forEach(tr => {
	const [sourceTd, targetTd] = tr.querySelectorAll('td');

	if (sourceTd && targetTd) {
	    targetTd.innerHTML = formatted(sourceTd.textContent);
	}
    });
})();
