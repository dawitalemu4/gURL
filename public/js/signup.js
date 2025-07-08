window.onload = () => {

    const shortcuts = localStorage.getItem("shortcuts");
    const shortcutsModal = document.getElementById("shortcuts-toggle-modal");
    const tokenString = localStorage.getItem("auth");

    htmx.ajax("GET", `/handle/navbar/signup/${tokenString}`, { target: "#navbar-profile", swap: "innerHTML" });

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

document.getElementById("signup-form").addEventListener("submit", async (e) => {

    e.preventDefault();

    const username = document.getElementById("signup-username").value;
    const email = document.getElementById("signup-email").value;
    const password = document.getElementById("signup-password").value;
    const response = document.getElementById("signup-response");
    const timer = document.getElementById("signup-timer");

    const createRequest = await fetch("/api/user/new", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            "username": username,
            "email": email,
            "password": password,
            "date": `${Date.now()}`,
            "favorites": null,
            "old_pw": "",
            "deleted": false
        })
    });
    const created = await createRequest.json();

    if (typeof created === "string" && createRequest.status == 200) {

        localStorage.setItem("auth", created);
        htmx.ajax("GET", `/handle/signup/${created}`, { target: "#signup-response", swap: "innerHTML" });

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

        htmx.ajax("GET", `/handle/signup/null`, { target: "#signup-response", swap: "innerHTML" });

        setTimeout(() => {
            response.innerHTML = "";
        }, 1500);
    };
});
