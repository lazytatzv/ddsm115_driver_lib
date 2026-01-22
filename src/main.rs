use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io::{self, Write /*, ErrorKind*/};
use std::time::Duration;

// With Lifetime
struct SerialParams<'a> {
    port_name: &'a str,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
    timeout: Duration,
}

// Helper function
fn send_command(port: &mut Box<dyn SerialPort>, command: &[u8]) {
    match port.write_all(&command) {
        Ok(()) => (),
        Err(e) => (),
    }
}

// Helper function
fn read_response(port: &mut Box<dyn SerialPort>) {
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

// Set params and open a port
fn init_port() -> Result<Box<dyn SerialPort>, String> {
    let serial_params = SerialParams {
        port_name: "/dev/ttyACM0",
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        stop_bits: StopBits::One,
        parity: Parity::None,
        flow_control: FlowControl::None,
        timeout: Duration::from_millis(10),
    };

    let builder = serialport::new(serial_params.port_name, serial_params.baud_rate)
        .stop_bits(serial_params.stop_bits)
        .data_bits(serial_params.data_bits)
        .parity(serial_params.parity)
        .flow_control(serial_params.flow_control)
        .timeout(serial_params.timeout);

    let port = builder.open().map_err(|e| {
        format!(
            "Error opening port: {}, Error: {}",
            serial_params.port_name, e
        )
        .to_string()
    })?;

    Ok(port)
}

// Set a motor's id
// This is important as the first step
fn set_id(port: &mut Box<dyn SerialPort>, id: u8) {
    let command = [0xAA, 0x55, 0x53, id, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    // We must send the command five times in a row
    for _ in 0..5 {
        send_command(port, &command);
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

    send_command(port, &command);

    // no feedback
}

// wrapper function
fn switch_to_current_mode(port: &mut Box<dyn SerialPort>, id: u8) {
    switch_mode(port, id, 1);
}

fn switch_to_velocity_mode(port: &mut Box<dyn SerialPort>, id: u8) {
    switch_mode(port, id, 2);
}

fn switch_to_position_mode(port: &mut Box<dyn SerialPort>, id: u8) {
    switch_mode(port, id, 3);
}

fn main() {}
