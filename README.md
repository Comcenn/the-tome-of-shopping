# the-tome-of-shopping

A shopping list in a CLI written in Rust.

Comprised of a workspace containing:

- **api** — Axum-based backend storing shopping list data  
- **cli** — Interactive REPL + command-line client  
- **shared** — Common types, traits, and models used by both crates  

This project demonstrates async execution, REPL-driven UX, and a credential-based login flow.

---

## Workspace Layout

shopping-workspace/
├── api/        # Axum API server
├── cli/        # Interactive CLI + REPL
├── shared/     # Shared models, traits, DTOs
└── Cargo.toml  # Workspace root

---

## Installing Rust

To build and run this workspace, you need a Rust toolchain installed.

The recommended way to install Rust is via **rustup**, the official installer and version manager:

https://www.rust-lang.org/tools/install

After installation, ensure your environment is configured:

```sh
rustc --version
cargo --version
```

---

## Running Tests

From the workspace root, run all tests across the API, CLI, and shared crates:

```sh
cargo test
```

---

## Running API and CLI

We will run in development mode meaning you do not have to build the binaries(but you can if yuo want to).

To run Api cd to workspace root and run(it is better to start it first):

```sh
cargo run -p api
```

For logging on the api you need to set the `RUST_LOG` ENV variable to `"debug"` or which ever level you would like.

Once you have the api running in a seperate terminal you can run the cli with:

```sh
cargo run -p cli
```

---

## First Starting Cli

You will be asked to provide a username and password when you first start cli, these will be stored in a `credentials.toml` file in your home directory. This will be read everytime the Cli starts, if yuo want to change yuor creds update the file. If you want to login as a different user you can use the `login` command and type a username:

```sh
shopping-list> login james
```

Once you press enter you will be asked for a password, once given you will either be a new user or have logged in again to a previous user.

All commands are given <command> <args...> and are all on one line. You can use the `help` command to explore the commands, typing `help` after a command will give you more info on the args. I have not put loads of text to describe them yet but it gives enough to be able to help.

Available commands:

- `list`: Prints shopping list
- `add`: adds an item requires using parameters `--name "Fish Fingers" --price 10.99 --quantity 2`
- `mark`: Sets `Picked Up` to either `true` or `false` (1 or 0) the first argument is the items `id` number which can be found through the `list` command. Adding `--ticked` as an argument sets it to `true` leaving it out sets it to `false`
- `reorder`: Alters an items order use the items `id` as the first argument and then the order you would like to set it as. 
- `total`: Displays an itemised total and if you add `--limit` and then an amount of money you `total` will alert you if your total is over the limit.
- `email`: Send an email(this is a mock) at the minute, but can accept the email address to send the list.
- `login`: login as anothe user see above.



