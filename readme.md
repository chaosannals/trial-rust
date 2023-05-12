# trial rust

## cargo 常用命令

```bash
# workspace 指定构建项目
cargo build --bin yourapp

# workspace 指定运行项目
cargo run --bin yourapp
```

## cargo generate (模板生成工具)

```bash
# 全局安装该工具
cargo install cargo-generate

# 指定 git 安装模板(这里是 slint 的示例)
cargo generate --git https://github.com/slint-ui/slint-rust-template
```

## 配置镜像

~/.cargo 目录下，可以创建 config 文件配置镜像。
这个 config 文件不一定有，没有自己创建。

配置

```ini
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```
