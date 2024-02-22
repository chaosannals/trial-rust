# trial rust

## cargo 常用命令

```bash
# 可执行文件
cargo new --bin yourapp
# 库
cargo new --lib yourlib

# init 唯一区别于 new 的点就是 不会建目录，而是在目录内初始化。
cargo init --bin yourapp
```

```bash
# workspace 指定构建项目
# -p 更通用，可以指定 lib 库类型
cargo build -p yourapp
cargo build --bin yourapp

# 指定编译目标
cargo build --release --target=x86_64-unknown-linux-musl

# workspace 指定运行项目
# -p 更通用，可以指定 lib 库类型
cargo run -p yourapp
cargo run --bin yourapp
```

## cargo generate (模板生成工具)

```bash
# 全局安装该工具
cargo install cargo-generate

# 指定 git 安装模板(这里是 slint 的示例)
cargo generate --git https://github.com/slint-ui/slint-rust-template
```

## rustup 命令

```bash
# 更新
rustup update

# 添加 linux 目标用作交叉编译
rustup target add x86_64-unknown-linux-musl

# 列举 编译工具集合
rustup target list

# 指定编译目标
cargo build --release --target=x86_64-unknown-linux-musl
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


## 常用工具库

### cargo-watch

类似 nodemon 这种监听代码变化并自动重启命令的命令工具

```bash
# 安装
cargo install cargo-watch

# -w 指定监听源码目录
# -x 指定要执行的命令
cargo watch -w src/ -x run
```


## 数据库

- [sqlx]()
- [diesel]()
- [rusqlite](https://github.com/rusqlite/rusqlite)