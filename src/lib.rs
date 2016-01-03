// XXX Temporary hack to avoid getting spammed with warnings
#![allow(unused)]

extern crate shaman;

mod cpu;
mod memory;
mod gpu;
mod timekeeper;
mod debugger;
mod cdrom;
mod padmemcard;
mod spu;
pub mod bios;

pub use cdrom::disc::{Disc, Region};

/// Version of the rustation library set in Cargo.toml
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Like VERSION but as a `\0`-terminated C string. Useful when you
/// need a static string in C bindings.
pub const VERSION_CSTR: &'static str = concat!(env!("CARGO_PKG_VERSION"), '\0');

/// The are a few hardware differences between PAL and NTSC consoles,
/// for instance runs slightly slower on PAL consoles.
#[derive(Clone,Copy)]
pub enum HardwareType {
    Ntsc,
    Pal,
}
