const cacheRef = new Map();
const backRefMap = new Map();
const backRefChangedSet = new Set();

const re_bold = /(?<=^|\s)&#x27;&#x27;&#x27;(\S+?(?:\s+\S+)*?)&#x27;&#x27;&#x27;(?=\s|$)/g;
const re_italic = /(?<=^|\s)&#x27;&#x27;(\S+?(?:\s+\S+)*?)&#x27;&#x27;(?=\s|$)/g;
const re_underline = /(?<=^|\s)__(\S+?(?:\s+\S+)*?)__(?=\s|$)/g;
const re_quote = /(?<=^|\n)((?:>|&gt;)(?!>|&gt;)..*?)(?=\r|\n|$)/g;
const re_ref = /(?<=^|\s)(?:>>|&gt;&gt;)\s*([0-9a-fA-F]+)(?=\s|$)/g;
const re_global_ref = /(?<=^|\s)(?:>>>|&gt;&gt;&gt;)\s*\/(\w+?\/(?:[0-9a-fA-F]+)?)\/?(?=\s|$)/g;
const re_alt_ref = /(?<=^|\s)\[\s{0,3}(?:প্রকাশনা|post)\s+?([0-9a-fA-F]+)\s{0,3}\](?=\s|$)/g;
const re_spoiler = /\[spoiler\](.+?)\[\/spoiler\]/g;
const re_strikethrough = /(?<=^|\s)~~(\S+?(?:\s+\S+)*?)~~(?=\s|$)/g;
const re_redText = /(?<=^|\s)==(\S+?(?:\s+\S+)*?)==(?=\s|$)/g;

const re_embed = /(?<=^|\s|\[|\()(?:(?<=^|.)\[)?((?:http|https):\/\/\S+?\.\S+?(?:jpeg|jpg|png|webp|gif))(?:\](?=.|$))?(?=\s|\]|\)|$)/g;
const re_link = /(?<=^|\s|\[|\()((?:http|https):\/\/\S+?)(?=\s|\]|\)|$)/g;

const san_map = {
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#x27;',
};
const reg = /[<>"']/ig;

function sanitized(string) {
  return string.replace(reg, (match)=>(san_map[match]));
}

function formatted(text) {
    const sanitized_text = sanitized(text);
    return sanitized_text.replaceAll(
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
	re_global_ref, '<a class="ref global-ref" href="/$1">$&</a>'
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
</figure>`
    ).replaceAll(
	re_link, '<a href="$1" target="_blank">$1</a>'
    )
}

function toLocalTime(date) {
    const today = new Date();
    const utcDate = new Date(date.trim());

    const options = {
	month: 'long',
	day: 'numeric',
	hour: '2-digit',
	minute: '2-digit',
	hour12: false,
    };

    if (utcDate.getFullYear() != today.getFullYear()) {
	options.year = 'numeric';
    }

    return utcDate.toLocaleString('bn-BD',options);
}

function formatLocalTime() {
    const times = document.querySelectorAll('.thread-time,.reply-time');

    times.forEach(element => {
	element.textContent = toLocalTime(element.textContent);
    });
}

function flagEmoji(cc) {
    if (cc == 'ZZ') return cc;
    return `
<img
  src="https://flagcdn.com/16x12/${cc.toLowerCase()}.png"
  srcset="https://flagcdn.com/32x24/${cc.toLowerCase()}.png 2x,
    https://flagcdn.com/48x36/${cc.toLowerCase()}.png 3x"
  alt="${cc}">
`;
}

function add_ref(id) {
    var inputBox = document.getElementById('postComment');
    inputBox.value += "\n>>" + id + '\n';
    inputBox.focus();
}

function enref(id) {
    if (document.getElementById('replyForm')) {
	add_ref(id);
	return false;
    } else {
	return true;
    }
}

function getQueryParam(param) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(param);
}

window.onload = function() {
    const id = getQueryParam('enref');
    if (id) {
        add_ref(id);
    }
};

function postIsVisible(id) {
    const el = document.getElementById(id);
    if (!el) return false;
    const rect = el.getBoundingClientRect();
    return (
        rect.top >= 0 &&
        rect.left >= 0 &&
        rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
        rect.right <= (window.innerWidth || document.documentElement.clientWidth) &&
	el.style.display != 'none'
    );
}

function previewFor(id, referer) {
    const el = document.getElementById(id);
    if (!el) return;
    preview.innerHTML = el.innerHTML;
    preview.classList.add('active');
}

function positionPreviewTo(el) {
    const rect = el.getBoundingClientRect();
    const offset = 15;
    let x = rect.right + window.scrollX + offset;
    let y = rect.top + window.scrollY;

    if (
	x + preview.offsetWidth > window.innerWidth + window.scrollX &&
	    x > (window.innerWidth + window.scrollX) / 2
    ) {
	x = rect.left + window.scrollX - preview.offsetWidth - offset;
    }

    if (
	y + preview.offsetHeight > window.innerHeight + window.scrollY &&
	    y > (window.innerHeight + window.scrollY) / 2
    ) {
	y = rect.top + window.scrollY - preview.offsetHeight;
    }

    preview.style.left = `${x}px`;
    preview.style.top = `${y}px`;
}

function addHighlightFor(id) {
    const el = document.getElementById(id);
    if (el) el.classList.add('highlighted');
}

function removeHighlights() {
    document.querySelectorAll('.highlighted').forEach(el => {
	el.classList.remove('highlighted');
    });
}

function localizeRef(el) {
    let id = el.getAttribute('href').split('/').pop()
    if (id.startsWith('#')) id = id.slice(1);
    id = id.padStart(3, '0').toLowerCase();
    const referenced = document.getElementById(id);
    if (referenced) {
	el.setAttribute('href', `#${id}`);
	if (referenced.classList.contains('thread'))
	    el.insertAdjacentHTML('afterend', '<span>(OP)</span>');
    }
}

function references(post) {
    return Array.from(post.querySelectorAll (
	'.thread-body .ref:not(.global-ref),' +
	'.reply-body .ref:not(.global-ref)'
    )).map(r => r.hash.slice(1))
}

function addPostsBackrefs(post) {
    references(post).forEach( h => {
	const referenced_post = document.getElementById(h);
	if (referenced_post) {
	    if (!backRefMap.has(h)) {
		backRefMap.set(h, new Set());
	    }
	    backRefMap.get(h).add(`\
<a class="ref backref" href="#${post.id}">&gt;&gt;${post.id}</a>
`);
	    backRefChangedSet.add(h);
	}
    });
}

function addBackrefsForAllPost() {
    const posts = document.querySelectorAll(
	'.thread,' +
	'.reply'
    );

    posts.forEach(post => {
	addPostsBackrefs(post);
    });
}

function writePostsBackrefs() {
    backRefChangedSet.forEach(id => {
	const post = document.getElementById(id);

	const backlinks = post.querySelector('.backlinks');
	backlinks.innerHTML = Array.from(backRefMap.get(id)).join(' ');
    });

    backRefChangedSet.clear();
}

(function() {
    const comments = document.querySelectorAll(
	'.thread .thread-body .thread-comment,' +
	'.reply .reply-body .reply-comment'
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

(function() {document.querySelectorAll('.cc').forEach(el => {
  el.innerHTML = flagEmoji(el.textContent.trim());
})})();

formatLocalTime();

const preview = document.createElement('div');

(function() {
    preview.id = "post-preview";
    preview.className = "tooltip";
    const container = document.querySelector("main") || document.body;
    container.appendChild(preview);
})();




document.addEventListener('pointerover', (e) => {
    const a = e.target.closest && e.target.closest('.ref');
    if (!a) return;
    let id = a.getAttribute('href').split('/').pop();
    if (id.startsWith('#')) id = id.slice(1);
    id = id.padStart(3, '0').toLowerCase();
    if (postIsVisible(id)) {
	addHighlightFor(id);
    } else {
	previewFor(id);
	positionPreviewTo(a);
    }
});

document.addEventListener('pointerout', e => {
    removeHighlights();
    preview.classList.remove('active');
});

document.querySelectorAll('.ref').forEach(el => {
    localizeRef(el);
});

(function() {
    addBackrefsForAllPost();
    writePostsBackrefs();
})();
