# Man on a Mission

> Reduce the struggle to manage missions!

## Requirements

You will need to install the following tools:

- [PostgreSQL](https://www.postgresql.org/download/)
- [Rust toolchain](https://www.rust-lang.org/tools/install)

After cloning the project, to ensure that the commit messages are in [the appropriate format](https://www.conventionalcommits.org/),
please run the following commands:

```bash
npm install
npx husky install
```

Moreover, it is advisable to install the SQLx CLI:

```bash
cargo install sqlx-cli
```

Make a copy of the `.env.sample` file and rename it to `.env`.
Fill in the appropriate values for the environment variables.

## Building

To build the project, run the following command:

```bash
cargo build
```

## Running

To run the project, run the following command:

```bash
cargo run
```

Make sure that an active PostgreSQL instance is running.
