use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

use serialport::SerialPort;
use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct JumpSignal(pub Arc<Mutex<bool>>);

pub fn setup_serial_listener(signal: Res<JumpSignal>) {
    let signal_clone = signal.0.clone();

    thread::spawn(move || {
        let port = serialport::new("COM5", 115200)
            .timeout(Duration::from_millis(100))
            .open();

        if let Ok(mut port) = port {
            let reader = BufReader::new(port);
            for line in reader.lines() {
                if let Ok(data) = line {
                    if data.trim() == "JUMP" {
                        if let Ok(mut lock) = signal_clone.lock() {
                            *lock = true;
                        }
                    }
                }
            }
        } else {
            eprintln!("⚠️ Kunne ikke åbne COM5");
        }
    });
}
