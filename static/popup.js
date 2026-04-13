document.getElementById("popupThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postPopup").style.display = "block";
};

document.getElementById("closePopupThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postPopup").style.display = "none";
};
