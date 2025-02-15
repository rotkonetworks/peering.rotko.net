# Peering Rotko Networks

Frontend project of a web app with information about Rotko Network peering.

## Setup

### Tailwind

Install npm:

https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

Install the Tailwind CSS CLI:

```bash
npm install tailwindcss @tailwindcss/cli
```

Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./resources/style/output.css --watch
```

### Dioxus CLI

Install Dioxus CLI:

```bash
# skip if you already have cargo-binstall
cargo install cargo-binstall

# install the precompiled `dx` tool
cargo binstall dioxus-cli
```

You may need to add to the path the bin of the Dioxus CLI installation:

```bash
export PATH="$PATH:/path/to/dioxus_cli"
```

## Run

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```
To run for a different platform, use the `--platform platform` flag. E.g.: `desktop`.