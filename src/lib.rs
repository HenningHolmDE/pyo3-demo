use axum::{response::Html, routing::get, Router};
use pyo3::{prelude::*, types::PyFunction};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Set up and run axum web service
async fn web_app(string_handler: impl Fn() -> String) {
    // build our application with a route
    let app = Router::new().route("/", get(async { Html(string_handler()) }.await));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Run axum web server in Tokio runtime
#[pyfunction]
fn run_web_server(callback: &PyFunction) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(web_app(|| -> String {
        callback
            .call0()
            .expect("callback not callable")
            .str()
            .expect("callback result not convertible to string")
            .to_string()
    }))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_web_server, m)?)?;
    Ok(())
}
