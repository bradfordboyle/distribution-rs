Distribution
============

A Rust verion of philovivero's [distribution][] script.

```sh
$ cat tests/stdin.01.txt | distribution --graph
                   Key|     Ct    (Pct) Histogram
----------------------|--------------------------------------------------------
         /etc/mateconf|7780758 (44.60%) --------------------
           /etc/brltty|3143272 (18.02%) --------
       /etc/apparmor.d|1597915  (9.16%) ----
/etc/bash_completion.d| 597836  (3.43%) --
             /etc/mono| 535352  (3.07%) --
              /etc/ssl| 465414  (2.67%) --
          /etc/ardour2| 362303  (2.08%) -
              /etc/X11| 226309  (1.30%) -
      /etc/ImageMagick| 202358  (1.16%) -
           /etc/init.d| 143281  (0.82%) -
```

Building
--------

You can build the binary with

```sh
cargo build [--release]
```

The resulting binary will be in `./targe/[debug|release]/distribution`

Testing
-------

You can run the tests with

```sh
cargo test
```

Docker Image
------------

A Dockerfile for development and coverage reporting is included. This will build a Docker image with [`kcov`][] installed.

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

[distribution]: https://github.com/philovivero/distribution
[`kcov`]: https://github.com/SimonKagstrom/kcov
