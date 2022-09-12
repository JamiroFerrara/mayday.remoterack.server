use tokio:: sync::broadcast;

pub struct Client {
    pub socket: tokio::net::TcpStream,
    pub address: std::net::SocketAddr,
    pub tx: broadcast::Sender<(String, std::net::SocketAddr)>,
    pub rx: broadcast::Receiver<(String, std::net::SocketAddr)>,
}
