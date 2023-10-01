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
