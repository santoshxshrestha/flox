function getCookie(name) {
    const cookies = document.cookie.split("; ");
    for (const cookie of cookies) {
        const [key, value] = cookie.split("=");
        if (key === name) return decodeURIComponent(value);
    }
    return null;
}

document.addEventListener("DOMContentLoaded", () => {
    const usernameCookie = getCookie("username");
    const contentCookie = getCookie("content");

    const deleteButtons = document.querySelectorAll(".delete-btn");

    deleteButtons.forEach((btn) => {
        const msgUsername = btn.getAttribute("data-username");
        const msgContent = btn.getAttribute("data-content");

        if (msgUsername == usernameCookie && msgContent == contentCookie) {
            btn.disabled = false;
            btn.style.display = " ";
        } else {
            btn.style.display = "none";
            btn.disabled = true;
        }
    });
});
