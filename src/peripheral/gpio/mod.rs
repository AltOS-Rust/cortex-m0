/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

//! This module provides types for configuring and controlling GPIO connections.

mod port;
mod moder;
mod otyper;
mod bsrr;
mod ospeedr;
mod pupdr;
mod afr;
mod defs;

use core::ops::{Deref, DerefMut};
use volatile::Volatile;
use super::rcc;
use self::defs::*;

pub use self::port::Port;
pub use self::moder::Mode;
pub use self::otyper::Type;
pub use self::ospeedr::Speed;
pub use self::pupdr::Pull;
pub use self::afr::AlternateFunction;

use self::moder::MODER;
use self::otyper::OTYPER;
use self::ospeedr::OSPEEDR;
use self::pupdr::PUPDR;
use self::bsrr::BSRR;
use self::afr::{AFRL, AFRH};

/// An IO group containing up to 16 pins. For some reason, the datasheet shows the memory
/// for groups D and E as reserved, so for now they are left out.
#[derive(Copy, Clone)]
pub enum Group {
    /// GPIO Group A
    A,
    /// GPIO Group B
    B,
    /// GPIO Group C
    C,
    /// GPIO Group F
    F,
}

/// A GPIO contains the base address for a memory mapped GPIO group associated with it.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawGPIO {
    moder: MODER,
    otyper: OTYPER,
    ospeedr: OSPEEDR,
    pupdr: PUPDR,
    idr: u32,
    odr: u32,
    bsrr: BSRR,
    lckr: u32,
    afrl: AFRL,
    afrh: AFRH,
    brr: u32,
}

/// Creates struct for accessing the GPIO groups.
///
/// Has a RawGPIO data member in order to access each register for the
/// GPIO peripheral.
#[derive(Copy, Clone, Debug)]
pub struct GPIO(Volatile<RawGPIO>);

impl GPIO {
    fn group(group: Group) -> GPIO {
        match group {
            Group::A => GPIO::new(GROUPA_ADDR),
            Group::B => GPIO::new(GROUPB_ADDR),
            Group::C => GPIO::new(GROUPC_ADDR),
            Group::F => GPIO::new(GROUPF_ADDR),
        }
    }

    fn new(mem_addr: *const u32) -> GPIO {
        unsafe {
            GPIO(Volatile::new(mem_addr as *const _))
        }
    }

    /// Wrapper for enabling a GPIO group.
    pub fn enable(group: Group) {
        RawGPIO::enable(group);
    }
}

impl Deref for GPIO {
    type Target = RawGPIO;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for GPIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawGPIO {
    /// Enable a GPIO group. This must be done before setting any pins within a group.
    ///
    /// Example Usage:
    /// ```
    ///   GPIO::enable(Group::B); // Enable IO group B (LED is pb3)
    /// ```
    pub fn enable(group: Group) {
        let mut rcc = rcc::rcc();

        // Get the register bit that should be set to enable this group
        let io_group = match group {
            Group::A => rcc::Peripheral::GPIOA,
            Group::B => rcc::Peripheral::GPIOB,
            Group::C => rcc::Peripheral::GPIOC,
            Group::F => rcc::Peripheral::GPIOF,
        };
        rcc.enable_peripheral(io_group);
    }

    /// Set the mode for the specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_mode(&mut self, mode: Mode, port: u8) {
        self.moder.set_mode(mode, port);
    }

    /// Gets the mode for the specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn get_mode(&self, port: u8) -> Mode {
        self.moder.get_mode(port)
    }

    /// Sets the type for the specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_type(&mut self, p_type: Type, port: u8) {
        self.otyper.set_type(p_type, port);
    }

    /// Gets the type for the specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn get_type(&self, port: u8) -> Type {
        self.otyper.get_type(port)
    }

    /// Turns on GPIO pin at specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_bit(&mut self, port: u8) {
        self.bsrr.set(port);
    }

    /// Resets bit at specified port.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn reset_bit(&mut self, port: u8) {
        self.bsrr.reset(port);
    }

    /// Sets the port speed for the GPIO pin.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_speed(&mut self, speed: Speed, port: u8) {
        self.ospeedr.set_speed(speed, port);
    }

    /// Get the current port speed.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn get_speed(&self, port: u8) -> Speed {
        self.ospeedr.get_speed(port)
    }

    /// Set behavior of GPIO pin when it is not asserted.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_pull(&mut self, pull: Pull, port: u8) {
        self.pupdr.set_pull(pull, port);
    }

    /// Get currently defined behavior of GPIO pin when not asserted.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn get_pull(&self, port: u8) -> Pull {
        self.pupdr.get_pull(port)
    }

    /// Set the GPIO function type.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn set_function(&mut self, function: AlternateFunction, port: u8) {
        match port {
            0...7 => self.afrl.set_function(function, port),
            8...15 => self.afrh.set_function(function, port),
            _ => panic!("AFRL/AFRH::set_function - specified port must be between [0..15]!"),
        }
    }

    /// Get the GPIO function type.
    ///
    /// # Panics
    ///
    /// Port must be a value between [0..15] or the kernel will panic.
    fn get_function(&self, port: u8) -> AlternateFunction {
        match port {
            0...7 => self.afrl.get_function(port),
            8...15 => self.afrh.get_function(port),
            _ => panic!("AFRL/AFRH::set_function - specified port must be between [0..15]!"),
        }
    }
}
