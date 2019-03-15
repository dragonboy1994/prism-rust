use byteorder::{BigEndian, ByteOrder};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

const MSG_SIZE: usize = 10240;
const REPEAT_TIME: usize = 100000;

fn main() {
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);
    let server = prism::network::server::Server::start(listen_addr).unwrap();

    let message = prism::network::message::Message::EchoRequest("a".repeat(MSG_SIZE));
    let encoded: Vec<u8> = bincode::serialize(&message).unwrap();
    let length: u32 = encoded.len() as u32;
    let mut length_encoded = [0; 4];
    BigEndian::write_u32(&mut length_encoded, length);

    let socket = std::net::TcpStream::connect("127.0.0.1:9999").unwrap();
    let mut writer = std::io::BufWriter::new(socket);

    let start = Instant::now();
    for _ in 0..REPEAT_TIME {
        writer.write(&length_encoded).unwrap();
        writer.write(&encoded).unwrap();
    }
    writer.flush().unwrap();
    let end = Instant::now();
    let time = end.duration_since(start).as_micros() as f64;
    let throughput = MSG_SIZE as f64 * REPEAT_TIME as f64 * 1000000.0 / time / 1024.0 / 1024.0;
    println!("Message size: {} KB", MSG_SIZE / 1024);
    println!(
        "Throughput: {:.3} MB/s, {:.2} messages/s",
        throughput,
        REPEAT_TIME as f64 * 1000000.0 / time
    );
}