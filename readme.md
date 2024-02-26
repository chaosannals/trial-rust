# trial rust

## rustup

```bash
# 清华源
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
```

```bat
@rem 清华源
set RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
```


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

# 列举所有编译工具集合
rustup target list

# 查看信息 可查看已安装 target
rustup show

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


## 交叉编译

### [musl](https://musl.cc)

注：网上一些博客是错的，他实验的时候切换多个版本，导致 target 里面其实是多个版本的缓存结果，而他记录到博客的，其实是混合的结果，他并没有从头走一遍流程。

Win10 下载工具集

[x86_64-linux-musl-cross](https://musl.cc/x86_64-linux-musl-cross.tgz) (这个亲测是不行的，应该是原作者试的时候由于缓存导致这个成功了。)

[x86_64-w64-mingw32-cross](https://musl.cc/x86_64-w64-mingw32-cross.tgz)(这个可以)

解压并添加目录下 bin 目录到 PATH 里面。

修改 ~/.cargo/config ，添加编译参数 配置链接器为 lld 
```ini
[target.x86_64-unknown-linux-musl]
linker = "x86_64-w64-mingw32-gcc"
rustflags = ["-Clinker=rust-lld"]
# 这2个 rustflags 应该是一个意思，但是可能由于编译器版本需要使用指定的。测试时使用上面个这个。
rustflags = ["-C", "linker-flavor=ld.lld"]
```

```bash
# 添加 linux 目标用作交叉编译
rustup target add x86_64-unknown-linux-musl

# 指定编译目标
cargo build --release --target=x86_64-unknown-linux-musl
```

注：交叉编译遇到 依赖 C 库的库时需要安装对应的编译工具和依赖库，很是麻烦。建议使用 WSL 或 虚拟机 Docker 等去编译。

### cross

```bash
# 使用 docker 进行编译的交叉编译工具。
cargo install cross --git https://github.com/cross-rs/cross
```
