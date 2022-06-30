use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let executor = PyModule::import(py, "executor")?;
        executor.getattr("executor_foo")?.call1(())?;
        Ok(())
    })
}



