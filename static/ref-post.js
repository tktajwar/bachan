const cacheRef = new Map();

async function fetchSummary(id) {
    if (!cacheRef.has(id)) {
	const res = await fetch(`/api/summary/${id}`);
	if(!res.ok) return;
	cacheRef.set(id, await res.text());
    }
    return cacheRef.get(id);
}

async function previewPost(id, el) {
    const data = await fetchSummary(id);
    el.title = data;
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

document.addEventListener('pointerover', (e) => {
    clientX = e.clientX;
    clientY = e.clientY;
    const a = e.target.closest && e.target.closest('.ref');
    if (!a) return;
    const id = a.getAttribute('href').split('/').pop();
    previewPost(id, a);
    addHighlightFor(id);
});

document.addEventListener('pointerout', e => {
    removeHighlights();
});
