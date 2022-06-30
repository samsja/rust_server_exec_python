use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let executor_module = PyModule::import(py, "executor")?;
        let executor = executor_module.getattr("MySlowToLoadExec")?.call1(())?;
        executor.getattr("foo")?.call1(())?;
        Ok(())
    })
}



