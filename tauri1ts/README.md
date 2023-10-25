# Tauri + Vue 3 + TypeScript Demo

```bash
# 安装 cargo create-tauri-app 构建工具（全局，只需装一次）
cargo install create-tauri-app --locked

# 使用该命令生成项目（交互界面选择配置）
cargo create-tauri-app

# 使用的 npm i 安装
npm i

# 启动开发调试
npm run tauri dev

# 打包 需要修改 tauri.conf.json 里面 identifier ，不能是默认的 com.tauri.dev
# 下载打包工具需要翻墙
npm run tauri build
```

现阶段 tauri 开始引入一堆 rust 前端，使用 js/ts 的前端项目的优势还是有的，比较早实现，比较稳定，且 js/ts 的框架更成熟。尝试几个按官方流程的 rust 前端，几个编译失败了，少数成功的几个也有一些问题。
