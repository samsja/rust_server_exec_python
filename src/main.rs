use pyo3::prelude::*;
use pyo3::types::PyModule;
use warp::Filter;

#[tokio::main]
async fn main() {
    pyo3::prepare_freethreaded_python();
    let executor_module: Py<PyModule> =
        Python::with_gil(|py| PyModule::import(py, "executor")?.extract()).unwrap();

    let executor: Py<PyAny> = Python::with_gil(|py| {
        executor_module
            .getattr(py, "MySlowToLoadExec")?
            .call1(py, ())
    })
    .unwrap();

    let hello = warp::path::end().map(move || get_exec_return(&executor));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

fn get_exec_return(executor: &Py<PyAny>) -> String {

    let return_message = Python::with_gil(|py| {
        executor
            .as_ref(py)
            .getattr("foo")
            .unwrap()
            .call1(())
            .unwrap()
            .to_string()
    });

    return_message
}
