[Source](https://www.youtube.com/watch?v=4nOdO4SDdO0)

[Estimating Pi Using ArcTan](https://www.asc.ohio-state.edu/orban.14/math_coding/pi_series/index.html)

# Notes
- Run Rust code with the `--release` flag.
- Build PyO3 code with the `--release` flag.

# Results
PyO3 and Rust have very similar performance, vanilla Python is ~x50 times slower and [Numba](https://numba.pydata.org/numba-doc/latest/user/5minguide.html) provides huge improvement for high number of iterations.

## 100M
- Python: 8.17
- Numba: 0.297
- Rust: 0.118
- PyO3: 0.119

## 10M
- Python: 0.825
- Numba: 0.183 
- Rust: 0.014
- PyO3: 0.014
