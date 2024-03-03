use std::{process::abort, thread};

use axum::{extract::State, response::Html, routing::get, Router};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// App state accessible by handlers
#[derive(Clone, Debug)]
struct AppState {
    callback: PyObject,
}

/// Example handler calling Python callback function
async fn handler(State(state): State<AppState>) -> Html<String> {
    Html(Python::with_gil(|py| {
        state
            .callback
            .call0(py)
            .expect("callback not callable")
            .to_string()
    }))
}

/// Set up and run axum web service
async fn web_app(callback: PyObject) {
    // prepare app state to be accessed by handler functions
    let state = AppState { callback };

    // build our application with a route
    let app = Router::new().route("/", get(handler)).with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Run axum web server in Tokio runtime
#[pyfunction]
fn run_web_server(py: Python, callback: PyObject) -> PyResult<()> {
    let asyncio = py.import("asyncio")?;
    let event_loop = asyncio.call_method0("new_event_loop")?;
    asyncio.call_method1("set_event_loop", (event_loop,))?;

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(web_app(callback))
    });

    let event_loop = (*event_loop).call_method0("run_forever");
    if event_loop.is_err() {
        abort();
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_web_server, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_as_string() {
        assert_eq!(sum_as_string(10, 5).unwrap(), String::from("15"));
    }
}
