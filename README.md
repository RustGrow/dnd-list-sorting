Welcome to the Lists Sorting (Drag and Drop) example, created using version 0.6 of the GUI library for Rust, [Dioxus](https://dioxuslabs.com/)!

![Screenshot (253)](https://github.com/user-attachments/assets/6cce780a-50e9-43cc-bcdc-999a3c4b019a)

### Important. This project uses the web platform
# Quick start
1. Reinstall the CLI to the git version.
For Windows users need to install the [Netwide Assembler (NASM)](https://www.nasm.us/pub/nasm/releasebuilds/2.16.03/win64/). On startup it will open the shell and inside execute this command.
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```
2. Clone this repository
```bash
git clone https://github.com/DioxusGrow/dnd-list-sorting
```
and 👇

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve
```

- Open the browser to http://localhost:8080
