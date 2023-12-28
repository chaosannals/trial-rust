# PyO3 demo

## 全局安装工具

```bash
# 他自己提供了个开发工具，安装此开发工具
pip install maturin

# 或者
pipx install maturin

# 安装后会在 ~/.local/bin 里面
# Windows 用户需要自己加入这个路径到 PATH

# build 指令必须在项目目录中执行，即使使用了workspace，但生成还是在 workspace 的 target 里 wheels 里面有个 python whl 包
maturin build
```

## 在 Python 虚拟环境中安装工具

```bash
# 初始化项目

# 生成 python 虚拟环境
python -m venv .env

# 进入虚拟环境（Linux）
source .env/bin/activate
# Windows 下是  ./.env/Scripts/activate

# 安装工具
pip install maturin

# 创建项目目录并进入
mkdir o3pydemo
cd o3pydemo

# 初始化项目
maturin init --bindings pyo3
# 初始化后项目可能比较老，因为模板原因，修改 Cargo.toml 依赖为新版 pyo3 

# 执行开发（此步骤完成：编译，并安装到 Python 虚拟环境中）
maturin develop

# 进入 python 交互式编程界面
python
# 此时已经可以使用该库了。
```
