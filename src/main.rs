use serialport::{DataBits, StopBits, Parity, FlowControl};

fn main() {
    // Make this configurable later
    let port_name = "/dev/ttyACM0";

    // Based on datasheet
    let baud_rate = 115200;
    let data_bits = DataBits::Eight;
    let stop_bits = StopBits::One;
    let parity = Parity::None;
    let flow_control = FlowControl::None;


    let builder = serialport::new(port_name, baud_rate)
        .stop_bits(stop_bits)
        .data_bits(data_bits)
        .parity(parity)
        .flow_control(flow_control);


}
