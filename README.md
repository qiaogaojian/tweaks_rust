## Rust 和 Python 交互流程

新建项目

```shell
cargo new --lib rustcore
```

修改 Cargo.toml 配置项目为 C动态库类型 cdylib

```toml
[lib]
name = "rustcore"
# "cdylib" is necessary to produce a shared library for Python to import from.
# 这个表示编译时候使用的c标准的动态库 Python的底层就是用c语言写的，必须是c标准库，Python才能导入
create-type = ["cdylib"]
```

添加 pyo3

```shell
cargo add pyo3@0.20.3 --features "extension-module"
```

创建 python 虚拟环境

```shell
micromamba create -p ./venv python=3.9
```

测试交互代码

```rust
use pyo3::prelude::*;

#[pyfunction]
fn say_hello_to_python() -> PyResult<String> {
    Ok("Hello, Python!".to_string())
}

#[pymodule]
fn rustcore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello_to_python, m)?)?;
    Ok(())
}
```

python 端配置

```shell
pip install maturin -i https://pypi.tuna.tsinghua.edu.cn/simple
```

python 端安装或更新 rust 端库

```shell
micromamba activate ./venv
cd lib/rustcore
maturin develop --release
```

python 端测试交互代码

```python
import rustcore

rustcore.say_hello_to_python()
```
