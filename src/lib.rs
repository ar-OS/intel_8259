/// Thanks to OSDEV: https://wiki.osdev.org/PIC
///
/// This library implements the IBM PC/AT 8259 PIC architecture,
/// using two PICs chips, which provides a total of 15 interrupts.

extern crate cpuio;

pub mod timer;
pub mod consts;
pub mod pic;
pub mod intel8259;
