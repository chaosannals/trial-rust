# sea-orm

Sqlite 不会自动创建数据库文件。

注：外部有个 seaormdemo 是 actix-web 的。

```bash
# 安装命令行工具
cargo install sea-orm-cli

# 迁移命令查看
sea-orm-cli migrate -h

# 初始化迁移，在项目里执行会多出迁移项目。（只是复制出一个模板，没有自动化项目配置，目前是个半残命令）
sea-orm-cli migrate init

# 创建迁移脚本文件。此脚本会自动更新 lib.rs 文件。
sea-orm-cli migrate generate create_bakery_table
sea-orm-cli migrate generate create_chef_table
```

```bash
# Linux
export DATABASE_URL="sqlite:./test.db"
sea-orm-cli migrate refresh
```

```bat
@rem Windows cmd 执行，不过路径是相对临时文件的。指定绝对路径。
@rem 必须是 cmd ，在 powershell 下 set 无效。
@rem cmd 不能加双引号，路径还必须是绝对的。很不好用。
@rem 这个框架没有好好适配 Windows，这样 Linux 和 Windows 配置不能一致。不过执行可以通过。
set DATABASE_URL=sqlite://D:\t1.db
sea-orm-cli migrate refresh
```