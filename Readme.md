[![build](https://github.com/Wulf0x67E7/near-safe-cell/workflows/build/badge.svg)](https://github.com/Wulf0x67E7/near-safe-cell/actions)
[![test](https://github.com/Wulf0x67E7/near-safe-cell/workflows/test/badge.svg)](https://github.com/Wulf0x67E7/near-safe-cell/actions)
[![codecov](https://codecov.io/gh/Wulf0x67E7/near-safe-cell/branch/main/graph/badge.svg?token=MPEI5ZWY57)](https://codecov.io/gh/Wulf0x67E7/near-safe-cell)
[![docs](https://docs.rs/near-safe-cell/badge.svg)](https://docs.rs/near-safe-cell/)
[![crates.io](https://img.shields.io/crates/v/near-safe-cell.svg)](https://crates.io/crates/near-safe-cell)
# `NearSafeCell`, a more ergonomic `UnsafeCell` wrapper/replacement.
## Rational:
The standard `UnsafeCell`-Api isn't great to work with in some situations.
- You have no way to get a `&T` from an `&UnsafeCell<T>` without using the trivially and always safe `unsafe{ &*cell.get() };`.
- `UnsafeCell::get` returns a `&mut T`, which has to be unsafely de- and re-referenced to use anyway, so why not just return a reference and make the function itself unsafe?
- `UnsafeCell::get` is named confusingly and inconsistently with itself and the rest of the standard library - it should really be called `UnsafeCell::get_mut_ptr`.
- If you do actually just need a pointer (e.g. as a map key) you could simple use `&self as *const T`. No need for an `UnsafeCell` at all.

## Usage:
```rust
use near_safe_cell::NearSafeCell;

// Implements 'Default'
let mut cell: NearSafeCell<usize> = NearSafeCell::default();
cell = NearSafeCell::new(24);

// Implements all Formatting traits
assert_eq!(format!("{:b}", cell), "11000");
assert_eq!(format!("{:?}", cell), "NearSafeCell(24)");

// Implements 'Deref' and 'DerefMut'
assert_eq!(&*cell, &24);
assert_eq!(&mut *cell, &mut 24);

// You can still get pointers.
let const_ptr: *const usize = cell.get_ptr(); // &self
let mut_ptr: *mut usize = cell.get_mut_ptr(); // &self

// Safety: 'interiorly_mutable' is dropped before retrieving 'shared'.
let interiorly_mutable: &mut usize = unsafe{ cell.get_mut_unsafe() }; // &self
assert_eq!(interiorly_mutable, &mut 24);
*interiorly_mutable = 42;
drop(interiorly_mutable);

// Multiple shared references.
let shared: &usize = cell.get(); // &self
let shared2: &usize = cell.get(); // &self
assert_eq!(shared, &42);
assert_eq!(shared, shared2);

// One mutable reference.
let mutable: &mut usize = cell.get_mut(); // &mut self
assert_eq!(mutable, &mut 42);
*mutable = 242;

// Consuming the cell to get the value.
let value: usize = cell.unwrap(); // self
assert_eq!(value, 242);
```