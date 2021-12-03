#![no_std]
#![feature(alloc)]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::bindings;

pub struct vmmouse_data{
      abs_dev: *mut bindings::input_dev,
      phys: String,
      dev_name :String,
}
struct HelloWorldModule {
    message: String,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let mut a : bindings::bool_ = true;
        let mut test = vmmouse_data {
                abs_dev: 0 as *mut bindings::input_dev,
                phys:String::from("World"),
                dev_name:String::from("Hellow"),
};
        println!("Hello from Rust! {} {}",a,test.phys);
 Ok(HelloWorldModule {
            message: "Hello World!".to_string(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
    }
}

static mut MODULE: Option<HelloWorldModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <HelloWorldModule as linux_kernel_module::KernelModule>::init() {
        Ok(m) => {
            unsafe {
                MODULE = Some(m);
            }
            return 0;
        }
        Err(_e) => {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        MODULE = None;
    }
}

#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";

