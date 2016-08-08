extern crate cpython;

use cpython::*;

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python(); // obtain `Python` token

    let sys = py.import("sys").unwrap();
    let version: String = sys.get(py, "version").unwrap().extract(py).unwrap();

    let os = py.import("os").unwrap();
    let getenv = os.get(py, "getenv").unwrap();
    let user: String = getenv.call(py, ("USER",), None).unwrap().extract(py).unwrap();

    println!("Hello {}, I'm Python {}", user, version);

    let functools = py.import("functools").expect("could not import functools");
    // print the first 5 items of the functools dict
    for (x, y) in functools.dict(py).items(py).into_iter().take(5) {
        println!("{:?}: {:?}", x, y);
    }
    println!("...");

    let lambda = py.eval("(lambda x, y: x*y, range(1,10))", None, None).unwrap();
    // ^= getattr("functools", "reduce")(*args, **kwargs)
    let result = functools.call(py, "reduce", lambda, None).expect("the python call failed :(");
    println!("{:?}", result)
}