var lostFocus = function () {
  input = document.getElementById("prompt-input");
  setTimeout(function () {
    input.focus();
  }, 0);
};

document.getElementById("prompt-input").addEventListener("blur", lostFocus);
document.body.addEventListener("click", lostFocus, true);
document.addEventListener(
  "keydown",
  function (e) {
    if (e.keyCode == 13) {
      enterPressed();
    }
  },
  false
);

function enterPressed() {
  let cmd = document.getElementById("prompt-input").value;
  if (cmd && cmd.length > 0) {
    document.getElementById("prompt-input").value = "";
    if (!window.isContactFormOpen) {
      let cmdHistory = document.createElement("p");
      cmdHistory.innerHTML = "$ " + cmd;
      document.getElementById("terminal-history").appendChild(cmdHistory);
    }
    if (window.isContactFormOpen) {
      contactFormStep(cmd);
    } else if (
      [
        "help",
        "twitter",
        "linkedin",
        "github",
        "malt",
        "contact",
        "clear",
      ].includes(cmd)
    ) {
      runCmd(cmd);
    } else {
      let cmdNotFound = document.createElement("p");
      cmdNotFound.innerHTML = "antoinesh: command not found: " + cmd;
      document.getElementById("terminal-history").appendChild(cmdNotFound);
    }
  }
  document.getElementById("terminal-history").scrollTop =
    document.getElementById("terminal-history").scrollHeight;
}

function contactFormStep(cmd) {
  document.getElementById("terminal-history").lastChild.innerHTML += ` ${cmd}`;
  window.contactFormValue = {
    ...window.contactFormValue,
    [window.contactCurrentStep]: cmd,
  };
  nextStep();
}

function nextStep() {
  switch (window.contactCurrentStep) {
    case "name":
      let askEmail = document.createElement("p");
      askEmail.innerHTML = "Please type your email:";
      document.getElementById("terminal-history").appendChild(askEmail);
      window.contactCurrentStep = "email";
      break;
    case "email":
      let askMessage = document.createElement("p");
      askMessage.innerHTML = "Please type your message:";
      document.getElementById("terminal-history").appendChild(askMessage);
      window.contactCurrentStep = "message";
      break;
    case "message":
      contactMe(window.contactFormValue);
      window.isContactFormOpen = false;
      window.contactFormValue = {};
      window.contactCurrentStep = null;
      break;
  }
}

function runCmd(cmd) {
  switch (cmd) {
    case "twitter":
    case "linkedin":
    case "github":
    case "malt":
      let url = getUrl(cmd);
      window.open(url, "_blank");
      let link = document.createElement("a");
      link.href = url;
      link.target = "_blank";
      link.style = "color: #F3EFE0";
      link.innerHTML =
        "If the page did not open, please click here or visit " + url;
      document.getElementById("terminal-history").appendChild(link);
      break;
    case "help":
      let help = document.createElement("pre");
      help.innerHTML = `help        show this help
contact     send me a message
twitter     open my twitter in a new tab
linkedin    open my linkedin in a new tab
github      open my github in a new tab
malt        open my malt in a new tab
clear       clear the terminal output
`;
      document.getElementById("terminal-history").appendChild(help);
      break;
    case "contact":
      window.isContactFormOpen = true;
      window.contactCurrentStep = "name";
      window.contactFormValue = {};
      let askName = document.createElement("p");
      askName.innerHTML = "Please type your name:";
      document.getElementById("terminal-history").appendChild(askName);
      break;
    case "clear":
      document.getElementById("terminal-history").innerHTML = "";
      break;
    default:
      break;
  }
}

function getUrl(website) {
  let urls = {
    twitter: "https://twitter.com/Antoine64480",
    github: "https://github.com/alaborderie",
    linkedin: "https://www.linkedin.com/in/antoine-~-laborderie-866090130/",
    malt: "https://www.malt.fr/profile/antoinelaborderie",
  };

  return urls[website];
}

function contactMe(formValue) {
  fetch("/contact", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(formValue),
  })
    .then((res) => {
      console.log(res);
      if (res.ok) {
        let success = document.createElement("p");
        success.innerHTML = "Thanks for reaching out!";
        document.getElementById("terminal-history").appendChild(success);
      } else {
        showError();
      }
    })
    .catch((_er) => {
      showError();
    });
}

function showError() {
  let error = document.createElement("p");
  error.innerHTML = "Oops! Looks like the form wasn't valid, please try again!";
  document.getElementById("terminal-history").appendChild(error);
}
