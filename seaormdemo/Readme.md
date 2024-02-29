# SeaORM

dbormdemo 里面还有示例，有 dbfirst。

此 ORM 的目录有定式，需要按照其目录结构。（参考 SeaORM 源码 examples 目录下的示例）

还有[教程](https://www.sea-ql.org/sea-orm-tutorial)

目前项目目录有点脏，依赖也很乱。预计没个三五年，这个库的使用时稳定不下来了。

注，使用 SQLITE 执行Migration 或者 启动服务器时，如果报不能打开数据库文件，是因为不会自动创建数据库文件，此文件需要手动创建。（这个很坑，猜测是为了保持和 Mysql 一致的行为，即不自动创建数据库）

```bash
# 纯 rust 工具，没有 diesel cli 各种 Windows 安装问题。
cargo install sea-orm-cli

# 生成数据库 migrate 项目
# 第一个文件日期有问题，固定的 20220101 开头，应该是固定的样板文件，一般删了。
# 依赖文件也有问题，不会自动匹配依赖库
# 注：这个就复制几个文件出来，没有做任何适配。配置都是对不上的。
sea-orm-cli migrate init


# 
# sea-orm-cli migrate generate NAME_OF_MIGRATION [--local-time]
sea-orm-cli migrate generate add_new_table --local-time

# 执行
sea-orm-cli migrate up
```

