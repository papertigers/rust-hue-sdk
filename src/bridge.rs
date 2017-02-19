extern crate serde_json;

use std;
use std::str;
use std::io::Result as IoResult;
use std::time::Duration;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::collections::HashSet;

use hyper::Client;
use serde_json::Value;

use user::User;
use utils;
use error::Result;

/// Returns a HashSet of hue bridge SocketAddr's
pub fn discover() -> IoResult<HashSet<Ipv4Addr>> {

    let string_list = vec![
        "M-SEARCH * HTTP/1.1",
        "HOST:239.255.255.250:1900",
        "MAN:\"ssdp:discover\"",
        "ST:ssdp:all",
        "MX:1"
    ];
    let joined = string_list.join("\r\n");

    let socket =
        UdpSocket::bind("0.0.0.0:0")?;

    let two_second_timeout = Duration::new(2, 0);
    let _ = socket.set_read_timeout(Some(two_second_timeout));
    socket.send_to(joined.as_bytes(), "239.255.255.250:1900")?;

    let mut bridges = HashSet::new();
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

    Ok(bridges)
}

/// Hue Bridge
#[derive(Debug)]
pub struct Bridge {
    ip: Ipv4Addr,
    client: Client,
}

impl Bridge {
    /// Returns a hue bridge with the given ip
    pub fn new(addr: Ipv4Addr) -> Bridge {
        let mut client = Client::new();
        client.set_read_timeout(Some(Duration::new(2,0)));
        client.set_write_timeout(Some(Duration::new(2,0)));
        Bridge {
            ip: addr,
            client: client,
        }
    }

    /// Attempt to register with the hue bridge
    pub fn register(&self, name: &str) -> Result<User>{
        #[derive(Debug, Serialize, Deserialize)]
        struct Devicetype {
            devicetype: String,
        }

        let url = format!("http://{}/api", self.ip);
        let payload = Devicetype { devicetype: name.to_owned() };
        let body = serde_json::to_string(&payload)?;

        let response = self.client.post(&url).body(&body).send()?;
        let json: Value = serde_json::from_reader(response)?;
        utils::hue_result(json).and_then(|json| {
            let user: User = serde_json::from_value(json)?;
            Ok(user)
        })
    }
}
