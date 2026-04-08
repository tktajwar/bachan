function add_ref(id) {
    var inputBox = document.getElementById('reply-box');
    inputBox.value += "\n>>" + id + '\n';
    inputBox.focus();
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
