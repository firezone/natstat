use anyhow::Result;
use std::{
    net::{IpAddr, SocketAddr, UdpSocket},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use clap::{Parser, Subcommand};

const MAX_UDP_PORT: u16 = 65535;
const PACKET_NUMBER: usize = 1000;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Client {
        #[arg(short, long, env)]
        src_ip_address: IpAddr,
        #[arg(short, long, env)]
        dst_socket_address: SocketAddr,
        #[arg(short, long, env)]
        magic_number: u32,
    },
    Server {
        #[arg(short, long, env)]
        listen_addr: SocketAddr,
        #[arg(short, long, env)]
        magic_number: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Client {
            src_ip_address,
            dst_socket_address,
            magic_number,
        } => client(src_ip_address, dst_socket_address, magic_number)?,
        Commands::Server {
            listen_addr,
            magic_number,
        } => server(listen_addr, magic_number)?,
    }
    Ok(())
}

fn server(listen_addr: SocketAddr, magic_number: u32) -> Result<()> {
    let serv = UdpSocket::bind(listen_addr)?;
    let mut buf = [0u8; 6];
    while let Ok((len, addr)) = serv.recv_from(&mut buf) {
        if len == 6 && u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) == magic_number {
            println!(
                "{},{},{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                addr.port(),
                u16::from_be_bytes([buf[4], buf[5]])
            );
        }
    }
    Ok(())
}

fn client(src_ip_address: IpAddr, dst_socket_address: SocketAddr, magic_number: u32) -> Result<()> {
    for i in 1..=MAX_UDP_PORT {
        println!("Pinging port {i}");
        if let Err(err) = send_iter(&src_ip_address, i, &dst_socket_address, magic_number) {
            eprintln!("Error sending packet: {err}");
        }
    }
    Ok(())
}

fn send_iter(from: &IpAddr, src_port: u16, to: &SocketAddr, magic_number: u32) -> Result<()> {
    for _ in 0..PACKET_NUMBER {
        let socket = UdpSocket::bind((*from, src_port))?;
        let mut bytes_to_send = [0u8; 6];
        bytes_to_send[0..4].copy_from_slice(&magic_number.to_be_bytes());
        bytes_to_send[4..6].copy_from_slice(&src_port.to_be_bytes());
        socket.send_to(&bytes_to_send, to)?;
        std::thread::sleep(Duration::from_nanos(1_000))
    }
    Ok(())
}
