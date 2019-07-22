rust-cpython:
	cd rust_cpython_strd && RUSTFLAGS="-C target-cpu=native" cargo build --release && cd .. && cp rust_cpython_strd/target/release/librust_cpython_strd.so rust_cpython_strd.so

pyo3:
	cd pyo3_strd && RUSTFLAGS="-C target-cpu=native" cargo build --release && cd .. && cp pyo3_strd/target/release/libpyo3_strd.so pyo3_strd.so
