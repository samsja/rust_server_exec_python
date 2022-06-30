use pyo3::prelude::*;


use warp::Filter;

#[tokio::main]
async fn main() {
    
    pyo3::prepare_freethreaded_python();
    let hello = warp::path::end()
        .map(return_message);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


fn return_message() -> String {

    let return_statement = call_exec();

    match return_statement {
        Ok(message) => { message }
        Err(_) => { String::from("internal error") }
    }
    
}

fn call_exec() -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let executor_module = PyModule::import(py, "executor")?;
        let executor = executor_module.getattr("MySlowToLoadExec")?.call1(())?;
        let return_string = executor.getattr("foo")?.call1(())?.to_string();
        Ok(return_string)
    })
}



