pub mod bindings;

use bindings::uint32;

#[no_mangle]
pub extern "C" fn mult(x: uint32, y: uint32) -> uint32 {
    let mut result: uint32 = 0;
    let mut i: uint32 = 0;
    while i < y {
        result = unsafe { bindings::add(result, x) };

        i = unsafe { bindings::add(i, 1) };
    }

    return result;
}