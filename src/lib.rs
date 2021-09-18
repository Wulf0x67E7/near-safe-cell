#![no_std]
#![deny(
    warnings,
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic
)]
#![doc = include_str!("../Readme.md")]

use core::{
    cell::UnsafeCell,
    default::Default,
    fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
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

impl<T: Debug> Debug for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NearSafeCell").field(self.get()).finish()
    }
}

impl<T: Display> Display for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: Octal> Octal for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: LowerHex> LowerHex for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: UpperHex> UpperHex for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: Pointer> Pointer for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: Binary> Binary for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: LowerExp> LowerExp for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: UpperExp> UpperExp for NearSafeCell<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn usage() {
        let mut cell = NearSafeCell::<usize>::default();
        assert_eq!(cell.get(), &usize::default());
        cell = NearSafeCell::new(24);

        let _const_ptr = cell.get_ptr();
        let _mut_ptr = cell.get_mut_ptr();

        assert_eq!(cell.deref(), &24);
        assert_eq!(cell.deref_mut(), &mut 24);

        let interiorly_mutable = unsafe { cell.get_mut_unsafe() };
        assert_eq!(interiorly_mutable, &mut 24);
        *interiorly_mutable = 42;
        drop(interiorly_mutable);

        let shared = cell.get();
        let shared2 = cell.get();
        assert_eq!(shared, &42);
        assert_eq!(shared, shared2);

        let mutable = cell.get_mut();
        assert_eq!(mutable, &mut 42);
        *mutable = 242;

        let value = cell.unwrap();
        assert_eq!(value, 242);
    }

    include!("test_utilities.rs");
    #[test]
    fn formatting() {
        let mut buffer = [0u8; 32];
        let cell = NearSafeCell::new(42);
        assert_eq!(
            format(&mut buffer, format_args!("{:?}", cell)).unwrap(),
            "NearSafeCell(42)"
        );
        assert_eq!(format(&mut buffer, format_args!("{}", cell)).unwrap(), "42");
        assert_eq!(
            format(&mut buffer, format_args!("{:o}", cell)).unwrap(),
            "52"
        );
        assert_eq!(
            format(&mut buffer, format_args!("{:x}", cell)).unwrap(),
            "2a"
        );
        assert_eq!(
            format(&mut buffer, format_args!("{:X}", cell)).unwrap(),
            "2A"
        );
        assert_eq!(
            format(&mut buffer, format_args!("{:b}", cell)).unwrap(),
            "101010"
        );
        assert_eq!(
            format(&mut buffer, format_args!("{:e}", cell)).unwrap(),
            "4.2e1"
        );
        assert_eq!(
            format(&mut buffer, format_args!("{:E}", cell)).unwrap(),
            "4.2E1"
        );
        let cell = NearSafeCell::new(42 as *const u8);
        assert_eq!(
            format(&mut buffer, format_args!("{:p}", cell)).unwrap(),
            "0x2a"
        );
    }
}
