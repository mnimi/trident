/*!
Trident console library.
*/

#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
#![feature(decl_macro)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

extern crate t_alloc as alloc;

#[macro_use]
extern crate lazy_static;
extern crate volatile;

pub mod colour;
pub mod con;
pub mod text;
pub mod write;
mod panic;

pub use self::colour::{Colour, ColourCode};
pub use self::write::{print, println};
