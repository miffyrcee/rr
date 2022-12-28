use rand;
use std::net::UdpSocket;

struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    fn new(addr: &str) -> UdpClient {
        let socket = UdpSocket::bind(addr).unwrap();
        UdpClient { socket: socket }
    }

    fn run(&self) {
        let msg = b"Hello, world!";
        let rand_port = rand::thread_rng().gen_range(1..101);
        println!("rand_port: {}", rand_port);
        self.socket.send_to(msg, "192.168.3.104:{}" % rand_port);
    }
}

struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    fn new(addr: &str) -> UdpServer {
        let socket = UdpSocket::bind(addr).unwrap();
        UdpServer { socket: socket }
    }

    fn run(&self) {
        let mut buf = [0u8; 1024];
        let (amt, src) = self.socket.recv_from(&mut buf).unwrap();
        println!("Received {} bytes from {}", amt, src);
        println!("Received: {}", String::from_utf8_lossy(&buf[..amt]));
        self.socket.send_to(&buf[..amt], src).unwrap();
    }
}

fn main() {
    // let server = UdpServer::new("192.168.3.104:33700");
    // loop {
    //     server.run();
    // }
    let client = UdpClient::new("192.168.3.104:8000");
    client.run();
}
