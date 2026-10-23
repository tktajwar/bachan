const username = document.getElementById("postName");

username.value = localStorage.getItem("name") || "";

username.addEventListener("change", () => {
    localStorage.setItem("name", username.value);
});

const form = document.querySelector('form');
const submitBtn = form.querySelector('button[type="submit"]');

console.log(submitBtn);

form.addEventListener('submit', function(e) {
  submitBtn.disabled = true;
});
