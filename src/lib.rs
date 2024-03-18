use std::collections::HashMap;
use reqwest::blocking::Client;
use pyo3::prelude::*;
use pythonize::pythonize;
use pyo3::types::PyDict;


#[pyfunction]
fn send_request(_py: Python, method: String, url: String, data: Option<PyObject>) -> Py<PyAny> {
    let client = Client::new();

    return Python::with_gil(|py| {
        let response = match method.as_str() {
            "GET" => client.get(url).send(),
            "POST" => {
                let data = data.unwrap().extract::<HashMap<String, String>>(py).unwrap();
                client.post(url).json(&data).send()
            },
            "PUT" => {
                let data = data.unwrap().extract::<HashMap<String, String>>(py).unwrap();
                client.put(url).json(&data).send()
            },
            "PATCH" => {
                let data = data.unwrap().extract::<HashMap<String, String>>(py).unwrap();
                client.patch(url).json(&data).send()
            },
            "DELETE" => client.delete(url).send(),
            _ => panic!("Invalid method")
        };

        match response {
            Ok(response) => {
                let status_code: u16 = response.status().as_u16();
                let mut json = response.json::<serde_json::Value>().unwrap();
                let json_with_status = json.as_object_mut().unwrap();
                json_with_status.insert("status_code".to_string(), serde_json::Value::Number(serde_json::Number::from(status_code)));
                pythonize(py, &json_with_status).unwrap()
            },
            Err(err) => {
                let error = err.to_string();
                let error_dict = PyDict::new(py);
                error_dict.set_item("error", error).unwrap();
                error_dict.set_item("status_code", 0).unwrap();
                error_dict.into()

            }
        }
    });
}

/// A Python module implemented in Rust.
#[pymodule]
fn http_requests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_request, m)?)?;
    Ok(())
}
