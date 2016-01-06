// XXX Temporary hack to avoid getting spammed with warnings
extern crate shaman;
#[macro_use]
extern crate log;

pub mod gpu;
pub mod cdrom;
pub mod bios;
pub mod memory;
pub mod cpu;
pub mod shared;

mod interrupt;
mod timekeeper;
mod padmemcard;
mod spu;
mod debugger;

/// Version of the rustation library set in Cargo.toml
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Like VERSION but as a `\0`-terminated C string. Useful when you
/// need a static string in C bindings.
pub const VERSION_CSTR: &'static str = concat!(env!("CARGO_PKG_VERSION"), '\0');
