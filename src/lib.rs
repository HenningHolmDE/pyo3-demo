use std::thread;

use axum::{extract::State, response::Html, routing::get, Router};
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use tokio::sync::oneshot;

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
async fn web_app(callback: PyObject, shutdown_rx: oneshot::Receiver<()>) {
    // prepare app state to be accessed by handler functions
    let state = AppState { callback };

    // build our application with a route
    let app = Router::new().route("/", get(handler)).with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Web server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
            println!("Web server shutting down");
        })
        .await
        .unwrap();
}

/// Start/stop wrapper for the web server
#[pyclass]
struct WebServer {
    handler: PyObject,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

#[pymethods]
impl WebServer {
    #[new]
    fn py_new(handler: PyObject) -> Self {
        Self {
            handler,
            shutdown_tx: None,
        }
    }

    fn start(&mut self) -> PyResult<()> {
        if self.shutdown_tx.is_some() {
            return Err(PyRuntimeError::new_err("Web server already running"));
        }
        let handler_clone = self.handler.clone();

        let (tx, rx) = oneshot::channel::<()>();
        self.shutdown_tx = Some(tx);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(web_app(handler_clone, rx))
        });
        Ok(())
    }

    fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(()).ok();
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<WebServer>()?;
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
