pub mod alloc {
    pub use __alloc::alloc::*;
    pub use __core::alloc::*;
}

pub mod any {
    pub use __core::any::*;
}
pub mod arch {
    pub use __core::arch::*;
}
pub mod array {
    pub use __core::array::*;
}
pub mod ascii {
    pub use __core::ascii::*;
}
pub mod borrow {
    pub use __alloc::borrow::*;
    pub use __core::borrow::*;
}
pub mod boxed {
    pub use __alloc::boxed::*;
}
pub mod cell {
    pub use __core::cell::*;
}
pub mod char {
    pub use __core::char::*;
}
pub mod clone {
    pub use __core::clone::*;
}
pub mod cmp {
    pub use __core::cmp::*;
}
pub mod collections {
    pub use __alloc::collections::*;
    pub use hashbrown::HashMap;
    pub use hashbrown::HashSet;
}
pub mod convert {
    pub use __core::convert::*;
}
pub mod default {
    pub use __core::default::*;
}
pub mod f32 {
    pub use __core::f32::*;
}
pub mod f64 {
    pub use __core::f64::*;
}
pub mod ffi {
    pub use __core::ffi::*;
}
pub mod fmt {
    pub use __alloc::fmt::*;
    pub use __core::fmt::*;
}
#[cfg(feature = "futures")]
pub mod future {
    pub use __core::future::*;
}
pub mod hash {
    pub use __core::hash::*;
}
pub mod hint {
    pub use __core::hint::*;
}
pub mod i128 {
    pub use __core::i128::*;
}
pub mod i16 {
    pub use __core::i16::*;
}
pub mod i32 {
    pub use __core::i32::*;
}
pub mod i64 {
    pub use __core::i64::*;
}
pub mod i8 {
    pub use __core::i8::*;
}
pub mod intrinsics {
    pub use __core::intrinsics::*;
}
pub mod isize {
    pub use __core::isize::*;
}
pub mod iter {
    pub use __core::iter::*;
}
pub mod marker {
    pub use __core::marker::*;
}
pub mod mem {
    pub use __core::mem::*;
}
pub mod num {
    pub use __core::num::*;
}
pub mod ops {
    pub use __core::ops::*;
}
pub mod option {
    pub use __core::option::*;
}
pub mod panic {
    pub use __core::panic::*;
}
pub mod panicking {
    pub use __core::panicking::*;
}
pub mod pin {
    pub use __core::pin::*;
}
pub mod prelude {
    pub mod v1 {
        pub use __alloc::prelude::v1::*;
        pub use __alloc::{format, vec};
        pub use __core::prelude::v1::*;
    }
}

pub mod ptr {
    pub use __core::ptr::*;
}
pub mod raw {
    pub use __core::raw::*;
}
pub mod raw_vec {
    pub use __alloc::raw_vec::*;
}
pub mod rc {
    pub use __alloc::rc::*;
}
pub mod result {
    pub use __core::result::*;
}
pub mod slice {
    pub use __alloc::slice::*;
    pub use __core::slice::*;
}
pub mod str {
    pub use __alloc::str::*;
    pub use __core::str::*;
}
pub mod string {
    pub use __alloc::string::*;
}
pub mod sync {
    pub use __alloc::sync::*;
    pub use __core::sync::*;
}

#[cfg(feature = "futures")]
pub mod task {
    pub use __core::task::*;
}
pub mod time {
    pub use __core::time::*;
}
pub mod u128 {
    pub use __core::u128::*;
}
pub mod u16 {
    pub use __core::u16::*;
}
pub mod u32 {
    pub use __core::u32::*;
}
pub mod u64 {
    pub use __core::u64::*;
}
pub mod u8 {
    pub use __core::u8::*;
}
pub mod unicode {
    pub use __core::unicode::*;
}
pub mod usize {
    pub use __core::usize::*;
}
pub mod vec {
    pub use __alloc::vec::*;
}
