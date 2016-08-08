
use std::os::raw::{c_int, c_long};

#[no_mangle]
pub extern "C" fn square(x: c_int) -> c_long {
    (x as c_long) * (x as c_long)
}
#[test]
fn test_square() {
    let t = [(2, 4), (-2, 4), (100, 10_000)];
    for &(val, res) in t.iter() {
        assert_eq!(square(val), res);
    }
}
