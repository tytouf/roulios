#![feature(no_std)]
#![no_std]
#![feature(core_intrinsics)]

//! Volatile registers
//!
//! Newtypes to mark a region of memory as a volatile register.
//!
//! NOTE: This crate is meant to be used only with Cortex-M processors (a.k.a. ARM
//! microcontrollers).
//!
//! All operations should (*) be atomic unless otherwise noted
//!
//! (*) iff the register is aligned, the size is a word or a fraction of it (half-word, byte, etc).
//! Note that accessing some registers may incur in wait states under some conditions (e.g. during
//! a clock source switch), check your reference manual for more information.

//extern crate core;

use core::intrinsics;
use core::marker::Copy;
use core::ops::FnOnce;

/// Read-only register
#[repr(C)]
pub struct RO<T>(T) where T: Copy;

impl<T> RO<T> where T: Copy {
    pub fn get(&self) -> T {
        unsafe {
            intrinsics::volatile_load(&self.0)
        }
    }
}

/// Read-set register
///
/// Clearing any bit of the register has no effect, so you almost never want to "update" (read,
/// modify and write) the register.
#[repr(C)]
pub struct RS<T>(T) where T: Copy;

impl<T> RS<T> where T: Copy {
    pub fn get(&self) -> T {
        unsafe {
            intrinsics::volatile_load(&self.0)
        }
    }

    pub fn set<I>(&self, value: I) where I: Into<T> {
        unsafe {
            intrinsics::volatile_store(&self.0 as *const T as *mut T, value.convert_into())
        }
    }
}

/// Read-write register
#[repr(C)]
pub struct RW<T>(T) where T: Copy;

impl<T> RW<T> where T: Copy {
    pub fn get(&self) -> T {
        unsafe {
            intrinsics::volatile_load(&self.0)
        }
    }

    pub fn set<I>(&self, value: I) where I: Into<T> {
        unsafe {
            intrinsics::volatile_store(&self.0 as *const T as *mut T, value.convert_into())
        }
    }

    /// WARNING: This operation is never atomic.
    pub fn update<F>(&self, f: F) where F: FnOnce(T) -> T {
        self.set(f(self.get()))
    }
}

/// Write-only register
#[repr(C)]
pub struct WO<T>(T) where T: Copy;

impl<T> WO<T> where T: Copy {
    pub fn set<I>(&self, value: I) where I: Into<T> {
        unsafe {
            intrinsics::volatile_store(&self.0 as *const T as *mut T, value.convert_into())
        }
    }
}

/// Temporary trait, until "generic conversions" land in libcore/libstd
pub trait Into<T> {
    fn convert_into(self) -> T;
}

impl<T> Into<T> for T {
    fn convert_into(self) -> T {
        self
    }
}
