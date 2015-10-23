#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use std::ptr;
use libc::{c_uint, size_t, c_double, c_void};

mod bindings;

#[test]
fn it_works() {
}
