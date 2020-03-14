use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ws_2_ch_rs485_hat::Ws2ChRs485Hat;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rs485 = Ws2ChRs485Hat::new(115_200, 115_200)?;

    let buffer = Box::new(b"Hello World!").to_vec();
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
        let n = rs485.ch_2_read(&mut buf[offset..])?;
        offset += n;
        sleep(Duration::from_millis(10));
    }
    rs485.ch_1_flush()?;
    rs485.ch_2_flush()?;
    println!("{}", std::str::from_utf8(&buf[..offset])?);
    Ok(())
}
