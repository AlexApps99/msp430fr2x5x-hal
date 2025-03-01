//! embedded_hal SPI implmentation
use crate::hal::spi::{Mode, Phase, Polarity};
use crate::{
    clock::{Aclk, Smclk},
    gpio::{Alternate1, Pin, Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, P1, P4},
    hal,
    hw_traits::eusci::{EusciSPI, Ucmode, Ucssel, UcxSpiCtw0},
};
use core::marker::PhantomData;
use embedded_hal::spi::FullDuplex;
use msp430fr2355 as pac;
use nb::Error::WouldBlock;

/// Marks a eUSCI capable of SPI communication (in this case, all euscis do)
pub trait EUsciSPIBus: EusciSPI {
    /// Master In Slave Out (refered to as SOMI in datasheet)
    type MISO;
    /// Master Out Slave In (refered to as SIMO in datasheet)
    type MOSI;
    /// Serial Clock
    type SCLK;
    /// Slave Transmit Enable (acts like CS)
    type STE;
}

impl EUsciSPIBus for pac::E_USCI_A0 {
    type MISO = UsciA0MISOPin;
    type MOSI = UsciA0MOSIPin;
    type SCLK = UsciA0SCLKPin;
    type STE = UsciA0STEPin;
}

impl EUsciSPIBus for pac::E_USCI_A1 {
    type MISO = UsciA1MISOPin;
    type MOSI = UsciA1MOSIPin;
    type SCLK = UsciA1SCLKPin;
    type STE = UsciA1STEPin;
}

impl EUsciSPIBus for pac::E_USCI_B0 {
    type MISO = UsciB0MISOPin;
    type MOSI = UsciB0MOSIPin;
    type SCLK = UsciB0SCLKPin;
    type STE = UsciB0STEPin;
}

impl EUsciSPIBus for pac::E_USCI_B1 {
    type MISO = UsciB1MISOPin;
    type MOSI = UsciB1MOSIPin;
    type SCLK = UsciB1SCLKPin;
    type STE = UsciB1STEPin;
}

/// SPI MISO pin for eUSCI A0
pub struct UsciA0MISOPin;
impl<DIR> From<Pin<P1, Pin7, Alternate1<DIR>>> for UsciA0MISOPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin7, Alternate1<DIR>>) -> Self {
        UsciA0MISOPin
    }
}

/// SPI MOSI pin for eUSCI A0
pub struct UsciA0MOSIPin;
impl<DIR> From<Pin<P1, Pin6, Alternate1<DIR>>> for UsciA0MOSIPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin6, Alternate1<DIR>>) -> Self {
        UsciA0MOSIPin
    }
}

/// SPI SCLK pin for eUSCI A0
pub struct UsciA0SCLKPin;
impl<DIR> From<Pin<P1, Pin5, Alternate1<DIR>>> for UsciA0SCLKPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin5, Alternate1<DIR>>) -> Self {
        UsciA0SCLKPin
    }
}

/// SPI STE pin for eUSCI A0
pub struct UsciA0STEPin;
impl<DIR> From<Pin<P1, Pin4, Alternate1<DIR>>> for UsciA0STEPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin4, Alternate1<DIR>>) -> Self {
        UsciA0STEPin
    }
}

/// SPI MISO pin for eUSCI A1
pub struct UsciA1MISOPin;
impl<DIR> From<Pin<P4, Pin3, Alternate1<DIR>>> for UsciA1MISOPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin3, Alternate1<DIR>>) -> Self {
        UsciA1MISOPin
    }
}

/// SPI MOSI pin for eUSCI A1
pub struct UsciA1MOSIPin;
impl<DIR> From<Pin<P4, Pin2, Alternate1<DIR>>> for UsciA1MOSIPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin2, Alternate1<DIR>>) -> Self {
        UsciA1MOSIPin
    }
}

/// SPI SCLK pin for eUSCI A1
pub struct UsciA1SCLKPin;
impl<DIR> From<Pin<P4, Pin1, Alternate1<DIR>>> for UsciA1SCLKPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin1, Alternate1<DIR>>) -> Self {
        UsciA1SCLKPin
    }
}

/// SPI STE pin for eUSCI A1
pub struct UsciA1STEPin;
impl<DIR> From<Pin<P4, Pin0, Alternate1<DIR>>> for UsciA1STEPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin0, Alternate1<DIR>>) -> Self {
        UsciA1STEPin
    }
}

/// SPI MISO pin for eUSCI B0
pub struct UsciB0MISOPin;
impl<DIR> From<Pin<P1, Pin3, Alternate1<DIR>>> for UsciB0MISOPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin3, Alternate1<DIR>>) -> Self {
        UsciB0MISOPin
    }
}

/// SPI MOSI pin for eUSCI B0
pub struct UsciB0MOSIPin;
impl<DIR> From<Pin<P1, Pin2, Alternate1<DIR>>> for UsciB0MOSIPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin2, Alternate1<DIR>>) -> Self {
        UsciB0MOSIPin
    }
}

/// SPI SCLK pin for eUSCI B0
pub struct UsciB0SCLKPin;
impl<DIR> From<Pin<P1, Pin1, Alternate1<DIR>>> for UsciB0SCLKPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin1, Alternate1<DIR>>) -> Self {
        UsciB0SCLKPin
    }
}

/// SPI STE pin for eUSCI B0
pub struct UsciB0STEPin;
impl<DIR> From<Pin<P1, Pin0, Alternate1<DIR>>> for UsciB0STEPin {
    #[inline(always)]
    fn from(_val: Pin<P1, Pin0, Alternate1<DIR>>) -> Self {
        UsciB0STEPin
    }
}

/// SPI MISO pin for eUSCI B1
pub struct UsciB1MISOPin;
impl<DIR> From<Pin<P4, Pin7, Alternate1<DIR>>> for UsciB1MISOPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin7, Alternate1<DIR>>) -> Self {
        UsciB1MISOPin
    }
}

/// SPI MOSI pin for eUSCI B1
pub struct UsciB1MOSIPin;
impl<DIR> From<Pin<P4, Pin6, Alternate1<DIR>>> for UsciB1MOSIPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin6, Alternate1<DIR>>) -> Self {
        UsciB1MOSIPin
    }
}

/// SPI SCLK pin for eUSCI B1
pub struct UsciB1SCLKPin;
impl<DIR> From<Pin<P4, Pin5, Alternate1<DIR>>> for UsciB1SCLKPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin5, Alternate1<DIR>>) -> Self {
        UsciB1SCLKPin
    }
}

/// SPI STE pin for eUSCI B1
pub struct UsciB1STEPin;
impl<DIR> From<Pin<P4, Pin4, Alternate1<DIR>>> for UsciB1STEPin {
    #[inline(always)]
    fn from(_val: Pin<P4, Pin4, Alternate1<DIR>>) -> Self {
        UsciB1STEPin
    }
}

/// Struct used to configure a SPI bus
pub struct SPIBusConfig<USCI: EUsciSPIBus> {
    usci: USCI,
    prescaler: u16,

    // Register configs
    ctlw0: UcxSpiCtw0,
}

impl<USCI: EUsciSPIBus> SPIBusConfig<USCI> {
    /// Create a new configuration for setting up a EUSCI peripheral in SPI mode
    pub fn new(usci: USCI, mode: Mode, msb_first: bool) -> Self {
        let ctlw0 = UcxSpiCtw0 {
            ucckph: match mode.phase {
                Phase::CaptureOnFirstTransition => true,
                Phase::CaptureOnSecondTransition => false,
            },
            ucckpl: match mode.polarity {
                Polarity::IdleLow => false,
                Polarity::IdleHigh => true,
            },
            ucmsb: msb_first,
            uc7bit: false,
            ucmst: true,
            ucsync: true,
            ucstem: true,
            ucswrst: true,
            ucmode: Ucmode::FourPinSPI0,
            ucssel: Ucssel::Uclk,
        };

        SPIBusConfig {
            usci,
            prescaler: 0,
            ctlw0,
        }
    }

    /// Configures this peripheral to use smclk
    #[inline]
    pub fn use_smclk(&mut self, _smclk: &Smclk, clk_divisor: u16) {
        self.ctlw0.ucssel = Ucssel::Smclk;
        self.prescaler = clk_divisor;
    }

    /// Configures this peripheral to use aclk
    #[inline]
    pub fn use_aclk(&mut self, _aclk: &Aclk, clk_divisor: u16) {
        self.ctlw0.ucssel = Ucssel::Aclk;
        self.prescaler = clk_divisor;
    }

    /// Performs hardware configuration and creates an SPI bus
    pub fn spi_pins<
        SO: Into<USCI::MISO>,
        SI: Into<USCI::MOSI>,
        CLK: Into<USCI::SCLK>,
        STE: Into<USCI::STE>,
    >(
        &mut self,
        _miso: SO,
        _mosi: SI,
        _sclk: CLK,
        _cs: STE,
    ) -> SPIPins<USCI> {
        self.configure_hw();
        SPIPins(PhantomData)
    }

    #[inline]
    fn configure_hw(&self) {
        self.usci.ctw0_wr_rst(true);

        self.usci.ctw0_wr(&self.ctlw0);
        self.usci.brw_wr(self.prescaler);
        self.usci.uclisten_set(false);

        self.usci.ctw0_wr_rst(false);

        self.usci.transmit_interrupt_set(false);
        self.usci.receive_interrupt_set(false);
    }
}

/// Represents a group of pins configured for SPI communication
pub struct SPIPins<USCI: EUsciSPIBus>(PhantomData<USCI>);

impl<USCI: EUsciSPIBus> SPIPins<USCI> {
    /// Enable or disable Rx interrupts, which fire when a byte is ready to be read
    #[inline(always)]
    pub fn rx_interrupt_set(&mut self, flag: bool) {
        let usci = unsafe { USCI::steal() };
        usci.receive_interrupt_set(flag);
    }

    /// Enable or disable Tx interrupts, which fire when the transmit buffer is empty
    #[inline(always)]
    pub fn tx_interrupt_set(&mut self, flag: bool) {
        let usci = unsafe { USCI::steal() };
        usci.transmit_interrupt_set(flag);
    }

    /// Writes raw value to Tx buffer with no checks for validity
    #[inline(always)]
    pub fn write_no_check(&mut self, val: u8) {
        let usci = unsafe { USCI::steal() };
        usci.txbuf_wr(val)
    }
}

/// SPI transmit/receive errors
#[derive(Clone, Copy)]
pub enum SPIErr {
    /// Function not implemented
    Unimplemented = 0,
}

impl<USCI: EUsciSPIBus> hal::blocking::spi::Transfer<u8> for SPIPins<USCI> {
    type Error = SPIErr;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        for word in words.iter_mut() {
            nb::block!(self.send(*word))?;
            *word = nb::block!(self.read())?;
        }
        Ok(words)
    }
}

impl<USCI: EUsciSPIBus> hal::blocking::spi::Write<u8> for SPIPins<USCI> {
    type Error = SPIErr;

    fn write(&mut self, words: &[u8]) -> Result<(), SPIErr> {
        for word in words {
            nb::block!(self.send(*word))?;
            nb::block!(self.read())?;
        }
        Ok(())
    }
}

impl<USCI: EUsciSPIBus> FullDuplex<u8> for SPIPins<USCI> {
    type Error = SPIErr;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let usci = unsafe { USCI::steal() };
        if usci.receive_flag() {
            Ok(usci.rxbuf_rd())
        } else {
            Err(WouldBlock)
        }
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let usci = unsafe { USCI::steal() };
        if usci.transmit_flag() {
            usci.txbuf_wr(word);
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}
