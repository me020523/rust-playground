#![crate_name = "enclave"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(rustc_private)]
#![macro_use]

extern crate sgx_tstd as std;
extern crate sgx_types;

use sgx_trts::trts::*;
use sgx_types::*;
use std::io::{self, Write};
use std::println;
use std::slice;
use std::string::String;
use std::vec;
use std::vec::Vec;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };

    let _ = io::stdout().write(str_slice);

    let rust_raw_string = ". This is a ";

    let word: [u8; 4] = [82, 117, 115, 116];
    let word_vec: Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    let mut hello_string = String::from(rust_raw_string);

    for c in word.iter() {
        hello_string.push(*c as char)
    }

    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8").as_str();
    println!("{}", &hello_string);

    let within = rsgx_data_is_within_enclave(&word);
    println!("word is in enclave or not: {}", within);

    sgx_status_t::SGX_SUCCESS
}
