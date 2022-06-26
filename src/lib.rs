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

const WRITE_ADDRESS: u8 = 0x80;
const READ_ADDRESS: u8 = WRITE_ADDRESS >> 1;

/// Struct for GP2Y0E02B
#[derive(Debug, Copy, Clone)]
pub struct GP2Y0E02B<I2C> {
    com: I2C,
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
    /// Creates a new sensor
    pub fn new(i2c: I2C) -> Result<GP2Y0E02B<I2C>, Error<E>> {
        let chip = GP2Y0E02B { com: i2c };
        Ok(chip)
    }

    /// TODO
    pub fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        self.read_byte(reg as u8)
    }

    /// TODO
    pub fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        self.write_byte(reg as u8, byte)
    }

    /// Reads and returns distance measurement in millimeters
    pub fn read_distance(&mut self) -> Result<f32, E> {
        let high = self.read_byte(0x5E).unwrap() as f32;
        let low = self.read_byte(0x5F).unwrap() as f32;
        let shift = self.read_byte(0x35).unwrap() as u32;

        let v = (((high * 16.0) + low) / 16.0) / (u16::pow(2, shift) as f32);

        Ok(v)
    }

    fn read_byte(&mut self, reg: u8) -> Result<u8, E> {
        let mut data: [u8; 1] = [0];
        self.com.write_read(READ_ADDRESS, &[reg], &mut data)?;

        Ok(data[0])
    }

    fn read_bytes(&mut self, reg: u8) -> Result<[u8; 2], E> {
        let mut data: [u8; 2] = [0, 0];
        self.com.write_read(READ_ADDRESS, &[reg], &mut data)?;

        Ok(data)
    }

    fn write_byte(&mut self, reg: u8, byte: u8) -> Result<(), E> {
        let mut buffer = [0];
        self.com
            .write_read(WRITE_ADDRESS, &[reg, byte], &mut buffer)
    }
}

/// Register Map (Bank0) from <http://www.robot-electronics.co.uk/files/gp2y0e02_03_appl_e.pdf>
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Register {
    /// Hold Bit
    /// 0x00=Hold
    /// 0x01=Device enable normally
    /// Register avoid update during Hold.
    HOLD_BIT = 0x03,

    /// Maximum Emitting Pulse Width
    /// 0x07=320us, 0x06=240us, 0x05=160us, 0x04=80us, 0x03=40us
    MAXIMUM_EMITTING_PULSE_WIDTH = 0x13,

    ///Spot Symmetry Threshold
    SPOT_SYMMETRY_THRESHOLD = 0x1C,

    /// Signal Intensity Threshold
    /// Default is set in each sensor by E-Fuse.
    SIGNAL_INTENSITY_THRESHOLD = 0x2F,

    /// Maximum Spot Size Threshold
    MAXIMUM_SPOT_SIZE_THRESHOLD = 0x33,

    /// Minimum Spot Size Threshold
    /// Default is set in each sensor by E-Fuse.
    MINIMUM_SPOT_SIZE_THRESHOLD = 0x34,

    /// Shift Bit
    /// 0x01=Maximum Display 128cm
    /// 0x02=Maximum Display 64cm
    /// Default to 0x02
    SHIFT_BIT = 0x35,

    /// Median Filter
    /// 0x00= Data number of median calculation 7
    /// 0x10= Data number of median calculation 5
    /// 0x20= Data number of median calculation 9
    /// 0x30= Data number of median calculation 1
    MEDIAN_FILTER = 0x3F,

    /// SRAM Access
    /// 0x10=Access SRAM
    SRAM_ACCESS = 0x4C,

    /// Distance\[11:4\]
    /// Distance Value =(Distance\[11:4\]*16+Distance\[3:0\])/16/2^n
    /// n : Shift Bit (Register 0x35)
    DISTANCE_11_4 = 0x5E,

    /// Distance\[3:0\]
    /// Distance Value =(Distance\[11:4\]*16+Distance\[3:0\])/16/2^n
    /// n : Shift Bit (Register 0x35)
    DISTANCE_3_0 = 0x5F,

    /// AE\[15:8\]
    /// AE=AE\[15:8\]*256 + AE\[7:0\]
    /// Before read out,
    /// it need to write Address(0xEC) = Data(0xFF)
    AE_15_8 = 0x64,

    /// AE\[7:0\]
    /// AE=AE\[15:8\]*256 + AE\[7:0\]
    /// Before read out,
    /// it need to write Address(0xEC) = Data(0xFF)
    AE_7_0 = 0x65,

    /// AG\[7:0\]
    /// AE=AE\[15:8\]*256 + AE\[7:0\]
    /// Before read out,
    /// it need to write Address(0xEC) = Data(0xFF)
    AG_7_0 = 0x67,

    /// Cover Compensation\[5:0\]
    ///  Cover compensation coefficient =
    /// Cover Comp.\[10:6\]*64 + Cover Comp.\[5:0\]
    /// Cover Comp.\[5:0\] is assigned in Reg Field\[7:2\] of
    /// register 0x8D. So, it need to shift 2 bits toward right.
    COVER_COMPENSATION_5_0 = 0x8D,

    /// Cover Compensation\[10:6\]
    ///  Cover compensation coefficient =
    /// Cover Comp.\[10:6\]*64 + Cover Comp.\[5:0\]
    /// Cover Comp.\[5:0\] is assigned in Reg Field\[7:2\] of
    /// register 0x8D. So, it need to shift 2 bits toward right.
    COVER_COMPENSATION_10_6 = 0x8E,

    /// Cover Compensation Enable Bit
    /// 0x02=Enable
    /// 0x03=Disable
    COVER_COMPENSATION_ENABLE_BIT = 0x8F,

    /// Read out Image Sensor Data
    /// 0x00=Disable
    /// 0x10=Low Level (L)
    /// 0x11=Middle Level (M)
    /// 0x12=High Level (H)
    /// Intensity=H*65536 + M*256 + L
    READ_OUT_IMAGE_SENSOR_DATA = 0x90,

    /// Signal Accumulation Number
    /// 0x00=1 time accumulation
    /// 0x01=5 times accumulation
    /// 0x02=30 times accumulation
    /// 0x03=10 times accumulation
    SIGNAL_ACCUMULATION_NUMBER = 0xA8,

    /// Enable Bit (Signal Intensity)
    /// 0x00=enable (Default)
    /// 0x01=disable
    ENABLE_BIT_SIGNAL_INTENSITY = 0xBC,

    /// Enable Bit (Minimum spot size)
    /// 0x00=enable (Default)
    /// 0x01=disable
    ENABLE_BIT_MINIMUM_SPOT_SIZE = 0xBD,

    /// Enable Bit (Maximum spot size)
    /// 0x00=enable
    /// 0x01=disable (Default)
    ENABLE_BIT_MAXIMUM_SPOT_SIZE = 0xBE,

    /// Enable Bit (Spot symmetry)
    /// 0x00=enable (Default)
    /// 0x01=disable
    ENABLE_BIT_SPOT_SYMMETRY = 0xBF,

    /// E-Fuse Target Address
    /// E-Fuse Read Out
    /// E-Fuse Enable Bit
    /// Specify E-Fuse address in the target bank.
    /// Ex. A\[0\]=0x00, B\[10\]=0x0A, C\[63\]=0x3F
    /// 1=load E-Fuse data to Register (Bank3)
    /// 0=Enable, 1=Disable
    E_FUSE_TARGET_ADDRESS_READ_OUT_ENABLE_BIT = 0xC8,

    /// E-Fuse Bit Number
    /// E-Fuse Bank Assign
    /// Assign bit number in the register 0xC9 \[7:4\]
    /// Assign bank select in the register 0xC9 \[3:0\].
    /// 1:BankA, 2:BankB, 3:BankC, 4:BankD, 5:BankE
    E_FUSE_BIT_NUMBER_BANK_ASSIGN = 0xC9,

    /// E-Fuse Program
    /// Enable Bit
    /// 0x00=Disable
    /// 0x01=Enable
    E_FUSE_PROGRAM_ENABLE_BIT = 0xCA,

    /// E-Fuse Program Data
    E_FUSE_PROGRAM_DATA = 0xCD,

    /// Active/Stand-by State Control
    /// 0x00=Active state
    /// 0x01=Stand-by state
    ACTIVE_STAND_BY_STATE_CONTROL = 0xE8,

    /// Clock Select
    /// 0x7F=auto clock
    /// 0xFF=manual clock
    CLOCK_SELECT = 0xEC,

    /// Software Reset
    /// 0x06=software reset
    SOFTWARE_RESET = 0xEE,

    /// Bank Select
    /// 0x00=Bank0
    /// 0x03=Bank3 (E-Fuse)
    BANK_SELECT = 0xEF,

    /// Right Edge Coordinate (C)
    /// Spot Size = C－A (=0xF8\[7:0\]－0xF9\[7:0\])
    /// Spot Symmetry=|(C－A)－2*B|
    /// (=|(0xF8\[7:0\]－0xF9\[7:0\])－2*0xFA\[7:0\])|)
    RIGHT_EDGE_COORDINATE = 0xF8,

    /// Left Edge Coordinate (A)
    /// Spot Size = C－A (=0xF8\[7:0\]－0xF9\[7:0\])
    /// Spot Symmetry=|(C－A)－2*B|
    /// (=|(0xF8\[7:0\]－0xF9\[7:0\])－2*0xFA\[7:0\])|)
    LEFT_EDGE_COORDINATE = 0xF9,

    /// Peak Coordinate (B)
    /// Spot Size = C－A (=0xF8\[7:0\]－0xF9\[7:0\])
    /// Spot Symmetry=|(C－A)－2*B|
    /// (=|(0xF8\[7:0\]－0xF9\[7:0\])－2*0xFA\[7:0\])|)
    PEAK_COORDINATE = 0xFA,
}
