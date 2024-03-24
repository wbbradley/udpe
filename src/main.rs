use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    // Bind the UDP socket to port 8125
    let socket = UdpSocket::bind("0.0.0.0:8125")?;
    println!("Listening on UDP port 8125");

    loop {
        let mut buffer = [0; 1024]; // Buffer to store the incoming data

        // Receive data from the socket
        match socket.recv_from(&mut buffer) {
            Ok((number_of_bytes, src_addr)) => {
                println!(
                    "Received from {}: {}",
                    src_addr,
                    str::from_utf8(&buffer[..number_of_bytes])
                        .expect("Could not write buffer as string")
                );

                // Echo the received data back to the sender
                socket.send_to(&buffer[..number_of_bytes], src_addr)?;
            }
            Err(e) => {
                eprintln!("Could not receive a datagram: {}", e);
                break;
            }
        }
    }

    Ok(())
}
