# actix demo

```bash
# 监听
cargo watch -w webdemo/ -x run

# 编译 linux
cargo build --release --target=x86_64-unknown-linux-musl

```

## JSON serde

lowercase 全小写
UPPERCASE 全大写
PascalCase 大驼峰
camelCase 小驼峰
snake_case 小写下划线
SCREAMING_SNAKE_CASE 大写下划线
kebab-case 小写中划线
SCREAMING-KEBAB-CASE 大写中划线

```rust
#[serde(rename_all = "camelCase")]
```