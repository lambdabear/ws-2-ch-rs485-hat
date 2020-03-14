use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ws_2_ch_rs485_hat::Ws2ChRs485Hat;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rs485 = Ws2ChRs485Hat::new(9_600, 115_200)?;

    let buffer = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x02, 0xC4, 0x0B];
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

    sleep(Duration::from_millis(100));
    let mut buf = vec![0; 20];
    let mut offset = 0;
    for _ in 0..10 {
        let n = rs485.ch_1_read(&mut buf[offset..])?;
        offset += n;
        sleep(Duration::from_millis(10));
    }
    rs485.ch_1_flush()?;
    println!("receive: {:?}", &buf[..offset]);
    println!(
        "Humi {}%; Temp {}C;",
        u16::from_be_bytes([buf[3], buf[4]]) as f32 / 10.0,
        i16::from_be_bytes([buf[5], buf[6]]) as f32 / 10.0
    );
    Ok(())
}
