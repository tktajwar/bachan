const username = document.getElementById("postName");

username.value = localStorage.getItem("name") || "";

username.addEventListener("change", () => {
    localStorage.setItem("name", username.value);
});

function openPopupThreadMenu(event) {
    event.preventDefault();
    document.getElementById("postPopup").style.display = "block";
}

function closePopupThreadMenu(event) {
    event.preventDefault();
    document.getElementById("postPopup").style.display = "none";
}

document.getElementById("popupThreadButton")
    .addEventListener('mousedown', openPopupThreadMenu);

document.getElementById("closePopupThreadButton")
    .addEventListener('mousedown', closePopupThreadMenu);
