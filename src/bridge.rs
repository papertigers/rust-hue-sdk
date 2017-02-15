use std;
use std::io::Read;
use std::time::Duration;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str;

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_core::net::UdpSocket;

use rustc_serialize::json;
use hyper::Client;

/// Returns a HashSet of hue bridge SocketAddr's
pub fn discover() -> HashSet<Ipv4Addr> {
    //Storage for found hue bridges
    let mut bridges = HashSet::new();

    //Hue bridge discovery packet
    let string_list = vec![
        "M-SEARCH * HTTP/1.1",
        "HOST:239.255.255.250:1900",
        "MAN:\"ssdp:discover\"",
        "ST:ssdp:all",
        "MX:1"
    ];
    let joined = string_list.join("\r\n");

    //Create event loop for async io to happen on
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let sockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let socket = UdpSocket::bind(&sockaddr, &handle).unwrap();

    let msockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);
    socket.send_to(joined.as_bytes(), &msockaddr).unwrap();
    /*
    loop {
        let mut buf = [0;255];
        let sockread = match socket.recv_from(&mut buf) {
            Ok(val) => val,
            Err(e) => {
                match e.kind() {
                    // a timeout on unix is considered a WouldBlock
                    std::io::ErrorKind::WouldBlock => break,
                    _ => panic!(e),
                }
            }
        };
        let _ = str::from_utf8(&buf).and_then(|s| {
            // Hue docs say to use "IpBridge" over "hue-bridgeid"
            if s.contains("IpBridge") {
                if let IpAddr::V4(addr) = sockread.1.ip() {
                    bridges.insert(addr);
                }
            }
            Ok(s)
        });
    }
    */

    bridges
}

/// Hue Bridge
#[derive(Debug)]
pub struct Bridge {
    ip: Ipv4Addr,
}

impl Bridge {
    /// Returns a hue bridge with the given ip
    pub fn new(addr: Ipv4Addr) -> Bridge {
        Bridge {
            ip: addr,
        }
    }

    /// Attempt to register with the hue bridge
    pub fn register(&self, name: &str) {
        #[derive(RustcEncodable, RustcDecodable)]
        struct Devicetype {
            devicetype: String,
        }

        let client = Client::new();
        let url = format!("http://{}/api", self.ip);
        let payload = Devicetype { devicetype: name.to_owned() };
        let body = json::encode(&payload).unwrap();

        // TODO handle errors and return username
        let mut response = client.post(&url).body(&body).send().unwrap();
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        println!("{}", buf);
    }
}
