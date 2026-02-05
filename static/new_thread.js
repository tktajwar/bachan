document.getElementById("newThreadButton").onclick = function(event) {
    event.preventDefault();
    document.getElementById("postFormContainer").style.display = "flex";
    document.getElementById("newThreadButton").style.display = "none";
};

document.getElementById("image-upload").onchange = function() {
    if(this.files[0].size > 2097152) {
       alert("File is too big!");
       this.value = "";
    }
};
