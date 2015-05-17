extern crate threed_ice_sys as raw;

use std::mem;

use raw::*;

// https://github.com/copies/3d-ice/blob/master/bin/3D-ICE-Emulator.c
#[test]
fn emulator() {
    unsafe {
        let mut stkd: StackDescription_t = mem::uninitialized();
        stack_description_init(&mut stkd);
        stack_description_destroy(&mut stkd);
    }
}
