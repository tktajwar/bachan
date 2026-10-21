document.getElementById("postFormContainer").style.display = "none";

document.getElementById("newThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postFormContainer").style.display = "block";
    document.getElementById("newThreadButton").style.display = "none";
};

function load_previous_replies(btn) {
    const replies = btn.parentElement.querySelectorAll('.reply');
    replies.forEach(reply => {
	if (reply.style.display == 'none') {
	    reply.style.display = 'table';
	}
    });
    btn.style.display = 'none';
}

(function() {
    const MAX_VISIBLE = 5;
    const threads = document.querySelectorAll('.thread-and-reply');

    threads.forEach(thread => {
	const replies = thread.querySelectorAll('.reply');
	const hideCount = replies.length - MAX_VISIBLE;

	if (hideCount > 0) {
	    const reply_container = thread.querySelector('.board-thread-replies');
	    let load_prev_btn = document.createElement('a');
	    load_prev_btn.href = '#';
	    load_prev_btn.onclick = function(){load_previous_replies(this);return false};
	    load_prev_btn.class = 'load-more-replies';
	    load_prev_btn.textContent = `সকল মন্তব্য দেখুন (${hideCount} omitted)`;
	    reply_container.prepend(load_prev_btn);
	}

	for (let i = 0; i < hideCount; i++) {
	    const reply = replies[i];
	    reply.style.display = 'none';
	};
    });
})();
