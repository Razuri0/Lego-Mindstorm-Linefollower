# Lego-Mindstorm-Linefollower

## debug

> debug build

```sh
docker compose up -d rust
docker compose exec rust bash -c "
rustup update &&
rustup target add armv5te-unknown-linux-musleabi &&
cargo build --target=armv5te-unknown-linux-musleabi
"
```

## build

> Build the project via docker-compose

```sh
docker compose up -d rust
docker compose exec rust bash -c "
rustup update &&
rustup target add armv5te-unknown-linux-musleabi &&
cargo build --target=armv5te-unknown-linux-musleabi --release
"
```

## clean

> cleans cargo and deletes container

```sh
docker compose up -d rust
docker compose exec rust cargo clean
docker compose down
```

## readme

> generated the README from the maskfile

```sh
cp maskfile.md README.md
```