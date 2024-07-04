# Create C.R.A.P App

A CLI tool I created to simplify the process of creating new projects.

## Features

The tool includes the following technologies:

- **Clerk**: For authentication
- **React**: Pre-configured with Tailwind CSS and Shadcn UI
- **Actix Web**: For APIs
- **PostgreSQL**: As the database

## Installation

Follow these steps to install the CLI tool:

1) Clone the repo and cd into it
2) Build the executable
```bash
cargo build --release
```
3) Move the executable to a path included in you system PATH variable
```bash
#example
sudo mv target/release/create-crap-app /usr/local/bin
```
4) Run the cli to create your project
```bash
create-crap-app
```
