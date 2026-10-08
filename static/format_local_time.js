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


function formatMTime() {
    function timeAgo(ts) {
	const seconds = Math.floor((Date.now() - ts) / 1000);

	if (seconds < 60) return `${seconds}s`;
	const minutes = Math.floor(seconds / 60);
	if (minutes < 60) return `${minutes}m`;

	const hours = Math.floor(minutes / 60);
	if (hours < 24) return `${hours}h`;

	const days = Math.floor(hours / 24);
	return `${days}d`;
    }


    const times = document.querySelectorAll('.thread-mtime');

    times.forEach(element => {
	const utcDate = new Date(element.title.trim());
	element.textContent = `(${timeAgo(utcDate)} ago)`;
    });
}

formatLocalTime();
formatMTime();
setInterval(formatMTime, 1000);
