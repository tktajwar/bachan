const username = document.getElementById("postName");

username.value = localStorage.getItem("name") || "";

username.addEventListener("change", () => {
    localStorage.setItem("name", username.value);
});
