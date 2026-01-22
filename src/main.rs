use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Write/*, ErrorKind*/};
use std::time::Duration;

// With Lifetime
struct SerialParams<'a> {
    port_name: &'a str,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
}

// Helper function
fn send_command(port: &mut Box<dyn SerialPort>, command: &[u8]) {
    match port.write_all(&command) {
        Ok(()) => (),
        Err(e) => (),
    }
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
    };

    let builder = serialport::new(serial_params.port_name, serial_params.baud_rate)
        .stop_bits(serial_params.stop_bits)
        .data_bits(serial_params.data_bits)
        .parity(serial_params.parity)
        .flow_control(serial_params.flow_control);

    let port = builder.open().map_err(|e| {
        format!("Error opening port: {}, Error: {}", serial_params.port_name, e).to_string()
    })?;

    Ok(port)
}

// Set a motor's id
// This is important as the first step
fn set_id(port: &mut Box<dyn SerialPort>, id: u8) {
    let command = [
        0xAA,
        0x55,
        0x53,
        id,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    
    for _ in 0..5 {
        send_command(port, &command);
        std::thread::sleep(Duration::from_millis(50));
    }
}

// This is the second step
// 0x01 current loop
// 0x02 velocity
// 0x03 position
// The rotating velocity of the motor must be lower than 10rpm when switching to the position loop.
fn switch_mode(port: &mut Box<dyn SerialPort>, id: u8, mode: u8) {    

    let command = [
        id,
        0xA0,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        mode,
    ];

    send_command(port, &command);

}

fn main() {}
