<h1 align="center">
    <img src="https://github.com/user-attachments/assets/6c08e081-0ff1-43db-8abc-77b37cb6707c" alt="gURL icon" style="width: 50px; height: 50px;">
    <p>gURL</p>
</h1>

gURL is a self-hosted dev tool that achieves what postman does but with my personal preferences in its functionality and design. Made with Rust (Axum), HTMX, and grpcurl.

Everything is pretty much the same as [keycurl](https://github.com/dawitalemu4/keycurl), just a gRPC version. Visit [keycurl.github.io/features](https://keycurl.github.io/features) to view all features.


## Differences from Postman

- Use your keyboard to do everything if you want to (literally everything)
- Less cluttered UI and makes the most important things always accessible
- No need to sign up or log in and give your data to a third party
- But keep your request history and build profiles to save favorite requests (and autofill the form with the saved request!)
- And more...


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


## Startup Shortcuts

Check out my [startup script](https://github.com/dawitalemu4/gURL/blob/main/startup.sh) to easily start up gURL locally from a shortcut on your taskbar, or this [startup script](https://github.com/dawitalemu4/gURL/tree/main/.docker-setup/startup.sh) if you are using docker.

Visit [keycurl.github.io/shortcuts](https://keycurl.github.io/shortcuts) for demo videos and tutorials on how to make your own shortcut.


## Contributing

I'm open to contributions and suggestions, but fork this project if there are any crazy big changes you want to make that go against the [keycurl.github.io/contributing](https://keycurl.github.io/contributing).

Run `cargo test` to run the tests against your changes before creating a pr.

Follow the checklist in the [keycurl.github.io/contributing](https://keycurl.github.io/contributing) if you create a pull request or an issue.


## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International Public License - see the [LICENSE.txt](https://github.com/dawitalemu4/gURL/blob/main/LICENSE.txt).