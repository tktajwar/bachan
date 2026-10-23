const username = document.getElementById("postName");

username.value = localStorage.getItem("name") || "";

username.addEventListener("change", () => {
    localStorage.setItem("name", username.value);
});

const replyForm = document.querySelector('#replyForm');

if (replyForm) {
    const form = replyForm;
    const submitBtn = form.querySelector('button[type="submit"]');
    const commentBox = form.querySelector('#postComment');

    form.addEventListener('submit', async function(e) {
	e.preventDefault();
	submitBtn.disabled = true;

	try {
	    const formData = new FormData(form);
	    const params = new URLSearchParams(formData);
	    const response = await fetch(replyForm.action, {
		method: form.method || 'POST',
		headers: {
		    'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: params
	    });

	    if (!response.ok) {
		throw new Error(`Server error: ${response.status}`);
	    }

	    commentBox.value = '';
	    fetchUpdates();

	} catch (error) {
	    alert('Error: ' + error.message);
	}

	submitBtn.disabled = false;

    });
}
