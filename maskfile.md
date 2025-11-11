# Lego-Mindstorm-Linefollower

## build

> Build the project via docker-compose

```sh
docker compose run rust bash -c "rustup update &&
 rustup target add armv5te-unknown-linux-musleabi && 
 cargo build --target=armv5te-unknown-linux-musleabi --release"
```

## clean

> cleans cargo and deletes container

```sh
docker compose run --rm rust cargo clean
```

## readme

> generated the README from the maskfile

```sh
cp maskfile.md README.md
```