# actix demo

```bash
# 监听
cargo watch -w webdemo/ -x run -p webdemo

cargo run -p webdemo

# 编译 linux
cargo build --release --target=x86_64-unknown-linux-musl

```

## Sea Orm

```powershell
# migrate 配置链接字符串
$env:DATABASE_URL=mysql://root:123456@127.0.0.1/demo
```

```bat
@rem migrate 配置链接字符串
set DATABASE_URL=mysql://root:123456@127.0.0.1/demo

@rem 生成 migrate 项目
sea-orm-cli migrate init

@rem 生成脚本文件(这个不智能，没有修改模板，只是直接复制模板文件。)
sea-orm-cli migrate generate create_user_table

@rem 执行 migration
cargo run -p migration
```

```bash
# migrate 配置链接字符串
export DATABASE_URL="mysql://root:123456@127.0.0.1/demo"
```

## JSON serde

lowercase 全小写
UPPERCASE 全大写
PascalCase 大驼峰
camelCase 小驼峰
snake_case 小写下划线
SCREAMING_SNAKE_CASE 大写下划线
kebab-case 小写中划线
SCREAMING-KEBAB-CASE 大写中划线

```rust
#[serde(rename_all = "camelCase")]
```