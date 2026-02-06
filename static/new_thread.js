document.getElementById("newThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postFormContainer").style.display = "flex";
    document.getElementById("newThreadButton").style.display = "none";
};
