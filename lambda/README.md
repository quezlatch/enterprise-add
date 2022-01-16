AWS Rust Lambda
===============

* https://github.com/awslabs/aws-lambda-rust-runtime

Needs the cross compile targets

* rustup target add x86_64-unknown-linux-gnu
* rustup target add aarch64-unknown-linux-gnu

build in docker cos macos currently has problems with cross compliling (linker probs :( ))

To build lambda

```
LAMBDA_ARCH="linux/amd64"
RUST_VERSION="latest"
RUST_TARGET="x86_64-unknown-linux-gnu"
docker run \
  --platform ${LAMBDA_ARCH} \
  --rm --user "$(id -u)":"$(id -g)" \
  -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
  cargo build --release --target ${RUST_TARGET}
zip -j rust.zip target/x86_64-unknown-linux-gnu/release/bootstrap
```