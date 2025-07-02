const usernameInput = document.querySelector('input[name="username"]');
const contentInput = document.querySelector('input[name="content"]');
const submitBtn = document.getElementsByClassName("send-btn");

submitBtn[0].addEventListener("click", () => {
    document.cookie = `username=${usernameInput.value}`;
    document.cookie = `content=${contentInput.value}`;
});
