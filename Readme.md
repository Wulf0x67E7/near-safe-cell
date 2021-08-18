# `NearSafeCell`, a more ergonomic `UnsafeCell` wrapper/replacement.
## Rational:
The standard `UnsafeCell`-Api isn't great to work with in some situations.
- You have no way to get a `&T` from an `&UnsafeCell<T>` without using the trivially and always safe `unsafe{ &*cell.get() };`.
- `UnsafeCell::get()` returns a pointer, which has to be unsafely de- and re-referenced to use anyway, so why not just return a reference and make the function itself unsafe?
- If you do actually just need a pointer (e.g. as a map key) you could simple use `self as *const _`. No need for an `UnsafeCell` at all.
- `UnsafeCell::get()` is named confusingly and inconsistently with itself and the rest of the standard library - it should really be called `UnsafeCell::get_mut_ptr()`.
## Usage:
```rust
let mut cell: NearSafeCell<usize> = NearSafeCell::new(42usize);

// Safety: 'interiorly_mutable' is dropped before retrieving 'shared'.
let interiorly_mutable: &mut usize = unsafe{ cell.get_mut_unsafe() }; // &self
drop(interiorly_mutable);

let shared: &usize = cell.get(); // &self
let const_ptr: *const usize = cell.get_ptr(); // &self
let mut_ptr: *mut usize = cell.get_mut_ptr(); // &self
let mutable: &mut usize = cell.get_mut(); // &mut self
let value: usize = cell.unwrap(); // self
```