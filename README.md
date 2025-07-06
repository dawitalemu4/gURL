<h1 align="center">
    <img src="https://github.com/user-attachments/assets/6c08e081-0ff1-43db-8abc-77b37cb6707c" alt="gURL icon" style="width: 50px; height: 50px;">
    <p>gURL</p>
</h1>

gURL is a self-hosted dev tool that achieves what postman does but with my personal preferences in its functionality and design. Made with Rust (Axum), HTMX, and grpcurl.


## Differences from Postman

- Use your keyboard to do everything if you want to (literally everything)
- Less cluttered UI and makes the most important things always accessible
- No need to sign up or log in and give your data to a third party
- But keep your request history and build profiles to save favorite requests (and autofill the form with the saved request!)
- And more...


## Features

### Make Request

<video height="600px" controls autoplay preload muted loop>
    <source src="https://github.com/dawitalemu4/postwoman/assets/106638403/b09125dd-565e-479f-be58-194261c2667f">
</video>

<p style="visibility:hidden">https://github.com/dawitalemu4/postwoman/assets/106638403/b09125dd-565e-479f-be58-194261c2667f</p>


### History List

<video height="600px" controls autoplay preload muted loop>
    <source src="https://github.com/dawitalemu4/postwoman/assets/106638403/27d2a4b0-9890-43bd-9bfd-8f928445e011">
</video>

<p style="visibility:hidden">https://github.com/dawitalemu4/postwoman/assets/106638403/27d2a4b0-9890-43bd-9bfd-8f928445e011</p>


### Favorites List

<video height="600px" controls autoplay preload muted loop>
    <source src="https://github.com/dawitalemu4/postwoman/assets/106638403/9dd3a5b9-e647-48ec-932f-6b0bd84885bd">
</video>

<p style="visibility:hidden">https://github.com/dawitalemu4/postwoman/assets/106638403/9dd3a5b9-e647-48ec-932f-6b0bd84885bd</p>


### Fill Form from List

<video height="600px" controls autoplay preload muted loop>
    <source src="https://github.com/dawitalemu4/postwoman/assets/106638403/2b725e99-f3b3-4b6c-ba3c-33f6369b3e1d">
</video>

<p style="visibility:hidden">https://github.com/dawitalemu4/postwoman/assets/106638403/2b725e99-f3b3-4b6c-ba3c-33f6369b3e1d</p>


Go to [gURL.github.io/features](https://gURL.github.io/features) to view all features.


## Installation

### Docker Setup

See the README in the [.docker-setup](https://github.com/dawitalemu4/gURL/tree/main/.docker-setup) folder for the docker setup guide.

### Local Setup

To locally run gURL, you need to have Rust, Bash, and grpcurl installed on your machine.

1. Download the ZIP of this repo or clone the repository
```bash
git clone https://github.com/dawitalemu4/gURL.git
```

2. Rename the `.env.example` file to `.env` and use your own values (or you can just use the provided values)

5. Run the server (I prefer [cargo-watch](https://crates.io/crates/cargo-watch) for hot reload)
```bash
cargo run
```
or
```bash
cargo watch -x run
```

6. Open your browser and navigate to `localhost:YOURPORT`

Download links: [Rust](https://www.rust-lang.org/tools/install), [Bash](https://git-scm.com/downloads), [grpcurl](https://github.com/fullstorydev/grpcurl).

View [gURL.github.io/install](https://gURL.github.io/install) for more detailed instructions for your OS.


## Startup Shortcuts

Check out my [startup script](https://github.com/dawitalemu4/gURL/blob/main/startup.sh) to easily start up gURL locally from a shortcut on your taskbar, or this [startup script](https://github.com/dawitalemu4/gURL/tree/main/.docker-setup/startup.sh) if you are using docker.

Visit [gURL.github.io/shortcuts](https://gURL.github.io/shortcuts) for demo videos and tutorials on how to make your own shortcut.


## Contributing

I'm open to contributions and suggestions, but fork this project if there are any crazy big changes you want to make that go against the [gURL.github.io/contributing](https://gURL.github.io/contributing).

Run `cargo test` to run the tests against your changes before creating a pr.

Follow the checklist in the [gURL.github.io/contributing](https://gURL.github.io/contributing) if you create a pull request or an issue.


## FAQ

**Q:** **Why this UI style? Just use the terminal?**

**A:** I wanted to be able to see the history of my requests, favorite some requests when I'm working on a specific controller without having to spam up in my terminal, and see the status of the request on the request's preview before I select it, which postman didn't have and I hated guessing and trying each one to see which one works.

**Q:** **Why self-hosted?**

**A:**  I don't want to pay for cloud resources for your convenience. Jokes aside, I didn't want to figure out how to make API requests to an API that is locally hosted from gURL if it was hosted on a deployed server (without having a user download something locally), and I plan on using this on APIs I'm building locally.

Leave a post in the [discussions](https://github.com/dawitalemu4/gURL/discussions) if you have any questions.


## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International Public License - see the [LICENSE.txt](https://github.com/dawitalemu4/gURL/blob/main/LICENSE.txt).