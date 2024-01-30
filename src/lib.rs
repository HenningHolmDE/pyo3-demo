use pyo3::prelude::*;
use tiny_http::{Response, Server};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Run tiny-http web server
#[pyfunction]
fn run_web_server(py: Python, callback: PyObject) {
    let server = Server::http("127.0.0.1:8000").unwrap();

    for request in server.incoming_requests() {
        let response = Response::from_string(
            callback
                .call0(py)
                .expect("callback not callable")
                .to_string(),
        )
        .with_header(tiny_http::Header {
            field: "Content-Type".parse().unwrap(),
            value: "text/html; charset=utf8".parse().unwrap(),
        });
        request.respond(response).unwrap();
    }
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
