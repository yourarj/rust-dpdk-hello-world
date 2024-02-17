//! Rust hello world example
//! Make use of https://github.com/ANLAB-KAIST/rust-dpdk api
//!
//! This is complete unsafe
//!
//! In production code we should be making safe shim layer and rest of
//! RUST code should be safe.

use rust_dpdk::{
    rte_eal_cleanup, rte_eal_mp_wait_lcore, rte_eal_remote_launch, rte_get_next_lcore,
    rte_lcore_id, RTE_MAX_LCORE,
};
use std::env;
use std::os::raw::c_void;
use std::ptr;

unsafe extern "C" fn lcore_hello(_: *mut c_void) -> i32 {
    println!("hello from core {}", unsafe { rte_lcore_id() });
    0
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut a = core::ptr::addr_of_mut!(args) as *mut i8;
    unsafe { rust_dpdk::rte_eal_init(args.len() as i32, core::ptr::addr_of_mut!(a)) };

    let mut i = unsafe { rte_get_next_lcore(0, 1, 0) };

    while i < RTE_MAX_LCORE {
        unsafe { rte_eal_remote_launch(Some(lcore_hello), ptr::null_mut() as *mut c_void, i) };
        i = unsafe { rte_get_next_lcore(i, 1, 0) }
    }

    let mut b = 1;
    unsafe { lcore_hello(core::ptr::addr_of_mut!(b) as *mut c_void) };

    unsafe { rte_eal_mp_wait_lcore() };

    unsafe { rte_eal_cleanup() };
}
