#![no_std]

//! Manages a new GP2Y0E02B, SHARP I2C Distance Measuring Sensor, 4-50cm

#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(dead_code)]
#![allow(clippy::uninit_assumed_init, clippy::upper_case_acronyms)]

extern crate cast;
extern crate embedded_hal as ehal;
extern crate generic_array;
extern crate nb;

use cast::u16;
use core::mem::MaybeUninit;
use ehal::blocking::i2c::{Read, Write, WriteRead};
use generic_array::typenum::consts::*;
use generic_array::{ArrayLength, GenericArray};
