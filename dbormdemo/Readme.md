## diesel

这个库不及 sqlx 这种纯 rust 开发的库安装方便。
Windows 下依赖库安装和编译工具链很麻烦，
Linux 下应该没这个问题。

注：此示例的 t1.db 和 test.db 路径没有安排一致，所以不能直接运行。
代码仅供参考，执行时候要根据语义，自行复制 t1.db 和 test.db 的替换。

```bash
cargo build -p dieseldexe

cargo build -p seaormdemo
```

### 命令行工具 diesel cli

这个工具是 rust 和 c 库混合编译，依赖 c 库，编译过程很繁琐。

- libpq for the PostgreSQL backend
- libmysqlclient for the Mysql backend
- libsqlite3 for the SQlite backend


```bash
# 安装命令行工具（windows 下会报 找不到 mysqlclient 静态库，无法编译 mysqlclient-sys ）
cargo install diesel_cli

# 指定只安装 postgresql（Windows 下报 libpq.lib 找不到）
cargo install diesel_cli --no-default-features --features postgres
```
