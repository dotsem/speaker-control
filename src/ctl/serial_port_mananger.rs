use std::sync::{Mutex};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

pub struct SerialPortManager {
    port: Mutex<Box<dyn SerialPort>>,
}

impl SerialPortManager {
    pub fn new(port_name: &str, baudrate: u32) -> Result<Self, serialport::Error> {
        let port = serialport::new(port_name, baudrate)
            .timeout(Duration::from_secs(1))
            .open()?;
        
        Ok(Self { 
            port: Mutex::new(port)
        })
    }
    
    pub fn send_command(&self, cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Format command with carriage return
        let cmd_str = format!("{}\r", cmd.trim());
        
        // Lock the mutex to get exclusive access to the port
        let mut port = self.port.lock().unwrap();
        
        // Write command
        port.write_all(cmd_str.as_bytes())?;
        
        // Read response
        let mut buffer = [0u8; 100];
        let bytes_read = port.read(&mut buffer)?;
        
        // Convert to String, ignoring invalid UTF-8
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string().replace("\r", "");
        
        Ok(response)
    }
}