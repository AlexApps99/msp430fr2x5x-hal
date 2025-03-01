//! Embedded hal delay implementation
use crate::hal::blocking::delay::DelayMs;
use msp430::asm;

/// Delay provider struct
pub struct Delay {
    nops_per_ms: u16,
}

impl Delay {
    /// Creates a new Delay provider for a given clock frequency
    pub fn new(freq: u32) -> Self {
        // ~21 nops needed per 2^20 MHz to delay 1 ms
        let nops: u32 = 210 * (freq >> 20);
        Delay {
            nops_per_ms: (nops as u16),
        }
    }
}

impl DelayMs<u8> for Delay {
    #[inline]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u16);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        for _ in 0..ms {
            for _ in 0..self.nops_per_ms {
                asm::nop();
            }
        }
    }
}
