# Distribution

## Development

You build the development Docker images with

```sh
docker build --tag distribution-dev .
```

You run the development Docker image with

```sh
./scripts/run-dev.sh
```

Inside the development image

```sh
# TODO: move this to the Dockerfile
export PATH="${PATH}:/root/.cargo/bin"

cd /opt/distribution
cargo clean
cargo test

# generate coverage repot
./scripts/coverage.sh
```
