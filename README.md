See https://github.com/pola-rs/pyo3-polars for more details.

`run.py`
```python
import polars as pl
from expression_lib import Accumulate

df = pl.DataFrame({
    "Time": [5, 3, 4, 1, 2],
    "Minutes": [2, 1, 2, 1, 3],
})

print(
    df.with_columns(
       cum_min_time = pl.col("Time").accumulate.min_time("Minutes")
    )
)
```
Output:
```bash
% cd example/derive_expression
% make run
```
```
shape: (5, 3)
┌──────┬─────────┬──────────────┐
│ Time ┆ Minutes ┆ cum_min_time │
│ ---  ┆ ---     ┆ ---          │
│ i64  ┆ i64     ┆ i64          │
╞══════╪═════════╪══════════════╡
│ 5    ┆ 2       ┆ 7            │
│ 3    ┆ 1       ┆ 4            │
│ 4    ┆ 2       ┆ 4            │
│ 1    ┆ 1       ┆ 2            │
│ 2    ┆ 3       ┆ 2            │
└──────┴─────────┴──────────────┘
```
