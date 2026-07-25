function formatLocalTime() {
    const options = { 
	year: 'numeric', 
	month: 'long', 
	day: 'numeric', 
	hour: '2-digit', 
	minute: '2-digit', 
	hour12: false,
    };

    const times = document.querySelectorAll('.thread-time,.reply-time');

    times.forEach(element => {
	const utcDate = new Date(element.textContent.trim());
	const localDate = utcDate.toLocaleString('bn-BD',options);
	element.textContent = localDate;
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
