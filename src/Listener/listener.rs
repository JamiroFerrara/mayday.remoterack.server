use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener, sync::broadcast,
};

use crate::Listener::client::*;

pub struct Listener {
    pub tcp_listener: TcpListener,
    pub tx: broadcast::Sender<(String, std::net::SocketAddr)>,
    pub port: u16,
}

impl Listener {
    pub async fn new(channels: usize, ip: String, port: u16) -> Self {
        let listener = TcpListener::bind(ip + &port.to_string()).await.unwrap();
        let (tx, _rx) = broadcast::channel(channels);

        Self {
            tcp_listener: listener,
            tx,
            port,
        }
    }

    pub async fn accept(&self) -> Client {
        let (socket, address) = self.tcp_listener.accept().await.unwrap();
        match address {
            std::net::SocketAddr::V4(_) => println!("New v4 connection from {}", address),
            std::net::SocketAddr::V6(_) => println!("New v6 connection from {}", address),
        }

        //Returns a client
        Client {
            socket,
            address,
            tx: self.tx.clone(),
            rx: self.tx.subscribe()
        }
    }

    pub async fn start(&self) {
        loop {
            //This is a blocking call that waits for a new connection to be established
            let mut is_client = false;
            let mut client = self.accept().await;

            // Create task to handle multiple clients
            tokio::spawn(async move {
                let (read, mut writer) = client.socket.split();
                let mut reader = BufReader::new(read);
                let mut line = String::new();

                //Authenticates the client
                //This is a blocking call that waits for a new message to be received
                reader.read_line(&mut line).await.unwrap();
                let device_name = line.trim().to_string();
                is_client = is_device_client(device_name);
                line.clear();

                //Chat mechanism
                loop {
                    //Handle IO 
                    //Executes both statements in async manner 
                    tokio::select! {
                        // Read from client
                        result = reader.read_line(&mut line) => {
                            match result {
                                Ok(n) => {
                                    if n == 0 {
                                        break;
                                    }
                                    
                                    //Send message to all clients
                                    client.tx.send((line.clone(), client.address)).unwrap();
                                    line.clear();
                                }
                                Err(e) => {
                                    println!("Error reading from socket; err = {:?}", e);
                                    break;
                                }
                            }
                        }

                        // Write back to client
                        result = client.rx.recv() => {
                            match result {
                                Ok(res) => {
                                    let (msg, other_adder) = res;

                                        writer.write_all(&msg.as_bytes()).await.unwrap();
                                        println!("{}: {}", other_adder, msg);
                                }
                                Err(e) => {
                                    println!("Error reading from rx; err = {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                }

                if is_client {
                    println!("{} disconnected", client.address);
                }
            });
        }
    }
}

pub fn is_device_client(device_name: String) -> bool {
    if device_name == "RBP001" {
        // println!("BerryPi connected");
        return true;
    } else if device_name == "shell"{
        // println!("Shell connected");
        return false;
    }

    return false;
}
