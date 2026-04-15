document.getElementById("newThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postFormContainer").style.display = "block";
    document.getElementById("newThreadButton").style.display = "none";
};
