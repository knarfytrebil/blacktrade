extern crate cpython;

use cpython::{PyDict, PyResult, Python};

fn main() {
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

fn hello(py: Python) -> PyResult<()> {
    let code = "\n\nimport random\n\ndef trade():\n    if random.randint(0, 2) == 0:\n        return 1    \n    return sum(data)\n";
    let data = vec![1, 2, 3, 4];

    let locals = PyDict::new(py);
    py.run(code, None, Some(&locals));

    locals.set_item(py, "data", data);

    let res = py.eval("trade()", None, Some(&locals))?.extract(py)?;
    println!("{}", res);

    Ok(())
}
