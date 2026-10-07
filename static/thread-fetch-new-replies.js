const initialInterval = 2;
const maxInterval = 600;
let unseenCount = 0;
let interval = initialInterval;
let originalTitle = document.title;

async function fetchUpdates() {
    const res = await fetch(`/${board}/${thread_id}/updates?after_id=${lastId}`);
    if(!res.ok) return;
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
<div class="reply" id="${reply.id}">

<p class="reply-tagline">
  <span class="reply-utid">
    ${reply.user_name}#${reply.user_code} <span class="cc">${reply.user_country}</span>
  </span>
  <span class="reply-time">
    ${reply.birth_time}
  </span>
  <span class="reply-id">
    প্রকাশনা ${reply.id}
  </span>
</p>

<div class="reply-body">
  <div class="reply-comment">${reply.comment}</div>
</div>

<ul class="thread-buttons">
  <li class="thread-reply">
    <a href="/${board}/${thread_id}?enref=${reply.id}"
       onclick="return enref('${reply.id}')"
       title="(cite) পোস্টটি উদ্ধৃত করুন">
      <i class="fa-solid fa-angles-right"></i>${reply.id}
    </a>
  </li>
  <li class="thread-report">
    <!-- TODO -->
    <a href="#" title="(report) রিপোর্ট করুন">
      <i class="fa-regular fa-flag"></i>
    </a>
  </li>
  <li class="thread-mod">
    <a href="/mod/${reply.id}" title="(moderate) পরিচালনা করুন">
      <i class="fa-solid fa-ellipsis"></i>
    </a>
  </li>
</ul>


</div>
`;

	div.querySelectorAll('.ref').forEach(el => {
	    localizeRef(el);
	});
	frag.appendChild(div);
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
