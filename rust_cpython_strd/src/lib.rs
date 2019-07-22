#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

use levenshtein::levenshtein;

fn edit_distance(_py: Python<'_>, x: &str, y: &str) -> PyResult<usize> {
    Ok(levenshtein(x, y))
}


py_module_initializer!(librust_cpython_strd, initlibrust_cpython_strd, PyInit_rust_cpython_strd, |py, m | {
    r#try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    r#try!(m.add(py, "edit_distance", py_fn!(py, edit_distance(x: &str, y: &str))));
    Ok(())
});
