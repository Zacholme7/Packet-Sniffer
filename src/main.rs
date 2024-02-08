use pnet::datalink::{self, Channel, Config};

fn main() {
    let all_interfaces = datalink::interfaces();
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .expect("No suitable interface found");

    let config = Config::default();
    let channel = datalink::channel(default_interface, config).expect("Error creating datalink channel");

    match channel {
        Channel::Ethernet(_, mut receiver) => {
            // Now you have access to the receiver
            // You can start using it to receive packets here
            // For example, to receive packets in a loop:
            loop {
                match receiver.next() {
                    Ok(packet) => {
                        // Process the packet
                        println!("Received a packet! {:?}", packet);
                    },
                    Err(e) => {
                        eprintln!("An error occurred while reading the packet: {:?}", e);
                        break;
                    }
                }
            }
        }
        _ => todo!()
    }
}
