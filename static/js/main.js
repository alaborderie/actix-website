function contactMe(ev) {
  const nameInput = document.getElementById("name");
  const emailInput = document.getElementById("email");
  const messageInput = document.getElementById("message");
  const name = nameInput.value;
  const email = emailInput.value;
  const message = messageInput.value;
  console.log("hello");

  fetch("/contact", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name, email, message }),
  })
    .then((res) => {
      console.log(res);
      if (res.ok) {
        // TODO: Show toaster success
        [nameInput, emailInput, messageInput].forEach((input) => {
          input.value = "";
        });
        const apiResponseField = document.getElementById("apiResponseField");
        apiResponseField.innerHTML =
          "Merci pour votre message! Je vous répondrai dès que possible.";
        apiResponseField.className = "api-success";
      } else {
        showError(apiResponseField);
      }
    })
    .catch((_er) => {
      showError(apiResponseField);
    });
}

function showError(apiResponseField) {
  apiResponseField.innerHTML =
    "Tous les champs sont obligatoires, veuillez les remplir et réessayer svp !";
  apiResponseField.className = "api-error";
}
