//! This is a platform agnostic Rust driver for the 24x series serial EEPROM,
//! based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read a single byte from a memory address. See [`read_byte()`].
//! - Read a byte array starting on a memory address. See: [`read_data()`].
//! - Read the current memory address (please read notes). See: [`read_current_address()`].
//! - Write a byte to a memory address. See: [`write_byte()`].
//! - Write a byte array (up to a memory page) to a memory address. See: [`write_page()`].
//!
//! [`read_byte()`]: struct.Eeprom24x.html#method.read_byte
//! [`read_data()`]: struct.Eeprom24x.html#method.read_data
//! [`read_current_address()`]: struct.Eeprom24x.html#method.read_current_address
//! [`write_byte()`]: struct.Eeprom24x.html#method.write_byte
//! [`write_page()`]: struct.Eeprom24x.html#method.write_page
//!
//! Can be used at least with the devices listed below.
//!
//! ## The devices
//!
//! These devices provides a number of bits of serial electrically erasable and
//! programmable read only memory (EEPROM) organized as a number of words of 8 bits
//! each. The devices' cascadable feature allows up to 8 devices to share a common
//! 2-wire bus. The devices are optimized for use in many industrial and commercial
//! applications where low power and low voltage operation are essential.
//!
//! | Device | Memory bits | 8-bit words | Page size | Datasheet  |
//! |-------:|------------:|------------:|----------:|:-----------|
//! |  24x00 |    128 bits |          16 |       N/A | [24C00]    |
//! |  24x01 |      1 Kbit |         128 |   8 bytes | [AT24C01]  |
//! |  24x02 |      2 Kbit |         256 |   8 bytes | [AT24C02]  |
//! |  24x04 |      4 Kbit |         512 |  16 bytes | [AT24C04]  |
//! |  24x08 |      8 Kbit |       1,024 |  16 bytes | [AT24C08]  |
//! |  24x16 |     16 Kbit |       2,048 |  16 bytes | [AT24C16]  |
//! |  24x32 |     32 Kbit |       4,096 |  32 bytes | [AT24C32]  |
//! |  24x64 |     64 Kbit |       8,192 |  32 bytes | [AT24C64]  |
//! | 24x128 |    128 Kbit |      16,384 |  64 bytes | [AT24C128] |
//! | 24x256 |    256 Kbit |      32,768 |  64 bytes | [AT24C256] |
//! | 24x512 |    512 Kbit |      65,536 | 128 bytes | [AT24C512] |
//! | 24xM01 |      1 Mbit |     131,072 | 256 bytes | [AT24CM01] |
//! | 24xM02 |      2 Mbit |     262,144 | 256 bytes | [AT24CM02] |
//!
//! [24C00]: http://ww1.microchip.com/downloads/en/DeviceDoc/24AA00-24LC00-24C00-Data-Sheet-20001178J.pdf
//! [AT24C01]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
//! [AT24C02]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
//! [AT24C04]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8896E-SEEPROM-AT24C04D-Datasheet.pdf
//! [AT24C08]: http://ww1.microchip.com/downloads/en/DeviceDoc/AT24C08D-I2C-Compatible-2-Wire-Serial-EEPROM-20006022A.pdf
//! [AT24C16]: http://ww1.microchip.com/downloads/en/DeviceDoc/20005858A.pdf
//! [AT24C32]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C64]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C128]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf
//! [AT24C256]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf
//! [AT24C512]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf
//! [AT24CM01]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8812-SEEPROM-AT24CM01-Datasheet.pdf
//! [AT24CM02]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8828-SEEPROM-AT24CM02-Datasheet.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To create a new instance you can use the `new_<device>` methods.
//! There are many compatible vendors so the method has a somewhat generic name.
//! For example, if you are using an AT24C32, you can create a device by calling
//! `Eeprom24x::new_24x32(...)`.
//! Please refer to the [device table](#the-devices) above for more examples.
//!
//! ### Instantiating with the default address
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! // using the AT24C256
//! let mut eeprom = Eeprom24x::new_24x256(dev, address);
//! # }
//! ```
//!
//! ### Providing an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut eeprom = Eeprom24x::new_24x256(dev, address);
//! # }
//! ```
//!
//! ### Writting and reading a byte
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = Eeprom24x::new_24x256(dev, SlaveAddr::default());
//! let address = 0x1234;
//! let data = 0xAB;
//! eeprom.write_byte(address, data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! let retrieved_data = eeprom.read_byte(address);
//! # }
//! ```
//!
//! ### Writting a page
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = Eeprom24x::new_24x256(dev, SlaveAddr::default());
//! let address = 0x1234;
//! let data = [0xAB; 64];
//! eeprom.write_page(address, &data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! # }
//! ```

#![deny(missing_docs, unsafe_code, warnings)]
#![no_std]

extern crate embedded_hal as hal;

use core::marker::PhantomData;
use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Too much data passed for a write
    TooMuchData,
    /// Memory address is out of range
    InvalidAddr,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    /// Get slave address as u8
    fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => 0b101_0000,
            SlaveAddr::Alternative(a2, a1, a0) =>
                SlaveAddr::default().addr() |
                ((a2 as u8) << 2)           |
                ((a1 as u8) << 1)           |
                  a0 as u8
        }
    }

    fn devaddr(self, address: u32, devmask: u32, shift: u32) -> u8 {
        let hi: u8 = ((address >> shift) & devmask) as u8;
        self.addr() | hi
    }
}

/// Memory address size markers
pub mod addr_size {
    /// AT24x00, AT24x01, AT24x02, AT24x04, AT24x08, AT24x16:
    /// 1-byte memory address
    pub struct One(());
    /// AT24x32, AT24x64, AT24x128, AT24x256, AT24x512, AT24xM01, AT24xM02:
    /// 2-bytes memory address
    pub struct Two(());
}


/// Page size markers
pub mod page_size {
    /// No page write supported. e.g. for AT24x00
    pub struct No(());
    /// 8-byte pages. e.g. for AT24x01, AT24x02
    pub struct B8(());
    /// 16-byte pages. e.g. for AT24x04, AT24x08, AT24x16
    pub struct B16(());
    /// 32-byte pages. e.g. for AT24x32, AT24x64
    pub struct B32(());
    /// 64-byte pages. e.g. for AT24x128, AT24x256
    pub struct B64(());
    /// 128-byte pages. e.g. for AT24x512
    pub struct B128(());
    /// 256-byte pages. e.g. for AT24xM01, AT24xM02
    pub struct B256(());
}

/// EEPROM24X driver
#[derive(Debug, Default)]
pub struct Eeprom24x<I2C, PS, AS> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: SlaveAddr,
    /// FIXME: high mem addr bits: part of dev addr
    devmask: u32,
    /// Page size marker type.
    _ps: PhantomData<PS>,
    /// Address size marker type.
    _as: PhantomData<AS>,
}

/// Common methods for 1-byte memory addrs
impl<I2C, PS, AS> Eeprom24x<I2C, PS, AS>
{
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, PS> Eeprom24x<I2C, PS, addr_size::One>
{
    fn invalid_address(&self, address: u32) -> bool {
        address & !((self.devmask << 8) | 0xff) > 0
    }
}

impl<I2C, PS> Eeprom24x<I2C, PS, addr_size::Two>
{
    fn invalid_address(&self, address: u32) -> bool {
        address & !((self.devmask << 16) | 0xffff) > 0
    }
}

/// Common methods for 1-byte memory addrs
impl<I2C, E, PS> Eeprom24x<I2C, PS, addr_size::One>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_byte(&mut self, address: u32, data: u8) -> Result<(), Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 8);
        let payload = [(address & 0xff) as u8, data];
        self.i2c
            .write(devaddr, &payload)
            .map_err(Error::I2C)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: u32) -> Result<u8, Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 8);
        let memaddr = [(address & 0xff) as u8];
        let mut data = [0; 1];
        self.i2c
            .write_read(devaddr, &memaddr, &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Read starting in an address as many bytes as necessary to fill the data array provided.
    pub fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 8);
        let memaddr = [(address & 0xff) as u8];
        self.i2c
            .write_read(devaddr, &memaddr, data)
            .map_err(Error::I2C)
    }
}

/// Common methods for 2-bytes memory addrs
impl<I2C, E, PS> Eeprom24x<I2C, PS, addr_size::Two >
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_byte(&mut self, address: u32, data: u8) -> Result<(), Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 16);
        let payload = [((address >> 8) & 0xff) as u8 , (address & 0xff) as u8, data];
        self.i2c
            .write(devaddr, &payload)
            .map_err(Error::I2C)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: u32) -> Result<u8, Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 16);
        let memaddr = [((address >> 8) & 0xff) as u8 , (address & 0xff) as u8];
        let mut data = [0; 1];
        self.i2c
            .write_read(devaddr, &memaddr, &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Read starting in an address as many bytes as necessary to fill the data array provided.
    pub fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), Error<E>> {
        if self.invalid_address(address) {
            return Err(Error::InvalidAddr);
        }

        let devaddr = self.address.devaddr(address, self.devmask, 16);
        let memaddr = [((address >> 8) & 0xff) as u8 , (address & 0xff) as u8];
        self.i2c
            .write_read(devaddr, &memaddr, data)
            .map_err(Error::I2C)
    }
}

/// Specialization for platforms which implement `embedded_hal::blocking::i2c::Read`
impl<I2C, E, PS, AS> Eeprom24x<I2C, PS, AS>
where
    I2C: hal::blocking::i2c::Read<Error = E>,
{
    /// Read the contents of the last address accessed during the last read
    /// or write operation, _incremented by one_.
    ///
    /// Note: This may not be available on your platform.
    pub fn read_current_address(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .read(self.address.addr(), &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}

/// Specialization for devices without page access (e.g. 24C00)
impl<I2C, E> Eeprom24x<I2C, page_size::No, addr_size::One>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of a 24x00 device (e.g. 24C00)
    pub fn new_24x00(i2c: I2C, address: SlaveAddr) -> Self {
        Eeprom24x {
            i2c,
            address,
            devmask: 0x0,
            _ps: PhantomData,
            _as: PhantomData,
        }
    }
}

macro_rules! impl_create {
    ( $dev:expr, $part:expr, $mask:expr, $create:ident ) => {
        impl_create!{
            @gen [$create, $mask, concat!("Create a new instance of a ", $dev, " device (e.g. ", $part, ")")]
        }
    };

    (@gen [$create:ident, $mask:expr, $doc:expr] ) => {
        #[doc = $doc]
        pub fn $create(i2c: I2C, address: SlaveAddr) -> Self {
            Self::new(i2c, address, $mask)
        }
    };
}

macro_rules! impl_for_page_size {
    ( $AS:ident, $PS:ident, $page_size:expr, $( [ $dev:expr, $part:expr, $mask:expr, $create:ident ] ),* ) => {
        impl_for_page_size!{
            @gen [$AS, $PS, $page_size,
            concat!("Specialization for devices with a page size of ", stringify!($page_size), " bytes."),
            concat!("Create generic instance for devices with a page size of ", stringify!($page_size), " bytes."),
            $( [ $dev, $part, $mask, $create ] ),* ]
        }
    };

    (@gen [$AS:ident, $PS:ident, $page_size:expr, $doc_impl:expr, $doc_new:expr, $( [ $dev:expr, $part:expr, $mask:expr, $create:ident ] ),* ] ) => {
        #[doc = $doc_impl]
        impl<I2C, E> Eeprom24x<I2C, page_size::$PS, addr_size::$AS>
        where
            I2C: Write<Error = E>
        {
            $(
                impl_create!($dev, $part, $mask, $create);
            )*

            #[doc = $doc_new]
            fn new(i2c: I2C, address: SlaveAddr, devmask: u32) -> Self {
                Eeprom24x {
                    i2c,
                    address,
                    devmask,
                    _ps: PhantomData,
                    _as: PhantomData,
                }
            }
        }

        impl<I2C, E> Eeprom24x<I2C, page_size::$PS, addr_size::One>
        where
            I2C: Write<Error = E>
        {
            /// Write up to a page starting in an address.
            ///
            /// The maximum amount of data that can be written depends on the page
            /// size of the device. If too much data is passed, the error
            /// `Error::TooMuchData` will be returned.
            ///
            /// After writing a byte, the EEPROM enters an internally-timed write cycle
            /// to the nonvolatile memory.
            /// During this time all inputs are disabled and the EEPROM will not
            /// respond until the write is complete.
            pub fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), Error<E>> {
                if self.invalid_address(address) {
                    return Err(Error::InvalidAddr);
                }

                if data.len() == 0 {
                    return Ok(());
                }

                if data.len() > $page_size {
                    // This would actually be supported by the EEPROM but
                    // the data would be overwritten
                    return Err(Error::TooMuchData);
                }

                let devaddr = self.address.devaddr(address, self.devmask, 8);
                let mut payload: [u8; 1 + $page_size] = [0; 1 + $page_size];
                payload[0] = (address & 0xff) as u8;
                payload[1..= data.len()].copy_from_slice(&data);
                self.i2c
                    .write(devaddr, &payload[..= data.len()])
                    .map_err(Error::I2C)
            }
        }

        impl<I2C, E> Eeprom24x<I2C, page_size::$PS, addr_size::Two>
        where
            I2C: Write<Error = E>
        {
            /// Write up to a page starting in an address.
            ///
            /// The maximum amount of data that can be written depends on the page
            /// size of the device. If too much data is passed, the error
            /// `Error::TooMuchData` will be returned.
            ///
            /// After writing a byte, the EEPROM enters an internally-timed write cycle
            /// to the nonvolatile memory.
            /// During this time all inputs are disabled and the EEPROM will not
            /// respond until the write is complete.
            pub fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), Error<E>> {
                if self.invalid_address(address) {
                    return Err(Error::InvalidAddr);
                }

                if data.len() == 0 {
                    return Ok(());
                }

                if data.len() > $page_size {
                    // This would actually be supported by the EEPROM but
                    // the data would be overwritten
                    return Err(Error::TooMuchData);
                }

                let devaddr = self.address.devaddr(address, self.devmask, 16);
                let mut payload: [u8; 2 + $page_size] = [0; 2 + $page_size];
                payload[0] = ((address >> 8) & 0xff) as u8;
                payload[1] = (address & 0xff) as u8;
                payload[2..=(1 + data.len())].copy_from_slice(&data);
                self.i2c
                    .write(devaddr, &payload[..=(1 + data.len())])
                    .map_err(Error::I2C)
            }
        }

    };
}

impl_for_page_size!(
    One,
    B8,
    8,
    ["24x01", "AT24C01", 0b000, new_24x01],
    ["24x02", "AT24C02", 0b000, new_24x02]
);
impl_for_page_size!(
    One,
    B16,
    16,
    ["24x04", "AT24C04", 0b001, new_24x04],
    ["24x08", "AT24C08", 0b011, new_24x08],
    ["24x16", "AT24C16", 0b111, new_24x16]
);
impl_for_page_size!(
    Two,
    B32,
    32,
    ["24x32", "AT24C32", 0b000, new_24x32],
    ["24x64", "AT24C64", 0b000, new_24x64]
);
impl_for_page_size!(
    Two,
    B64,
    64,
    ["24x128", "AT24C128", 0b000, new_24x128],
    ["24x256", "AT24C256", 0b000, new_24x256]
);
impl_for_page_size!(
    Two,
    B128,
    128,
    ["24x512", "AT24C512", 0b000, new_24x512]
);
impl_for_page_size!(
    Two,
    B256,
    256,
    ["24xM01", "AT24CM01", 0b001, new_24xm01],
    ["24xM02", "AT24CM02", 0b011, new_24xm02]
);

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;
    use super::*;

    #[test]
    fn default_address_is_correct() {
        assert_eq!(0b101_0000, SlaveAddr::default().addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b101_0000, SlaveAddr::Alternative(false, false, false).addr());
        assert_eq!(0b101_0001, SlaveAddr::Alternative(false, false,  true).addr());
        assert_eq!(0b101_0010, SlaveAddr::Alternative(false,  true, false).addr());
        assert_eq!(0b101_0100, SlaveAddr::Alternative( true, false, false).addr());
        assert_eq!(0b101_0111, SlaveAddr::Alternative( true,  true,  true).addr());
    }
}

