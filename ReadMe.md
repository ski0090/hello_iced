# iced full stack example

## Frontend

### Run with Desktop

```powershell
cd frontend
cargo run
```

### Run with trunck

```powershell
cargo binstall trunk # install trunk
rustup target add wasm32-unknown-unknown # Add compile target
cd frontend
trunk serve
```

## backend

Run with webserver

```powershell
cd frontend
trunck build
cd ../backend
cargo run
```
