# slint ui

```bash
# 全局安装该工具
cargo install cargo-generate

# 生成项目
cargo generate --git https://github.com/slint-ui/slint-rust-template --name slinttdemo
```

官方提供 2 种使用方式，slint 语言示例基本正确，rust 语言的版本很多示例太老旧，新版本是编译不通过的。

直接通过依赖引入的项目无法编译成功，大概是 README 没修改。
通过模板生成的可以运行，目录结构差异很大。用自定义的 UI 语言（*.slint）描述 UI 再编译。
Cargo.toml 里:
1. build-dependencies 指定了构造工具。
2. build = "build.rs" 指定了构造的文件。

## Vscode 插件

搜索 slint 