use serialport::SerialPort;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct SerialReader {
    pub jump_pressed: Arc<Mutex<bool>>,
    pub duck_pressed: Arc<Mutex<bool>>,
}

impl SerialReader {
    pub fn new(port_name: &str) -> Self {
        let jump_pressed = Arc::new(Mutex::new(false));
        let duck_pressed = Arc::new(Mutex::new(false));

        let jump_clone = Arc::clone(&jump_pressed);
        let duck_clone = Arc::clone(&duck_pressed);

        let port_name_owned = port_name.to_string(); // ✅ Convert to owned `String`

        thread::spawn(move || {
            let port_result = serialport::new(&port_name_owned, 115_200)
                .timeout(Duration::from_millis(10))
                .open();

            if let Ok(port) = port_result {
                let reader = BufReader::new(port);

                for line in reader.lines() {
                    if let Ok(data) = line {
                        if data.contains("JUMP") {
                            *jump_clone.lock().unwrap() = true;
                        }
                        if data.contains("DUCK") {
                            *duck_clone.lock().unwrap() = true;
                        }
                    }
                }
            } else {
                eprintln!("⚠️ Could not open serial port. Is the Micro:bit connected?");
            }
        });

        SerialReader {
            jump_pressed,
            duck_pressed,
        }
    }

    pub fn reset_inputs(&self) {
        *self.jump_pressed.lock().unwrap() = false;
        *self.duck_pressed.lock().unwrap() = false;
    }
}
