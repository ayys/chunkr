This is holds the core of the chunkr api. The API consists of multiple services that work together to extract structured data from documents.

# Development

## Setup

```bash
# Install direnv + nix if you plan to use the provided flake
curl -fsSL https://direnv.net/install.sh | bash
curl -fsSL https://nixos.org/nix/install | sh

# Allow direnv to load the environment (provides toolchain, pdfium deps, etc.)
cd core && direnv allow

# Copy the example env file
cp .env.example .env
```

> The `flake.nix` pins the Rust toolchain, pdfium dependencies, and auxiliary CLI tools. If you prefer not to use Nix, ensure you install Rust (`rustup`), `libclang`, `libssl`, `libpq`, `pdftoppm`, and `libreoffice` manually.

## Running the Services

### Rust services

```bash
# Start the server
cargo run

# Start the workers
## Each worker is 1 task
cargo run --bin task
```

### Other Services

To run the other services it is recommended to use the docker compose file in the root of the repo.
For Docker Compose setup and usage instructions, please refer to [Quick Start with Docker Compose](../README.md#quick-start-with-docker-compose).

Set replicas to 0 for services you don't want to run/aren't actively being worked on.

