#![no_std]
#![feature(alloc)]
#![feature(alloc_prelude)]
#![feature(core_intrinsics)]
#![feature(core_panic)]
#![feature(core_panic_info)]
#![feature(raw)]
#![feature(raw_vec_internals)]
#![feature(unicode_internals)]

extern crate alloc as __alloc;
extern crate core as __core;

use bk_allocator::BkAllocator;

#[global_allocator]
static BK_ALLOCATOR: BkAllocator = BkAllocator;

pub mod kern {
    pub use bk_primitives as raw;
}

mod shim;

pub use shim::*;
