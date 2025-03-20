# Iced Full Stack Example

## Prerequisites

```bash
# Install required tools
cargo binstall trunk  # install trunk
rustup target add wasm32-unknown-unknown  # Add WebAssembly target
```

## Development

### Frontend Development

1. Desktop Mode (Native)

```bash
cargo run --bin frontend
```

2. Web Mode (WASM with hot-reload)

```bash
# Run from project root
trunk serve
```

### Backend Development

```bash
# First, build frontend
trunk build

# Then run backend server
cargo run --bin backend
```

The server will start at `http://127.0.0.1:3000`

## Project Structure

```
project_root/
├── frontend/ # Iced frontend application
│ ├── src/
│ └── index.html
├── backend/ # Axum backend server
│ └── src/
├── dist/ # Built frontend files (created by trunk)
├── Trunk.toml # Trunk configuration
└── README.md
```
