# tauri demo

```bash
# 安装 cargo 命令工具
cargo install tauri-cli

# 执行初始化 src-tauri 由改命令生成
cargo tauri init

# 启动开发
cargo tauri dev
```

```pwsh
$env:HTTP_PROXY="http://127.0.0.1:1088"
$env:HTTPS_PROXY="https://127.0.0.1:1088"
```

生成的 exe 是单文件程序，首次打开页面会空白好一会。

基于 wix3 的 msi 打包 bundle 可能因为墙的原因 TLS 错误无法打包。

此项目在时隔 1 年后再次执行 build 就不能成功了。
大概率因为版本控制关系拉到了新库，构建流程又改变了。
