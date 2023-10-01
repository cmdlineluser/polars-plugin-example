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
