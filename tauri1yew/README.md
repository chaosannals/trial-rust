# Tauri + Yew Demo

会报拒绝访问的错误（os error 5），如果使用纯 rust 不会报这个错误。
构造工具链有很多问题。

```bash
# 安装 cargo create-tauri-app 构建工具（全局，只需装一次）
cargo install create-tauri-app --locked

# 使用该命令生成项目（交互界面选择配置）
cargo create-tauri-app

# 安装 cargo tauri 开发工具（全局，只需装一次）
cargo install tauri-cli

# 如果是 yew 前端，需要安装 cargo trunk 命令（全局，只需装一次）
cargo install trunk

# 执行项目开发
cargo tauri dev
```

这版本的前端居然可以选择使用 rust 开发。1 年前发布 1.0 的时候前端必须是 js/ts 。

- Vanilla 裸的。。
- Yew 前端项目 Rust 项目
- Leptos
- Sycamore

新项目的构建改变了工具链，旧项目不能构建。

