pub use crate::batch_gpio::*;
use crate::bits::BitsExt;
use crate::hw_traits::gpio::{GpioPeriph, IntrPeriph};
use core::marker::PhantomData;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
use msp430fr2355 as pac;
use pac::{P1, P2, P3, P4, P5, P6};

/// Trait that encompasses all `Pinx` types for specifying a pin number.
pub trait PinNum {
    /// Pin number
    fn pin() -> u8;
}

/// Trait that encompasses all `Portx` types for specifying GPIO port
pub trait PortNum {
    /// PAC peripheral type associated with the port
    type Port: GpioPeriph;
}

/// Trait implemented on PAC GPIO types to map the PAC type to its respective port number type
pub trait GpioPort {
    /// Port number
    type PortNum: PortNum;
}

/// Pin number 0
pub struct Pin0;
impl PinNum for Pin0 {
    fn pin() -> u8 {
        0
    }
}

/// Pin number 1
pub struct Pin1;
impl PinNum for Pin1 {
    fn pin() -> u8 {
        1
    }
}

/// Pin number 2
pub struct Pin2;
impl PinNum for Pin2 {
    fn pin() -> u8 {
        2
    }
}

/// Pin number 3
pub struct Pin3;
impl PinNum for Pin3 {
    fn pin() -> u8 {
        3
    }
}

/// Pin number 4
pub struct Pin4;
impl PinNum for Pin4 {
    fn pin() -> u8 {
        4
    }
}

/// Pin number 5
pub struct Pin5;
impl PinNum for Pin5 {
    fn pin() -> u8 {
        5
    }
}

/// Pin number 6
pub struct Pin6;
impl PinNum for Pin6 {
    fn pin() -> u8 {
        6
    }
}

/// Pin number 7
pub struct Pin7;
impl PinNum for Pin7 {
    fn pin() -> u8 {
        7
    }
}

/// Port P1
pub struct Port1;
impl PortNum for Port1 {
    type Port = pac::p1::RegisterBlock;
}
impl GpioPort for P1 {
    type PortNum = Port1;
}

/// Port P2
pub struct Port2;
impl PortNum for Port2 {
    type Port = pac::p2::RegisterBlock;
}
impl GpioPort for P2 {
    type PortNum = Port2;
}

/// Port P3
pub struct Port3;
impl PortNum for Port3 {
    type Port = pac::p3::RegisterBlock;
}
impl GpioPort for P3 {
    type PortNum = Port3;
}

/// Port P4
pub struct Port4;
impl PortNum for Port4 {
    type Port = pac::p4::RegisterBlock;
}
impl GpioPort for P4 {
    type PortNum = Port4;
}

/// Port P5
pub struct Port5;
impl PortNum for Port5 {
    type Port = pac::p5::RegisterBlock;
}
impl GpioPort for P5 {
    type PortNum = Port5;
}

/// Port P6
pub struct Port6;
impl PortNum for Port6 {
    type Port = pac::p6::RegisterBlock;
}
impl GpioPort for P6 {
    type PortNum = Port6;
}

#[doc(hidden)]
pub trait GpioFunction {}
#[doc(hidden)]
pub trait ConvertToOutput {}
#[doc(hidden)]
pub trait ConvertToInput {}
#[doc(hidden)]
pub trait Known {}

/// Typestate for an unknown state.
/// When used as directional typestate, this signifies a non-alternate (GPIO) pin.
pub struct Unknown;
impl ConvertToInput for Unknown {}
impl ConvertToOutput for Unknown {}
impl GpioFunction for Unknown {}

/// Direction typestate for GPIO output
pub struct Output;
impl ConvertToInput for Output {}
impl GpioFunction for Output {}

/// Direction typestate for GPIO input.
/// The type parameter specifies pull direction of input.
pub struct Input<PULL>(PhantomData<PULL>);
impl<PULL> ConvertToOutput for Input<PULL> {}
impl<PULL> GpioFunction for Input<PULL> {}

/// Pull typestate for pullup inputs
pub struct Pullup;
impl Known for Pullup {}

/// Pull typestate for pulldown inputs
pub struct Pulldown;
impl Known for Pulldown {}

/// Pull typestate for floating inputs
pub struct Floating;
impl Known for Floating {}

/// A single GPIO pin on the chip.
pub struct Pin<PORT: PortNum, PIN: PinNum, DIR> {
    _port: PhantomData<PORT>,
    _pin: PhantomData<PIN>,
    _dir: PhantomData<DIR>,
}

macro_rules! make_pin {
    () => {
        Pin {
            _port: PhantomData,
            _pin: PhantomData,
            _dir: PhantomData,
        }
    };
}

/// Contention token for the PxOUT register.
/// Used to prevent races when accessing the PxOUT register from different pins on the same port.
pub struct Pxout<P: GpioPeriph>(PhantomData<P>);

/// Contention token for the PxDIR register.
/// Used to prevent races when accessing the PxDIR register from different pins on the same port.
pub struct Pxdir<P: GpioPeriph>(PhantomData<P>);

/// Contention token for the interrupt registers.
/// Used to prevent races when accessing the inerrupt registers from different pins on the same port.
pub struct Pxint<P: GpioPeriph>(PhantomData<P>);

impl<PORT: PortNum, PIN: PinNum, PULL> Pin<PORT, PIN, Input<PULL>> {
    /// Configures pin as pulldown input
    /// This method requires a `Pxout` token because configuring pull direction requires setting
    /// the PxOUT register, which can race with setting an output pin on the same port.
    pub fn pulldown(self, _pxout: &mut Pxout<PORT::Port>) -> Pin<PORT, PIN, Input<Pulldown>> {
        let p = PORT::Port::steal();
        p.pxout_mod(|b| b.clear(PIN::pin()));
        p.pxren_mod(|b| b.set(PIN::pin()));
        make_pin!()
    }

    /// Configures pin as pullup input
    /// This method requires a `Pxout` token because configuring pull direction requires setting
    /// the PxOUT register, which can race with setting an output pin on the same port.
    pub fn pullup(self, _pxout: &mut Pxout<PORT::Port>) -> Pin<PORT, PIN, Input<Pullup>> {
        let p = PORT::Port::steal();
        p.pxout_mod(|b| b.set(PIN::pin()));
        p.pxren_mod(|b| b.set(PIN::pin()));
        make_pin!()
    }

    /// Configures pin as floating input
    pub fn float(self, _pxout: &mut Pxout<PORT::Port>) -> Pin<PORT, PIN, Input<Floating>> {
        let p = PORT::Port::steal();
        p.pxren_mod(|b| b.clear(PIN::pin()));
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum, PULL: Known> Pin<PORT, PIN, Input<PULL>>
where
    PORT::Port: IntrPeriph,
{
    /// Enable rising edge interrupts on the input pin.
    /// Note that changing other GPIO configurations while interrupts are enabled can cause
    /// spurious interrupts.
    pub fn enable_interrupt_rising_edge(&mut self, _pxint: &mut Pxint<PORT::Port>) {
        let p = PORT::Port::steal();
        p.pxies_mod(|b| b.clear(PIN::pin()));
        p.pxifg_mod(|b| b.clear(PIN::pin()));
        p.pxie_mod(|b| b.set(PIN::pin()));
    }

    /// Enable falling edge interrupts on the input pin.
    /// Note that changing other GPIO configurations while interrupts are enabled can cause
    /// spurious interrupts.
    pub fn enable_interrupt_falling_edge(&mut self, _pxint: &mut Pxint<PORT::Port>) {
        let p = PORT::Port::steal();
        p.pxies_mod(|b| b.set(PIN::pin()));
        p.pxifg_mod(|b| b.clear(PIN::pin()));
        p.pxie_mod(|b| b.set(PIN::pin()));
    }

    /// Disable interrupts on input pin.
    pub fn disable_interrupt(&mut self, _pxint: &mut Pxint<PORT::Port>) {
        let p = PORT::Port::steal();
        p.pxie_mod(|b| b.clear(PIN::pin()));
    }
}

impl<PORT: PortNum, PIN: PinNum, DIR: ConvertToOutput> Pin<PORT, PIN, DIR> {
    /// Configures pin as output
    pub fn to_output(self, _pxdir: &mut Pxdir<PORT::Port>) -> Pin<PORT, PIN, Output> {
        let p = PORT::Port::steal();
        p.pxdir_mod(|b| b.set(PIN::pin()));
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum, DIR: ConvertToInput> Pin<PORT, PIN, DIR> {
    /// Configures pin as input
    pub fn to_input(self, _pxdir: &mut Pxdir<PORT::Port>) -> Pin<PORT, PIN, Input<Unknown>> {
        let p = PORT::Port::steal();
        p.pxdir_mod(|b| b.clear(PIN::pin()));
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Output> {
    /// Use the `Pxout` token to create a output pin "proxy" on which output operations can be
    /// done. The token ensures that different output pin writes on the same port don't race with
    /// each other. We need to do this because unlike ARM, output writes on MSP430 require
    /// read-modify-write, which is not atomic.
    pub fn proxy<'out: 'a + 'b, 'a, 'b>(
        &'a mut self,
        _pxout: &'b mut Pxout<PORT::Port>,
    ) -> OutputPinProxy<'out, PORT, PIN> {
        OutputPinProxy {
            _out: PhantomData,
            _port: PhantomData,
            _pin: PhantomData,
        }
    }
}

impl<PORT: PortNum, PIN: PinNum, PULL: Known> InputPin for Pin<PORT, PIN, Input<PULL>> {
    type Error = void::Void;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let p = PORT::Port::steal();
        Ok(p.pxin_rd().check(PIN::pin()) != 0)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|r| !r)
    }
}

/// Proxy type for an output pin
pub struct OutputPinProxy<'out, PORT: PortNum, PIN: PinNum> {
    _out: PhantomData<&'out u8>,
    _port: PhantomData<PORT>,
    _pin: PhantomData<PIN>,
}

impl<'out, PORT: PortNum, PIN: PinNum> OutputPin for OutputPinProxy<'out, PORT, PIN> {
    type Error = void::Void;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let p = PORT::Port::steal();
        p.pxout_mod(|b| b.clear(PIN::pin()));
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let p = PORT::Port::steal();
        p.pxout_mod(|b| b.set(PIN::pin()));
        Ok(())
    }
}

impl<'out, PORT: PortNum, PIN: PinNum> StatefulOutputPin for OutputPinProxy<'out, PORT, PIN> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        let p = PORT::Port::steal();
        Ok(p.pxout_rd().check(PIN::pin()) != 0)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|r| !r)
    }
}

impl<'out, PORT: PortNum, PIN: PinNum> ToggleableOutputPin for OutputPinProxy<'out, PORT, PIN> {
    type Error = void::Void;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        let p = PORT::Port::steal();
        p.pxout_mod(|b| b ^ (1 << PIN::pin()));
        Ok(())
    }
}

/// GPIO parts for a specific port, including all 8 pins and register contention tokens
pub struct Parts<PORT: PortNum, DIR0, DIR1, DIR2, DIR3, DIR4, DIR5, DIR6, DIR7> {
    /// Pin0
    pub pin0: Pin<PORT, Pin0, DIR0>,
    /// Pin1
    pub pin1: Pin<PORT, Pin1, DIR1>,
    /// Pin2
    pub pin2: Pin<PORT, Pin2, DIR2>,
    /// Pin3
    pub pin3: Pin<PORT, Pin3, DIR3>,
    /// Pin4
    pub pin4: Pin<PORT, Pin4, DIR4>,
    /// Pin5
    pub pin5: Pin<PORT, Pin5, DIR5>,
    /// Pin6
    pub pin6: Pin<PORT, Pin6, DIR6>,
    /// Pin7
    pub pin7: Pin<PORT, Pin7, DIR7>,

    /// Interrupt register contention token
    pub pxint: Pxint<PORT::Port>,
    /// PxOUT contention token
    pub pxout: Pxout<PORT::Port>,
    /// PxDIR contention token
    pub pxdir: Pxdir<PORT::Port>,
}

impl<PORT: PortNum, DIR0, DIR1, DIR2, DIR3, DIR4, DIR5, DIR6, DIR7>
    Parts<PORT, DIR0, DIR1, DIR2, DIR3, DIR4, DIR5, DIR6, DIR7>
{
    /// Converts all parts into a GPIO batch so the entire port can be configured at once
    pub fn batch(self) -> Batch<PORT, DIR0, DIR1, DIR2, DIR3, DIR4, DIR5, DIR6, DIR7> {
        Batch::new()
    }

    pub(super) fn new() -> Self {
        Self {
            pin0: make_pin!(),
            pin1: make_pin!(),
            pin2: make_pin!(),
            pin3: make_pin!(),
            pin4: make_pin!(),
            pin5: make_pin!(),
            pin6: make_pin!(),
            pin7: make_pin!(),
            pxint: Pxint(PhantomData),
            pxout: Pxout(PhantomData),
            pxdir: Pxdir(PhantomData),
        }
    }
}

// Methods for managing sel1, sel0, and selc registers
impl<PORT: PortNum, PIN: PinNum, DIR> Pin<PORT, PIN, DIR> {
    fn set_sel0(&mut self) {
        let p = PORT::Port::steal();
        p.pxsel0_mod(|b| b.set(PIN::pin()));
    }

    fn set_sel1(&mut self) {
        let p = PORT::Port::steal();
        p.pxsel1_mod(|b| b.set(PIN::pin()));
    }

    fn clear_sel0(&mut self) {
        let p = PORT::Port::steal();
        p.pxsel0_mod(|b| b.clear(PIN::pin()));
    }

    fn clear_sel1(&mut self) {
        let p = PORT::Port::steal();
        p.pxsel1_mod(|b| b.clear(PIN::pin()));
    }

    fn flip_selc(&mut self) {
        let p = PORT::Port::steal();
        // Change both sel0 and sel1 bits at once
        p.pxselc_wr(0u8.set(PIN::pin()));
    }
}

/// Typestate for GPIO alternate function 1
pub struct Alternate1;

/// Typestate for GPIO alternate function 2
pub struct Alternate2;

/// Typestate for GPIO alternate function 3
pub struct Alternate3;

#[doc(hidden)]
pub trait ToAlternate1 {}
#[doc(hidden)]
pub trait ToAlternate2 {}
#[doc(hidden)]
pub trait ToAlternate3 {}

impl<PORT: PortNum, PIN: PinNum, DIR: GpioFunction> Pin<PORT, PIN, DIR>
where
    Self: ToAlternate1,
{
    /// Convert pin to GPIO alternate function 1
    pub fn to_alternate1(mut self) -> Pin<PORT, PIN, Alternate1> {
        self.set_sel0();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum, DIR: GpioFunction> Pin<PORT, PIN, DIR>
where
    Self: ToAlternate2,
{
    /// Convert pin to GPIO alternate function 2
    pub fn to_alternate2(mut self) -> Pin<PORT, PIN, Alternate2> {
        self.set_sel1();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum, DIR: GpioFunction> Pin<PORT, PIN, DIR>
where
    Self: ToAlternate3,
{
    /// Convert pin to GPIO alternate function 3
    pub fn to_alternate3(mut self) -> Pin<PORT, PIN, Alternate3> {
        self.flip_selc();
        make_pin!()
    }
}

// sel0 = 1, sel1 = 0
impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate1> {
    /// Convert pin to GPIO function
    pub fn to_gpio(mut self) -> Pin<PORT, PIN, Unknown> {
        self.clear_sel0();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate1>
where
    Self: ToAlternate2,
{
    /// Convert pin to alternate function 2
    pub fn to_alternate2(mut self) -> Pin<PORT, PIN, Alternate2> {
        self.flip_selc();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate1>
where
    Self: ToAlternate3,
{
    /// Convert pin to alternate function 3
    pub fn to_alternate3(mut self) -> Pin<PORT, PIN, Alternate3> {
        self.set_sel1();
        make_pin!()
    }
}

// sel0 = 0, sel1 = 1
impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate2> {
    /// Convert pin to GPIO function
    pub fn to_gpio(mut self) -> Pin<PORT, PIN, Unknown> {
        self.clear_sel1();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate2>
where
    Self: ToAlternate1,
{
    /// Convert pin to alternate function 1
    pub fn to_alternate1(mut self) -> Pin<PORT, PIN, Alternate1> {
        self.flip_selc();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate2>
where
    Self: ToAlternate3,
{
    /// Convert pin to alternate function 3
    pub fn to_alternate3(mut self) -> Pin<PORT, PIN, Alternate3> {
        self.set_sel0();
        make_pin!()
    }
}

// sel0 = 1, sel1 = 1
impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate3> {
    /// Convert pin to GPIO function
    pub fn to_gpio(mut self) -> Pin<PORT, PIN, Unknown> {
        self.flip_selc();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate3>
where
    Self: ToAlternate1,
{
    /// Convert pin to alternate function 1
    pub fn to_alternate1(mut self) -> Pin<PORT, PIN, Alternate1> {
        self.clear_sel1();
        make_pin!()
    }
}

impl<PORT: PortNum, PIN: PinNum> Pin<PORT, PIN, Alternate3>
where
    Self: ToAlternate2,
{
    /// Convert pin to alternate function 2
    pub fn to_alternate2(mut self) -> Pin<PORT, PIN, Alternate2> {
        self.clear_sel0();
        make_pin!()
    }
}

// P1 alternate 1
impl<PIN: PinNum, DIR> ToAlternate1 for Pin<Port1, PIN, DIR> {}
// P1 alternate 2
impl<DIR> ToAlternate2 for Pin<Port1, Pin0, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port1, Pin1, DIR> {}
impl<PULL> ToAlternate2 for Pin<Port1, Pin2, Input<PULL>> {}
impl<DIR> ToAlternate2 for Pin<Port1, Pin6, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port1, Pin7, DIR> {}
// P1 alternate 3
impl<PIN: PinNum, DIR> ToAlternate3 for Pin<Port1, PIN, DIR> {}

// P2 alternate 1
impl<DIR> ToAlternate1 for Pin<Port2, Pin0, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port2, Pin1, DIR> {}
impl<PULL> ToAlternate1 for Pin<Port2, Pin2, Input<PULL>> {}
impl<DIR> ToAlternate1 for Pin<Port2, Pin3, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port2, Pin6, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port2, Pin7, DIR> {}
// P2 alternate 2
impl ToAlternate2 for Pin<Port2, Pin0, Output> {}
impl ToAlternate2 for Pin<Port2, Pin1, Output> {}
impl<DIR> ToAlternate2 for Pin<Port2, Pin6, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port2, Pin7, DIR> {}
// P2 alternate 3
impl<DIR> ToAlternate3 for Pin<Port2, Pin4, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port2, Pin5, DIR> {}

// P3 alternate 1
impl<DIR> ToAlternate1 for Pin<Port3, Pin0, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port3, Pin4, DIR> {}
// P3 alternate 3
impl<DIR> ToAlternate3 for Pin<Port3, Pin1, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port3, Pin2, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port3, Pin3, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port3, Pin5, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port3, Pin6, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port3, Pin7, DIR> {}

// P4 alternate 1
impl<PIN: PinNum, DIR> ToAlternate1 for Pin<Port4, PIN, DIR> {}
// P4 alternate 2
impl<DIR> ToAlternate2 for Pin<Port4, Pin0, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port4, Pin2, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port4, Pin3, DIR> {}

// P5 alternate 1
impl<DIR> ToAlternate1 for Pin<Port5, Pin0, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port5, Pin1, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port5, Pin2, DIR> {}
impl<DIR> ToAlternate1 for Pin<Port5, Pin3, DIR> {}
// P5 alternate 2
impl<DIR> ToAlternate2 for Pin<Port5, Pin0, DIR> {}
impl<DIR> ToAlternate2 for Pin<Port5, Pin1, DIR> {}
// P5 alternate 3
impl<DIR> ToAlternate3 for Pin<Port5, Pin0, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port5, Pin1, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port5, Pin2, DIR> {}
impl<DIR> ToAlternate3 for Pin<Port5, Pin3, DIR> {}

// P6 alternate 1
impl<PIN: PinNum, DIR> ToAlternate1 for Pin<Port6, PIN, DIR> {}
