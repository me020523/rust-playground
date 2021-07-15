extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::*;
use std::env;
use std::ffi::CString;
use std::mem;
use std::path::Path;

const TOKEN_LEN: usize = mem::size_of::<sgx_launch_token_t>();
const ENCLAVE_FILE: &str =
    "/home/shuaibc/code/git/rust-playground/sgx-helloworld/bin/enclave.signed.so";

extern "C" {
    pub fn say_something(
        eid: sgx_enclave_id_t,
        ret: &mut sgx_status_t,
        msg: *const u8,
        msg_len: usize,
    ) -> sgx_status_t;
}

fn main() {
    let lib_path = CString::new(ENCLAVE_FILE).unwrap();
    let mut launch_token: sgx_launch_token_t = [0; TOKEN_LEN];
    let mut launch_token_updated: i32 = 0;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };

    let dir = env::current_dir().unwrap();
    println!("{:?}", dir);

    assert_eq!(Path::new(ENCLAVE_FILE).exists(), true);

    let enclave = sgx_urts::rsgx_create_enclave(
        &lib_path,
        0,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
    .unwrap();

    let msg = "Hello, Rust SGX".as_bytes();

    let mut res = sgx_status_t::SGX_SUCCESS;
    unsafe {
        say_something(enclave, &mut res, msg.as_ptr(), msg.len());
    }
}
