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

use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ICSR(u32);

impl ICSR {
    pub fn set_pend_sv(&mut self) {
        self.0 |= ICSR_PENDSVSET;
    }

    pub fn clear_pend_sv(&mut self) {
        self.0 |= ICSR_PENDSVCLR;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icsr_set_pend_sv() {
        let mut icsr = ICSR(0);

        icsr.set_pend_sv();
        assert_eq!(icsr.0, 0b1 << 28);
    }

    #[test]
    fn test_icsr_clear_pend_sv() {
        let mut icsr = ICSR(0);

        icsr.clear_pend_sv();
        assert_eq!(icsr.0, 0b1 << 27);
    }
}
