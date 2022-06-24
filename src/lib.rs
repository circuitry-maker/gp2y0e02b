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

use ehal::blocking::i2c::{Read, Write, WriteRead};

const DEFAULT_ADDRESS: u8 = 0x80 >> 1;

/// Struct for GP2Y0E02B
#[derive(Debug, Copy, Clone)]
pub struct GP2Y0E02B<I2C> {
    com: I2C,
    address: u8,
}

/// Defines errors
#[derive(Debug, Copy, Clone)]
pub enum Error<E> {
    /// Underlying bus error
    BusError(E),
    /// Timeout
    Timeout,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::BusError(error)
    }
}

impl<I2C, E> GP2Y0E02B<I2C>
where
    I2C: Write<Error = E> + Read<Error = E> + WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    /// Creates a sensor with default configuration
    pub fn default(i2c: I2C) -> Result<GP2Y0E02B<I2C>, Error<E>> {
        GP2Y0E02B::new(i2c, DEFAULT_ADDRESS)
    }

    /// Creates a sensor with specific configuration
    pub fn new(i2c: I2C, address: u8) -> Result<GP2Y0E02B<I2C>, Error<E>> {
        let chip = GP2Y0E02B { com: i2c, address };
        Ok(chip)
    }

    fn read_register(&mut self) -> Result<u8, E> {
        let mut data: [u8; 1] = [0];
        let _ = self.com.read(0x35, &mut data);

        Ok(data[0])
    }

    fn read_byte(&mut self, reg: u8) -> Result<u8, E> {
        let mut data: [u8; 1] = [0];
        self.com.write_read(self.address, &[reg], &mut data)?;

        Ok(data[0])
    }

    fn read_bytes(&mut self, reg: u8) -> Result<[u8; 2], E> {
        let mut data: [u8; 2] = [0, 0];
        self.com.write_read(self.address, &[reg], &mut data)?;

        Ok(data)
    }

    /// Reads and returns distance measurement in millimeters
    pub fn read_distance(&mut self) -> Result<f32, E> {
        let high = self.read_byte(0x5E).unwrap() as f32;
        let low = self.read_byte(0x5F).unwrap() as f32;
        let shift = self.read_byte(0x35).unwrap() as u32;

        let v = (((high * 16.0) + low) / 16.0) / (u16::pow(2, shift) as f32);

        Ok(v)
    }
}
