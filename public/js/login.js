window.onload = () => {

    const shortcuts = localStorage.getItem("shortcuts");
    const shortcutsModal = document.getElementById("shortcuts-toggle-modal");
    const tokenString = localStorage.getItem("auth");

    htmx.ajax("GET", `/handle/navbar/login/${tokenString}`, { target: "#navbar-profile", swap: "innerHTML" });

    if (shortcuts && shortcuts == "false") {
        hideShortcuts();
    } else {
        showShortcuts();
    };

    shortcutsModal.addEventListener("mouseover", () => {
        document.getElementById("shortcuts-toggle").style.display = "flex";
    });

    shortcutsModal.addEventListener("mouseout", () => {
        document.getElementById("shortcuts-toggle").style.display = "none";
    });
};

const showShortcuts = () => {
    document.getElementById("shortcuts-modal").style.display = "flex";
    document.getElementById("shortcuts-toggle").src = "/public/hide.webp";
};

const hideShortcuts = () => {
    document.getElementById("shortcuts-modal").style.display = "none";
    document.getElementById("shortcuts-toggle").src = "/public/show.webp";
};

const toggleShortcuts = () => {
    const showShortcuts = localStorage.getItem("shortcuts");

    if (showShortcuts && showShortcuts == "false") {
        localStorage.setItem("shortcuts", "true");
        window.location.reload();
    } else {
        localStorage.setItem("shortcuts", "false");
        window.location.reload();
    };
};

document.getElementById("login-form").addEventListener("submit", async (e) => {

    e.preventDefault();

    const email = document.getElementById("login-email").value;
    const password = document.getElementById("login-password").value;
    const response = document.getElementById("login-response");
    const timer = document.getElementById("login-timer");

    const authReq = await fetch("/api/user/auth", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            "email": email,
            "password": password,
            "username": "doesntmatter",
            "favorites": null,
            "old_pw": "",
            "deleted": false
        })
    });
    const authenticated = await authReq.json();

    if (typeof authenticated === "string" && authReq.status == 200) {

        localStorage.setItem("auth", authenticated);
        htmx.ajax("GET", `/handle/login/${authenticated}`, { target: "#login-response", swap: "innerHTML" });

        setTimeout(() => {
            timer.innerHTML = "<p>$  redirecting in 3 secs.</p>";
        }, 1000);

        setTimeout(() => {
            timer.innerHTML = "<p>$  redirecting in 2 secs..</p>";
        }, 2000);

        setTimeout(() => {
            timer.innerHTML = "<p>$  redirecting in 1 secs...</p>";
        }, 3000);

        setTimeout(() => {
            window.location.href = "/";
        }, 3500);

    } else {

        htmx.ajax("GET", `/handle/login/null`, { target: "#login-response", swap: "innerHTML" });

        setTimeout(() => {
            response.innerHTML = "";
        }, 1500);
    };
});
