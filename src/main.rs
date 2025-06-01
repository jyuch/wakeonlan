use std::env;
use std::net::{Ipv4Addr, UdpSocket};

fn parse_mac(mac: &str) -> Result<[u8; 6], String> {
    let mac = mac.replace(":", "").replace("-", "");
    if mac.len() != 12 {
        return Err("Invalid MAC address length".to_string());
    }
    let mut bytes = [0u8; 6];
    for i in 0..6 {
        let byte_str = &mac[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| {
            format!(
                "MAC address contains an invalid hexadecimal digit {}",
                byte_str
            )
        })?;
    }
    Ok(bytes)
}

fn build_magic_packet(mac: [u8; 6]) -> Vec<u8> {
    let mut packet = vec![0xFF; 6];
    for _ in 0..16 {
        packet.extend_from_slice(&mac);
    }
    packet
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <MAC address>", args[0]);
        std::process::exit(1);
    }

    let mac_str = &args[1];
    let mac = match parse_mac(mac_str) {
        Ok(mac) => mac,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let packet = build_magic_packet(mac);

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let broadcast_addr = Ipv4Addr::new(255, 255, 255, 255);
    socket.send_to(&packet, (broadcast_addr, 7))?;

    Ok(())
}
