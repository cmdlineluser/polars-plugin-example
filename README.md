See https://github.com/pola-rs/pyo3-polars for more details.

`expressions.rs`
```rust
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

#[polars_expr(output_type=Int64)]
fn min_time(inputs: &[Series]) -> PolarsResult<Series> {
    let time = inputs[0].i64()?;
    let minutes = inputs[1].i64()?;

    let initial_value = time.get(0).unwrap() + minutes.get(0).unwrap();

    let out = time
        .into_iter()
        .zip(minutes.into_iter())
        .scan(initial_value, |state, (time, minutes)| {
            let time = time?;
            let minutes = minutes?;
            if *state > time {
                *state = time + minutes
            }
            Some(*state)
        })
        .collect();
    Ok(out)
}
```
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
