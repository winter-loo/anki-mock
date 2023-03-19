use pyo3::prelude::*;
use pyo3::types::PyBytes;
use anki::backend::init_backend;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use anki::backend::Backend as RustBackend;

#[pyfunction]
pub fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right + 4)
}

create_exception!(rsbridge, BackendError, PyException);

#[pyclass]
struct Backend {
    backend: RustBackend,
}

#[pyfunction]
fn open_backend(init_msg: &PyBytes) -> PyResult<Backend> {
    match init_backend(init_msg.as_bytes()) {
        Ok(backend) => Ok(Backend { backend }),
        Err(e) => Err(PyException::new_err(e)),
    }
}

#[pymethods]
impl Backend {
    fn command(
        &self,
        py: Python,
        service: u32,
        method: u32,
        input: &PyBytes,
    ) -> PyResult<PyObject> {
        let in_bytes = input.as_bytes();
        py.allow_threads(|| self.backend.run_method(service, method, in_bytes))
            .map(|out_bytes| {
                let out_obj = PyBytes::new(py, &out_bytes);
                out_obj.into()
            })
            .map_err(BackendError::new_err)
    }
}

#[pymodule]
fn rsbridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Backend>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_wrapped(wrap_pyfunction!(open_backend)).unwrap();
    Ok(())
}


