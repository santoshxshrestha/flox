document.addEventListener("DOMContentLoaded", () => {
  const deleteButtons = document.querySelectorAll(".delete-btn");

  deleteButtons.forEach((btn) => {
    const deletePerm = btn.getAttribute("delete-perm");

    if (deletePerm === "true") {
      btn.disabled = false;
      btn.style.display = "";
    } else {
      btn.style.display = "none";
      btn.disabled = true;
    }
  });
});
