const username = document.getElementById("postName");

username.value = localStorage.getItem("name") || "";

username.addEventListener("change", () => {
    localStorage.setItem("name", username.value);
});

function togglePreview(
    commentBox,
    previewBox,
    togglePreviewBtn,
    is_preview_mode,
) {
    is_preview_mode = !is_preview_mode;
    if (is_preview_mode) {
	if (commentBox.value.length == 0) {
	    return false;
	}
	previewBox.innerHTML = formatted(commentBox.value);
	previewBox.classList.remove('hidden');
	commentBox.classList.add('hidden');
	togglePreviewBtn.textContent = "এডিট করুন";
    } else {
	commentBox.classList.remove('hidden');
	previewBox.classList.add('hidden');
	togglePreviewBtn.textContent = "প্রিভিউ দেখুন";
    }
    return is_preview_mode;
}

const replyForm = document.querySelector('#replyForm');

if (replyForm) {
    const form = replyForm;
    const submitBtn = form.querySelector('button[type="submit"]');
    const togglePreviewBtn = form.querySelector('#toggle-preview');
    const commentBox = form.querySelector('#postComment');
    const previewBox = form.querySelector('#postPreview');
    let is_preview_mode = false;

    togglePreviewBtn.classList.remove('hidden');
    togglePreviewBtn.textContent = "প্রিভিউ দেখুন";

    togglePreviewBtn.addEventListener("click", () => {
	is_preview_mode = togglePreview(
	    commentBox,
	    previewBox,
	    togglePreviewBtn,
	    is_preview_mode
	);
    });

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

const postForm = document.querySelector('#postForm');

if (postForm) {
    const form = postForm;
    const submitBtn = form.querySelector('button[type="submit"]');
    const togglePreviewBtn = form.querySelector('#toggle-preview');
    const commentBox = form.querySelector('#postComment');
    const previewBox = form.querySelector('#postPreview');
    let is_preview_mode = false;

    togglePreviewBtn.classList.remove('hidden');
    togglePreviewBtn.textContent = "প্রিভিউ দেখুন";

    togglePreviewBtn.addEventListener("click", () => {
	is_preview_mode = togglePreview(
	    commentBox,
	    previewBox,
	    togglePreviewBtn,
	    is_preview_mode
	);
    });

    form.addEventListener('submit', async function(e) {
	setTimeout(() => {
	    submitBtn.disabled = true;
	}, 0);
    });
}

