use rppal::gpio::{Gpio, OutputPin};
use rppal::uart::{Parity, Queue, Uart};
use std::error::Error;

const CHANNEL1_EN: u8 = 27;
const CHANNEL2_EN: u8 = 22;

pub struct Ws2ChRs485Hat {
    ch_1: Uart,
    ch_2: Uart,
    en1: OutputPin,
    en2: OutputPin,
}

impl Ws2ChRs485Hat {
    pub fn new(ch1_baud_rate: u32, ch2_baud_rate: u32) -> Result<Self, Box<dyn Error>> {
        let ch_1 = Uart::with_path("/dev/ttySC0", ch1_baud_rate, Parity::None, 8, 1)?;
        let ch_2 = Uart::with_path("/dev/ttySC1", ch2_baud_rate, Parity::None, 8, 1)?;
        let mut en1 = Gpio::new()?.get(CHANNEL1_EN)?.into_output();
        let mut en2 = Gpio::new()?.get(CHANNEL2_EN)?.into_output();
        // default set receive enable
        en1.set_low();
        en2.set_low();
        Ok(Self {
            ch_1,
            ch_2,
            en1,
            en2,
        })
    }

    pub fn ch_1_read(&mut self, buffer: &mut [u8]) -> Result<usize, rppal::uart::Error> {
        self.en1.set_low();
        self.ch_1.read(buffer)
    }

    pub fn ch_2_read(&mut self, buffer: &mut [u8]) -> Result<usize, rppal::uart::Error> {
        self.en2.set_low();
        self.ch_2.read(buffer)
    }

    pub fn ch_1_write(&mut self, buffer: &[u8]) -> Result<usize, rppal::uart::Error> {
        self.en1.set_high();
        self.ch_1.write(buffer)
    }

    pub fn ch_2_write(&mut self, buffer: &[u8]) -> Result<usize, rppal::uart::Error> {
        self.en2.set_high();
        self.ch_2.write(buffer)
    }

    pub fn ch_1_flush(&self) -> Result<(), rppal::uart::Error> {
        self.ch_1.flush(Queue::Both)
    }

    pub fn ch_2_flush(&self) -> Result<(), rppal::uart::Error> {
        self.ch_2.flush(Queue::Both)
    }
}

// the tests is in examples
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
