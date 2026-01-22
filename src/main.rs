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

fn send_command(mut port: Box<dyn SerialPort>, command: &[u8]) {
    match port.write_all(&command) {
        Ok() => (),
        Err(e) => format!("{:?}", e),
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

fn set_id(mut port: Box<dyn SerialPort>, id: u8) {
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
        match port.write_all(&command) {
            Ok(_) => {
                print!("{:?}", command);
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn switch_mode(mut port: Box<dyn SerialPort>, id: u8, mode: u8) {

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
}

fn main() {}
