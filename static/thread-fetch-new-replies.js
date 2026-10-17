const initialInterval = 2;
const maxInterval = 600;
let unseenCount = 0;
let interval = initialInterval;
let originalTitle = document.title;

async function fetchUpdates() {
    const res = await fetch(`/${board}/${thread_id}/updates?after_id=${lastId}`);
    if(!res.ok) { console.debug(res); return };
    const data = await res.json();
    if (Array.isArray(data[0]) && data[0].length) {
	insertReplies(data[0]);
	unseenCount += data[0].length;
	updateTitle();
	interval = initialInterval;
	lastId = data[1];
    } else {
	interval = Math.min(interval * 2, maxInterval);
    }
}

async function poll() {
    fetchUpdates();
    setTimeout(poll, interval*1000);
}

function insertReplies(replies){
    const container = document.getElementById('thread-replies');
    if (!container) return;
    const frag = document.createDocumentFragment();
    replies.forEach(reply => {
	const div = document.createElement('div');
	div.className = 'reply';
	div.dataset.postId = String(reply.id);
	div.id = `${reply.id}`;
	div.innerHTML = `
<p class="reply-tagline">

  <span class="reply-utid">
    ${reply.user_name}${reply.user_code ? '#'+reply.user_code : ''}
    <span class="cc">${flagEmoji(reply.user_country)}</span>
  </span>

  <span class="reply-id">
    <a href="/${board}/${thread_id}?enref=${reply.id}"
       onclick="return enref('${reply.id}')"
       title="(cite) পোস্টটি উদ্ধৃত করুন">
      [ প্রকাশনা ${reply.id} ]
    </a>
  </span>

  <span class="reply-report">
    <a href="/${board}/moderation/${reply.id}"
      title="(report) রিপোর্ট করুন">
      <span class="reicon--flag"></span>
    </a>
  </span>
</p>


<div class="reply-body">
  <div class="reply-comment">${formatted(reply.comment)}</div>
</div>

<p class="reply-footer">
  <span class="reply-time">
    ${toLocalTime(reply.birth_time.trim())}
  </span>

  <span id="bl_${reply.id}" class="backlinks">
  </span>
</p>
`;

	div.querySelectorAll('.ref').forEach(el => {
	    localizeRef(el);
	});
	frag.appendChild(div);
	addPostsBackrefs(div);
	writePostsBackrefs();
    });
    container.appendChild(frag);
}

setTimeout(poll, interval*1000)

function inLocalTime(s) {
    const options = { 
	year: 'numeric', 
	month: 'long', 
	day: 'numeric', 
	hour: '2-digit', 
	minute: '2-digit', 
	hour12: false,
    };

    const utcDate = new Date(s.trim());
    return utcDate.toLocaleString('bn-BD',options);
}

function updateTitle() {
    if (unseenCount > 0) {
	document.title = `(${unseenCount}) ${originalTitle}`;
    }
}

function clearUnseen() {
    unseenCount = 0;
    document.title = `${originalTitle}`;
}

window.addEventListener('focus', clearUnseen);

let countDownTimer = document.getElementById('countDown');
if (countDownTimer) countDownTimer.remove();
