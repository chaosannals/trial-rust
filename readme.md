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

## C 混编的问题

```bash
# 查看 glic6 版本
strings /lib/libc.so.6 | grep GLIBC_
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

### wsl

```bat
@rem 关闭再打开 = 重启
wsl --shutdown
wsl
```

```bat
@rem 切换 默认系统
wslconfig /setdefault Ubuntu-22.04
```

```bash
# 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# vim 打开 /etc/profile 把 PATH 的修改命令写入
vi /etc/profile
```

```bash
# 如果 $HOME/.cargo/bin 存在则加入 PATH
if [ -d "$HOME/.cargo/bin" ] ; then
	PATH="$HOME/.cargo/bin:$PATH"
fi
```
注：需要手动添加 $HOME/.cargo/bin 到 PATH 环境变量，不然找不到 rustup


#### musl

ubuntu 安装

```bash
# 会自动安装 musl-dev
apt install musl-tools
```

编译
```bash
# 下载 musl
wget https://musl.libc.org/releases/musl-1.2.4.tar.gz

# 解压
tar zxvf musl-1.2.4.tar.gz

#
cd musl-1.2.4
./configure
make
make install
```

