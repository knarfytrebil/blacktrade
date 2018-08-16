use cpython::{PyDict, PyResult, Python, ToPyObject};

pub fn run<TPO>(py: Python, data: &Vec<TPO>, code: &String) -> PyResult<i64>
where
    TPO: ToPyObject,
{
    match py.run(code, None, None) {
        Ok(_) => {
            let globals: PyDict = py.eval("globals()", None, None)?.extract(py)?;
            globals.set_item(py, "data", data)?;
            let res = py.eval("main()", Some(&globals), None)?.extract(py)?;
            return Ok(res);
        }
        Err(e) => {
            return Err(e);
        }
    }
}
