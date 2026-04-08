function add_ref(id) {
    var inputBox = document.getElementById('reply-box');
    inputBox.value += "\n>>" + id + '\n';
    inputBox.focus();
}
