use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io::{self, Write /*, ErrorKind*/};
use std::time::Duration;

// TODO: Error Handling will be improved by myself later

// With Lifetime
#[derive(Debug)]
struct MySerialPort {
    port_name: &'static str,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
    timeout: Duration,
}

// Implement the Default trait manually
impl Default for MySerialPort {
    // baud_rate, data_bits, stop_bits, parity, flow_control
    // are fixed values based on the datasheet
    fn default() -> Self {
        // Self is not an instance but a type
        MySerialPort {
            port_name: "/dev/ttyACM0",
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None, // use crc8
            flow_control: FlowControl::None,
            timeout: Duration::from_millis(100),
        }
    }
}

impl MySerialPort {
    // ========== Constructor ========
    // Specify the port name and open the port automatically
    pub fn new(port_name: &'static str) -> Self {
        Self {
            port_name,
            ..Self::default() // Fill the rest with defaults
        }
    }

    // ========== Public functions ======
    //
    // Open a serial port
    pub fn open(&self) -> Result<Box<dyn SerialPort>, ()> {
        let builder = serialport::new(self.port_name, self.baud_rate)
            .stop_bits(self.stop_bits)
            .data_bits(self.data_bits)
            .parity(self.parity)
            .flow_control(self.flow_control)
            .timeout(self.timeout);

        let port = builder.open().unwrap();
        Ok(port)
    }

    // just &self is a reference not mutable
    pub fn configure_timeout(&mut self, timeout_ms: u64) {
        self.timeout = Duration::from_millis(timeout_ms);
    }

    // Set a motor's id
    // This is important as the first step
    pub fn set_id(port: &mut Box<dyn SerialPort>, id: u8) {
        let command = [0xAA, 0x55, 0x53, id, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        // We must send the command five times in a row
        for _ in 0..5 {
            Self::send_command(port, &command);
            std::thread::sleep(Duration::from_millis(50));
        }

        // no feedback
    }

    // This is the second step
    // 0x01 current loop
    // 0x02 velocity
    // 0x03 position
    // The rotating velocity of the motor must be lower than 10rpm when switching to the position loop.
    fn switch_mode(port: &mut Box<dyn SerialPort>, id: u8, mode: u8) {
        let command = [id, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, mode];

        Self::send_command(port, &command);

        // no feedback
    }

    // wrapper function
    fn switch_to_current_mode(port: &mut Box<dyn SerialPort>, id: u8) {
        Self::switch_mode(port, id, 1);
    }

    fn switch_to_velocity_mode(port: &mut Box<dyn SerialPort>, id: u8) {
        Self::switch_mode(port, id, 2);
    }

    fn switch_to_position_mode(port: &mut Box<dyn SerialPort>, id: u8) {
        Self::switch_mode(port, id, 3);
    }

    // ========== Helper functions ================
    fn send_command(port: &mut Box<dyn SerialPort>, command: &[u8]) {
        match port.write_all(&command) {
            Ok(()) => (),
            Err(e) => (),
        }
    }

    fn read_response(&self, port: &mut Box<dyn SerialPort>) {
        let mut serial_buf: Vec<u8> = vec![0; 10];

        port.read_exact(serial_buf.as_mut_slice())
            .expect("Failed to read");
    }

    // CRC16 (CCITT) calculation
    fn calc_crc(data: &[u8]) -> u16 {
        let mut crc: u16 = 0;
        for &byte in data {
            crc ^= (byte as u16) << 8;
            for _ in 0..8 {
                crc = if crc & 0x8000 != 0 {
                    (crc << 1) ^ 0x1021
                } else {
                    crc << 1
                };
            }
        }
        crc
    }
}

fn main() {}
