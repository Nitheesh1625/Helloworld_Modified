extern crate bindgen;
extern crate cc;
extern crate shlex;

use std::env;
use std::path::PathBuf;
use std::process::Command;

const INCLUDED_TYPES: &[&str] = &["input_dev","psmouse","psmouse_ret_t","x86_hypervisor_type","bool"];

const INCLUDED_FUNCTIONS: &[&str] = &[
    "test_bit",
    "psmouse_err",
    "psmouse_warn",
    "psmouse_dbg",
    "psmouse_reset",
    "input_report_key",
    "input_report_rel",
    "input_report_abs",
    "input_sync",   
    "input_unregister_device",
    "input_register_device",
    "input_free_device",
    "input_allocate_device",
    "input_set_capability",
    "kfree",
    "kzalloc",
    "snprintf",
    "ARRAY_SIZE"
];

const INCLUDED_VARS: &[&str] = &[
    "__asm__",
    "ENXIO",
    "x86_hyper_type",
    "GFP_KERNEL",
    "ENOMEM",
    "EV_KEY",
    "BTN_LEFT",
    "BTN_RIGHT",
    "BTN_MIDDLE",
    "EN_ABS",
    "ABS_X",
    "ABS_Y",
    "EV_REL",
    "REL_WHEEL",
    "ENOSYS"
];

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("Target={}", target);
    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .no_copy(".*")
        .derive_default(true)
        .rustfmt_bindings(true)
        .clang_arg(format!("--target={}", target));

    let output = String::from_utf8(
        Command::new("make")
            .arg("-C")
            .arg("kernel-cflags-finder")
            .arg("-s")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    Command::new("make")
        .arg("-C")
        .arg("kernel-cflags-finder")
        .arg("clean");

    println!("get output:{}", output);
   
    for arg in shlex::split(&output).unwrap() {
        builder = builder.clang_arg(arg.to_string());
    }

    println!("cargo:rerun-if-changed=src/bindgen_helper.h");
    builder = builder.header("src/bindgen_helper.h");

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.whitelist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
