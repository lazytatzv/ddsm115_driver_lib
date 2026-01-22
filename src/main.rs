use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};

struct SerialParams<'a> {
    port_name: &'a str,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
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

fn set_id() {
    
}

fn main() {}
