/*
//@authors: Nitheesh Prakash, Shriram Raja, Sairam Ganti
*/

// #define VMMOUSE_PROTO_MAGIC			0x564D5868U
// #define VMMOUSE_PROTO_PORT			0x5658

#![no_std]
#![feature(alloc)]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use alloc::vec::Vec;
use linux_kernel_module::bindings;
use linux_kernel_module::c_types;
use linux_kernel_module::println;

extern "C" {
    pub fn input_report_key_wrapper(
        dev: *mut bindings::input_dev,
        code: c_types::c_uint,
        value: c_types::c_int,
    );
    pub fn psmouse_dbg_wrapper(psmouse: *mut bindings::psmouse);
    pub fn psmouse_reset_wrapper(psmouse: *mut bindings::psmouse);
}

const VMMOUSE_PROTO_MAGIC: u32 = 0x564D5868;
const VMMOUSE_PROTO_PORT: u32 = 0x5658;

// /*
//  * Main commands supported by the vmmouse hypervisor port.
//  */
// #define VMMOUSE_PROTO_CMD_GETVERSION		10
// #define VMMOUSE_PROTO_CMD_ABSPOINTER_DATA	39
// #define VMMOUSE_PROTO_CMD_ABSPOINTER_STATUS	40
// #define VMMOUSE_PROTO_CMD_ABSPOINTER_COMMAND	41
// #define VMMOUSE_PROTO_CMD_ABSPOINTER_RESTRICT   86

const VMMOUSE_PROTO_CMD_GETVERSION: i32 = 10;
const VMMOUSE_PROTO_CMD_ABSPOINTER_DATA: i32 = 39;
const VMMOUSE_PROTO_CMD_ABSPOINTER_STATUS: i32 = 40;
const VMMOUSE_PROTO_CMD_ABSPOINTER_COMMAND: i32 = 41;
const VMMOUSE_PROTO_CMD_ABSPOINTER_RESTRICT: i32 = 86;

// /*
//  * Subcommands for VMMOUSE_PROTO_CMD_ABSPOINTER_COMMAND
//  */
// #define VMMOUSE_CMD_ENABLE			0x45414552U
// #define VMMOUSE_CMD_DISABLE			0x000000f5U
// #define VMMOUSE_CMD_REQUEST_RELATIVE		0x4c455252U
// #define VMMOUSE_CMD_REQUEST_ABSOLUTE		0x53424152U

const VMMOUSE_CMD_ENABLE: u32 = 0x45414552;
const VMMOUSE_CMD_DISABLE: u32 = 0x000000f5;
const VMMOUSE_CMD_REQUEST_RELATIVE: u32 = 0x4c455252;
const VMMOUSE_CMD_REQUEST_ABSOLUTE: u32 = 0x53424152;

// #define VMMOUSE_ERROR				0xffff0000U
const VMMOUSE_ERROR: u32 = 0xffff0000;

// #define VMMOUSE_VERSION_ID			0x3442554aU
const VMMOUSE_VERSION_ID: u32 = 0x3442554a;

// #define VMMOUSE_RELATIVE_PACKET			0x00010000U
const VMMOUSE_RELATIVE_PACKET: u32 = 0x00010000;

// #define VMMOUSE_LEFT_BUTTON			0x20
// #define VMMOUSE_RIGHT_BUTTON			0x10
// #define VMMOUSE_MIDDLE_BUTTON			0x08

const VMMOUSE_LEFT_BUTTON: u32 = 0x20;
const VMMOUSE_RIGHT_BUTTON: u32 = 0x10;
const VMMOUSE_MIDDLE_BUTTON: u32 = 0x08;

// /*
//  * VMMouse Restrict command
//  */
// #define VMMOUSE_RESTRICT_ANY                    0x00
// #define VMMOUSE_RESTRICT_CPL0                   0x01
// #define VMMOUSE_RESTRICT_IOPL                   0x02

const VMMOUSE_RESTRICT_ANY: u32 = 0x00;
const VMMOUSE_RESTRICT_CPL0: u32 = 0x01;
const VMMOUSE_RESTRICT_IOPL: u32 = 0x02;

// #define VMMOUSE_MAX_X                           0xFFFF
// #define VMMOUSE_MAX_Y                           0xFFFF

const VMMOUSE_MAX_X: u32 = 0xFFFF;
const VMMOUSE_MAX_Y: u32 = 0xFFFF;

// #define VMMOUSE_VENDOR "VMware"
// #define VMMOUSE_NAME   "VMMouse"

const VMMOUSE_VENDOR: &str = "VMware";
const VMMOUSE_NAME: &str = "VMMouse";

// /**
//  * struct vmmouse_data - private data structure for the vmmouse driver
//  *
//  * @abs_dev: "Absolute" device used to report absolute mouse movement.
//  * @phys: Physical path for the absolute device.
//  * @dev_name: Name attribute name for the absolute device.

pub struct vmmouse_data {
    abs_dev: *mut bindings::input_dev,
    phys: [c_types::c_char; 32],
    dev_name: [c_types::c_char; 128],
}

#[no_mangle]
pub fn vmmouse_report_button(
    psmouse: *mut bindings::psmouse,
    abs_dev: *mut bindings::input_dev,
    rel_dev: *mut bindings::input_dev,
    pref_dev: *mut bindings::input_dev,
    code: c_types::c_uint,
    value: c_types::c_int,
) {
    /*      if test_bit(code, abs_dev->key) {
                pref_dev = abs_dev;
          }
          else if test_bit(code, rel_dev->key) {
                pref_dev = rel_dev;
          }

          unsafe {
                input_report_key_wrapper(&mut pref_dev, code, value);
          }
    */
    println!("Button Reported");
}

#[no_mangle]
pub fn vmmouse_report_events() {
    println!("Event Reported");
}

/*#[derive(Debug)]
pub enum vmmouse_supported_hypervisors{
          X86_HYPER_VMWARE,
          X86_HYPER_KVM,
}

impl vmmouse_supported_hypervisors {
    pub fn iterator() -> Iter<'static, vmmouse_supported_hypervisors> {
        static vmmouse_supported_hypervisors: [vmmouse_supported_hypervisors; 2] = [X86_HYPER_VMWARE, X86_HYPER_KVM,];
        vmmouse_supported_hypervisors.iter()
    }
}*/
pub fn vmmouse_check_hypervisor() -> bindings::bool_ {
    /*	  for i in vmmouse_supported_hypervisors::iterator(){
            if i==bindings::x86_hyper_type {
                           return true;
            }
               }
               return false;
    */
    println!("Hypervisor Check");
    return true;
}

#[no_mangle]
pub extern "C" fn vmmouse_detect(
    psmouse: *mut bindings::psmouse,
    set_properties: bindings::bool_,
) -> c_types::c_int {
    let mut response: c_types::c_uint = 0;
    let mut version: c_types::c_uint = 0;
    if !vmmouse_check_hypervisor() {
        unsafe {
            psmouse_dbg_wrapper(psmouse);
        }
        return -6;
    }

    //check if the device is present

    if set_properties {
        println!("PSmouse vendor is {}", VMMOUSE_VENDOR);
        println!("PSmouse name  is {}", VMMOUSE_NAME);
    }

    return 0;
}

#[no_mangle]
pub fn vmmouse_disconect(psmouse: *mut bindings::psmouse) {
    //	vmmouse_diable(&mut psmouse);
/*    unsafe {
        psmouse_reset_wrapper(psmouse);
    }*/
    println!("Disconnect");
}

#[no_mangle]
pub extern "C" fn vmmouse_init(psmouse: *mut bindings::psmouse) -> c_types::c_int {
    println!("VMMOUSE INIT");
    return 0;
}
