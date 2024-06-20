![Check and Deploy](https://img.shields.io/github/check-runs/callieve/metro-map-editor/main?style=flat-square)
![GitHub License](https://img.shields.io/github/license/callieve/metro-map-editor?style=flat-square)
![Website](https://img.shields.io/website?url=https%3A%2F%2Fcalli.dev%2Funiversity%2Fmetro-map&up_message=Online%20editor&up_color=blue&down_message=Temporarily%20unavailable&down_color=red&style=flat-square&label=try%20it%20out!)

# Algorithmically Assisted Metro Map Editor

This project is being made as part of my master thesis / graduation project at the [TU/e] as part of the [algo] research group.

Try it out over at the deployed [site].

## Deploy it yourself

### Prerequisites:

Ensure you have [Rust] installed.
Then, install and enable the rust nightly toolchain for this repository using `rustup toolchain install nightly && rustup override set nightly`.
Since this repository uses WASM, ensure the wasm target has been added to your rust install with `rustup target add wasm32-unknown-unknown`.
As tailwindcss is being used, [npm] is needed.
Once npm has been installed, run `npm install -D` to install the tailwindcss package.
Lastly ensure [trunk] has been installed using `cargo install --locked trunk`.

### Deploy locally

To deploy and run the editor on your local machine, you only need to run `trunk serve --open`.
This will build the project, start listening on localhost:8080 and then open it in your browser.
It will also watch for changes to the project files.

[TU/e]: https://www.tue.nl/en/
[ALGO]: https://algo.win.tue.nl/
[site]: https://calli.dev/university/metro-map
[Rust]: https://www.rust-lang.org/learn/get-started
[NPM]: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
[trunk]: https://trunkrs.dev/
