# [trial-rust](https://github.com/chaosannals/trial-rust)

```bash
# storage
cargo run --bin trial-rust-storage

# hostsman
cargo run --bin trial-rust-hostsman

# mysql
cargo run --bin trial-rust-mysql

# iced
cargo run --bin trial-rust-icedui

# azul 构建麻烦

# imgui
cargo run --bin trial-rust-imgui

# winit
cargo run --bin trial-rust-winit

# kas
cargo run --bin trial-rust-kasui

# conrod

# native-windows-gui 目前效果最好的GUI
cargo run --bin trial-rust-nwgui

# uniqrow
cargo run --bin trial-rust-uniqrow
cargo build --bin trial-rust-uniqrow --release
```

## rustup 镜像

```sh
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

```cmd
set RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
set RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

## 创建项目

```bash
# 创建可执行项目
cargo new --bin <name>
```
