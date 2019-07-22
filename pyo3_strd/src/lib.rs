use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use levenshtein::levenshtein;

#[pyfunction]
/// Formats the sum of two numbers as string
fn edit_distance(x: &str, y: &str) -> PyResult<usize> {
    Ok(levenshtein(x, y))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pyo3_strd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(edit_distance))?;

    Ok(())
}