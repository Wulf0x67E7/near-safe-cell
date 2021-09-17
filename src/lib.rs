#![no_std]
#![deny(warnings, missing_docs, clippy::all)]
#![doc = include_str!("../Readme.md")]

use core::{
    cell::UnsafeCell,
    default::Default,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// A more ergonomic [`UnsafeCell`] replacement.
pub struct NearSafeCell<T>(UnsafeCell<T>);

impl<T: Default> Default for NearSafeCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> NearSafeCell<T> {
    /// Constructs a new [`NearSafeCell`] wrapping a `T`.
    pub const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }
    /// Consumes this [`NearSafeCell`], returning the wrapped `T`.
    pub fn unwrap(self) -> T {
        self.0.into_inner()
    }
    /// Returns a `&mut T` to the wrapped `T`, bypassing the borrow checker.
    /// # Safety
    /// There exists no other `&T` or `&mut T` to the wrapped `T` currently and until the returned `&mut T` is dropped.
    pub unsafe fn get_mut_unsafe(&self) -> &mut T {
        &mut *self.get_mut_ptr()
    }
    /// Returns a `&T` to the wrapped `T`.
    pub fn get(&self) -> &T {
        // Safety: We have shared access to self and we only return it as a shared reference.
        unsafe { &*self.get_ptr() }
    }
    /// Returns a `&mut T` to the wrapped `T`.
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
    /// Returns a `*const T` to the wrapped `T`.
    pub const fn get_ptr(&self) -> *const T {
        self.0.get()
    }
    /// Returns a `*mut T` to the wrapped `T`.
    pub const fn get_mut_ptr(&self) -> *mut T {
        self.0.get()
    }
}

impl<T> Deref for NearSafeCell<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
impl<T> DerefMut for NearSafeCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: Display> Display for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.get()))
    }
}
impl<T: Debug> Debug for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NearSafeCell").field(self.get()).finish()
    }
}
