use crc16::*;
use rppal::uart::Error as UartError;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ws_2_ch_rs485_hat::Ws2ChRs485Hat;

fn encode(buf: &mut Vec<u8>) {
    let crc = crc16::State::<MODBUS>::calculate(&buf).to_le_bytes();
    buf.extend_from_slice(&crc);
}

fn send(rs485: &mut Ws2ChRs485Hat, buffer: &[u8]) -> Result<(), UartError> {
    let len = buffer.len();
    let mut offset = 0;
    loop {
        let n = rs485.ch_1_write(&buffer[offset..])?;
        if n >= len {
            break;
        }
        offset += n;
        sleep(Duration::from_millis(5));
    }
    Ok(())
}

fn receive(rs485: &mut Ws2ChRs485Hat, buffer: &mut [u8]) -> Result<usize, UartError> {
    let mut offset = 0;
    for _ in 0..10 {
        let n = rs485.ch_1_read(&mut buffer[offset..])?;
        offset += n;
        sleep(Duration::from_millis(10));
    }
    rs485.ch_1_flush()?;
    Ok(offset)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rs485 = Ws2ChRs485Hat::new(9_600, 115_200)?;

    // encode changing device address request
    let mut req = vec![0x01, 0x06, 0x00, 0x66, 0x00, 0x03];
    encode(&mut req);
    send(&mut rs485, &req)?;

    sleep(Duration::from_millis(100));

    let mut buf = vec![0; 20];
    let offset = receive(&mut rs485, &mut buf).expect("uart error");
    println!("receive: {:?}", &buf[..offset]);

    sleep(Duration::from_millis(1000));

    // encode read status data request from new address
    let mut req = vec![0x03, 0x03, 0x00, 0x00, 0x00, 0x01];
    encode(&mut req);
    send(&mut rs485, &req)?;

    sleep(Duration::from_millis(100));

    let offset = receive(&mut rs485, &mut buf).expect("uart error");
    println!("receive: {:?}", &buf[..offset]);

    Ok(())
}
