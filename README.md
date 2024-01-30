# pyo3-demo

Demo project for playing around with [PyO3](https://github.com/PyO3/pyo3) for creating native Python extension modules in Rust.

## Requirements (from PyO3)

- Python 3.7 and up (CPython and PyPy)
- Rust 1.56 and up

## Set up virtual environment with [maturin](https://github.com/PyO3/maturin)
```bash
$ python -m venv .venv
$ source .venv/bin/activate
$ pip install maturin
```

## Build Rust crate and install Python module
```bash
$ maturin develop
```

## Run examples
```bash
# Call simple Rust function from Python
$ python examples/sum_as_string.py

# Run web server with Python function callback
$ python examples/run_web_server.py
```
