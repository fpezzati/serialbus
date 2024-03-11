use wasm_bindgen::prelude::wasm_bindgen;
use std::time::Duration;

#[wasm_bindgen]
pub fn start_srv() {
    let port = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_millis(10)).open();
    match port {
        Ok(mut port) => {
            println!("Receiving data");
            let mut buffer: Vec<u8> = vec![0; 1024];
            match port.read(buffer.as_mut_slice()) {
                Ok(bytesread) => {
                    println!("Data: {}", String::from_utf8(buffer[..bytesread].to_vec()).unwrap());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}