
//use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
//use tokio::net::TcpStream;
use bitcoin::consensus::{encode, Decodable};
use bitcoin::network::{address, constants, message, message_network};
use std::io::{BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr,TcpStream};

use std::{env, process};

mod config;
mod peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of the Config struct based on the command-line arguments
    let config = config::Config::new();
    print!("{:?}",config);
     let p2p= peer::P2P::new(config);
   let _= on_connection().await;
    Ok(())
}



async fn on_connection() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr =  "127.0.0.1:18444".parse().unwrap_or_else(|error| {
        eprintln!("Error parsing address: {:?}", error);
        process::exit(1);
    });

    let version_message = peer::build_version_message(address);

    let first_message = message::RawNetworkMessage {
        magic: constants::Network::Regtest.magic(),
        payload: version_message,
    };
   // let mut tcp=TcpStream::connect(address).await?;
    let mut stream = TcpStream::connect(address).expect("Failed to open connection");
        // Send the message
        let _ = stream.write_all(encode::serialize(&first_message).as_slice());
            println!("Sent version message");

        // Setup StreamReader
        let read_stream = stream.try_clone().unwrap();
        let mut stream_reader = BufReader::new(read_stream);
        loop {
                       // Loop an retrieve new messages
            let reply = message::RawNetworkMessage::consensus_decode(&mut stream_reader).unwrap();
            match reply.payload {
                message::NetworkMessage::Version(_) => {
                    println!("Received version message: {:?}", reply.payload);

                    let second_message = message::RawNetworkMessage {
                        magic: constants::Network::Regtest.magic(),
                        payload: message::NetworkMessage::Verack,
                    };

                    let _ = stream.write_all(encode::serialize(&second_message).as_slice());
                    println!("Sent verack message");
                }
                message::NetworkMessage::Verack => {
                    println!("Received verack message: {:?}", reply.payload);
                    break;
                }
                _ => {
                    println!("Received unknown message: {:?}", reply.payload);
                    break;
                }
            }
    
        }
        let _ = stream.shutdown(Shutdown::Both);
        Ok(())    
    
}
