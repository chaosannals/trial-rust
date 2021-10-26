# wasm-pack

[wasm-pack cli 下载页面](https://rustwasm.github.io/wasm-pack/installer/)

```bash
# 安装，由于是 0.x 使用了高版本 rust 的特性需要最新rust
cargo install cargo-generate

# 生成项目
wasm-pack new <name>
wask-pack new wasmpd

# 项目里面，构建。
wasm-pack build
```

## wasmpd

由 wasm-pack new 生成。

```bash
# 会生成 pkg 目录下，就是。
wasm-pack build
```

## wasmprslib

由 cargo new 生成，修改依赖。

```bash
# 会生成 pkg 目录下，就是。
wasm-pack build
```
