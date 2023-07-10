# sea-orm

Sqlite 不会自动创建数据库文件。

注：外部有个 seaormdemo 是 actix-web 的。

## sea-orm-migration

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

注：官方教程有 CLI 和 API 文档 2 种，不过都不完整，都需要自己手动改改。出2个版本就是为了好让使用者对照着，根据情况看着办吧。

## sea-orm-entity

```bash
# 安装工具（如果迁移执行则不需要，迁移不是必须的）
cargo install sea-orm-cli

# dbfirst
# 根据数据库生成 entity 
# Sqlite 在不同系统的路径写法差异比较大，需要自行适配。（这些和使用的终端，系统文件系统相关）
# 以下给出几个示例，如果使用 mysql 这种通过 socket 则不会有此类问题。
sea-orm-cli generate entity -u mysql://root:password@localhost:3306/bakeries_db -o src/entities

sea-orm-cli generate entity -u sqlite:./test.db -o src/entities
```

```bat
@rem 不同系统的路径差异比较大，
@rem 此命令是生成文件到主 crate 里面，这与 examples 文件里面生成独立的 crate 不同。
@rem 不清楚是否是被抛弃的做法。
sea-orm-cli generate entity -u sqlite:///D:\\t1.db -o src/entities
```