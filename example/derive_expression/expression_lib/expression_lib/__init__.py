import polars as pl
from polars.type_aliases import IntoExpr
from polars.utils.udfs import _get_shared_lib_location

lib = _get_shared_lib_location(__file__)

@pl.api.register_expr_namespace("accumulate")
class Accumulate:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def min_time(self, other: IntoExpr) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            args=[other],
            symbol="min_time",
            is_elementwise=True,
        )
