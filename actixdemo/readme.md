# actix demo

```bash
# 监听
cargo watch -w webdemo/ -x run

# 交叉编译 linux
cargo build --release --target=x86_64-unknown-linux-musl

```