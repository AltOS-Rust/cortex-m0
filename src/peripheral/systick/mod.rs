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

//! This module configures the system tick behavior.

mod control_status;
mod reload_value;
mod current_value;
mod defs;

use core::ops::{Deref, DerefMut};
use volatile::Volatile;
use self::defs::*;
use self::control_status::CSR;
use self::current_value::CVR;
use self::reload_value::RVR;

pub use self::control_status::ClockSource;

/// Returns an instance of the SysTick to modify system tick behavior.
pub fn systick() -> SysTick {
    SysTick::systick()
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[doc(hidden)]
pub struct RawSysTick {
    csr: CSR,
    rvr: RVR,
    cvr: CVR,
    calib: u32,
}

/// Control system tick behavior.
#[derive(Copy, Clone, Debug)]
pub struct SysTick(Volatile<RawSysTick>);

impl SysTick {
    fn systick() -> Self {
        unsafe {
            SysTick(Volatile::new(SYSTICK_ADDR as *const _))
        }
    }
}

impl Deref for SysTick {
    type Target = RawSysTick;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for SysTick {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}

impl RawSysTick {
    /// Enable system tick counter.
    ///
    /// When enabled, counter value will decrement after each clock cycle
    /// until it reaches zero, at which point it will be reset to the reload value.
    /// If SysTick interrupt is enabled, when the counter reaches zero a
    /// SysTick interrupt will be generated.
    pub fn enable_counter(&mut self) {
        self.csr.set_enable(true);
    }

    /// Disable system tick counter.
    pub fn disable_counter(&mut self) {
        self.csr.set_enable(false);
    }

    /// Enable SysTick interrupt.
    pub fn enable_interrupts(&mut self) {
        self.csr.set_interrupt(true);
    }

    /// Disable SysTick interrupt.
    pub fn disable_interrupts(&mut self) {
        self.csr.set_interrupt(false);
    }

    /// Use the system clock for the counter.
    pub fn use_processor_clock(&mut self) {
        self.csr.set_source(control_status::ClockSource::Processor);
    }

    /// Use an alternate clock for the counter.
    pub fn use_reference_clock(&mut self) {
        self.csr.set_source(control_status::ClockSource::Reference);
    }

    /// Check if counter reached zero.
    pub fn did_underflow(&self) -> bool {
        self.csr.did_underflow()
    }

    /// Get the reload value for the counter.
    pub fn get_reload_value(&self) -> u32 {
        self.rvr.get_reload_value()
    }

    /// Set the reload value for the counter.
    pub fn set_reload_value(&mut self, value: u32) {
        self.rvr.set_reload_value(value);
    }

    /// Get the current value for the counter.
    pub fn get_current_value(&self) -> u32 {
        self.cvr.get_current_value()
    }

    /// Clear the counter value.
    pub fn clear_current_value(&mut self) {
        self.cvr.clear_current_value();
    }
}
