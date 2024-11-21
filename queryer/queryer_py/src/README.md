#### use rust in python => 流程

```sh
python3 -m venv .env
source .env/bin/activate
pip install maturin
# 生成 python package
maturin develop
python3
```

```python3
>>> import queryer_py
>>> sql = queryer_py.example_sql()
>>> queryer_py.query(sql)
```
