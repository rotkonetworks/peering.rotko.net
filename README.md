# Peering Rotko Networks

Frontend project of a web app with information about Rotko Network peering.

## Install

### Tailwind

Install npm:

https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

Install the Tailwind CSS CLI:

```bash
npm install tailwindcss @tailwindcss/cli
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

Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./app/resources/style/tailwind.css --watch
```

Run the following command in the root of your project to start developing with the default platform (web):

```bash
cd app
dx serve
```
To run for a different platform, use the `--platform platform` flag. E.g.: `desktop`.

## Build

To build, run:

```bash
cd app
dx bundle --platform web
```

This will create the files to target folder:

```
target/dx/peering-rotko-net/release/web/public
```

## Deploy

Copy to the server.

```bash
scp -r target/dx/peering-rotko-net/release/web peering@peering.rotko.net:/home/peering
```