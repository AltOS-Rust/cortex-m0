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

//! This module controls the RCC (Reset and Clock Controller). It handles enabling and disabling
//! clocks, setting clock configurations and the reset flags that are set on a reset.

mod clock_control;
mod config;
mod enable;
mod defs;

use super::Control;
use arm::asm::dsb;
use self::defs::*;
use volatile::Volatile;
pub use self::clock_control::Clock;
pub use self::enable::Peripheral;

/// Returns an instance of the RCC struct so it can be used to modify clock configuration.
pub fn rcc() -> RCC {
    RCC::rcc()
}

/// Reset and Clock Controller
#[derive(Copy, Clone)]
pub struct RCC {
    mem_addr: *const u32,
    cr: clock_control::ClockControl,
    cfgr: config::ConfigControl,
    enr: enable::PeripheralControl,
}

impl Control for RCC {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr)
    }
}

impl RCC {
    fn rcc() -> Self {
        RCC {
            mem_addr: RCC_ADDR,
            cr: clock_control::ClockControl::new(RCC_ADDR),
            cfgr: config::ConfigControl::new(RCC_ADDR),
            enr: enable::PeripheralControl::new(RCC_ADDR),
        }
    }

    /// Enable the specified clock.
    pub fn enable_clock(&mut self, clock: Clock) {
        self.cr.enable_clock(clock);
    }

    /// Disable the specified clock. If the clock cannot be disabled (if it is driving the PLL, for
    /// example) then this method will return false. Otherwise, it returns true.
    pub fn disable_clock(&mut self, clock: Clock) -> bool {
        self.cr.disable_clock(clock)
    }

    /// Return true if the specified clock is enabled.
    pub fn clock_is_on(&self, clock: Clock) -> bool {
        self.cr.clock_is_on(clock)
    }

    /// Return true if the specified clock is ready to be used as the system clock.
    pub fn clock_is_ready(&self, clock: Clock) -> bool {
        self.cr.clock_is_ready(clock)
    }

    /// Return the clock driving the system clock.
    pub fn get_system_clock_source(&self) -> Clock {
        self.cfgr.get_system_clock_source()
    }

    /// Set the system clock source. The system clock can only be run off of the HSI, HSE, PLL, or
    /// HSI48 clocks. If another clock is specified, the kernel will panic.
    pub fn set_system_clock_source(&mut self, clock: Clock) {
        self.cfgr.set_system_clock_source(clock);
        // We need a memory barrier here since the hardware is writing to the system clock bit
        // the barrier ensures that the write to the control register takes effect before we
        // try to access the clock rate
        unsafe { dsb(); }
        clock_control::clock_rate::update_system_clock_rate();
    }

    /// Get the clock driving the PLL.
    pub fn get_pll_source(&self) -> Clock {
        self.cfgr.get_pll_source()
    }

    /// Set the specified clock to drive the PLL. Only the HSI, HSE or HSI48 clocks can drive the
    /// PLL. If another clock is specified, the kernel will panic.
    pub fn set_pll_source(&mut self, clock: Clock) {
        self.cfgr.set_pll_source(clock);
    }

    /// Get the current multiplier for the PLL, the multiplier is in a range of [2..16].
    pub fn get_pll_multiplier(&self) -> u8 {
        self.cfgr.get_pll_multiplier()
    }

    /// Set the PLL multiplier. The specified multiplier MUST be within the range of [2..16]. If it
    /// is outside of that range, the kernel will panic.
    pub fn set_pll_multiplier(&mut self, mul: u8) {
        self.cfgr.set_pll_multiplier(mul);
    }

    /// Get the current prediv factor for the PLL. The factor is in a range of [1..16].
    pub fn get_pll_prediv_factor(&self) -> u8 {
        self.cfgr.get_pll_prediv_factor()
    }

    /// Set the PLL prediv factor, the factor specified MUST be within the range of [1..16]. If it
    /// is outside that range, the kernel will panic.
    pub fn set_pll_prediv_factor(&mut self, factor: u8) {
        self.cfgr.set_pll_prediv_factor(factor);
    }

    /// Get the rate of the current system clock.
    pub fn get_system_clock_rate(&self) -> u32 {
        clock_control::clock_rate::get_system_clock_rate()
    }

    /// Enable a peripheral
    pub fn enable_peripheral(&mut self, peripheral: Peripheral) {
        self.enr.enable_peripheral(peripheral);
    }

    /// Turn off clock for the specified peripheral.
    pub fn disable_peripheral(&mut self, peripheral: Peripheral) {
        self.enr.disable_peripheral(peripheral);
    }

    /// Check if clock is turned on for specified peripheral.
    pub fn peripheral_is_enabled(&self, peripheral: Peripheral) -> bool {
        self.enr.peripheral_is_enabled(peripheral)
    }
}
