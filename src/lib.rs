#![allow(dead_code)]

extern crate sixel_sys as sixel;

pub mod encoder;
pub mod status;
// Should it be pub?
mod msc;
pub mod optflags;
pub mod pixelformat;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
