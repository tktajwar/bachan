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

formatLocalTime();
