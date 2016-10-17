use std;
use std::time::Duration;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str;

pub fn discover() -> HashSet<SocketAddr> {

    let string_list = vec![
        "M-SEARCH * HTTP/1.1",
        "HOST:239.255.255.250:1900",
        "MAN:\"ssdp:discover\"",
        "ST:ssdp:all",
        "MX:1"
            ];
    let joined = string_list.join("\r\n");

    let socket =
        UdpSocket::bind("0.0.0.0:0").unwrap();

    let five_second_timeout = Duration::new(5, 0);
    let _ = socket.set_read_timeout(Some(five_second_timeout));
    socket.send_to(joined.as_bytes(), "239.255.255.250:1900").unwrap();

    let mut bridges = HashSet::new();
    loop {
        let mut buf = [0;255];
        let sockread = match socket.recv_from(&mut buf) {
            Ok(val) => Ok(val),
                Err(e) => {
                    match e.kind() {
                        // a timeout on unix is considered a WouldBlock
                        std::io::ErrorKind::WouldBlock => break,
                        _ => panic!(e),
                    }
                    Err(e)
                }
        };
        let _ = str::from_utf8(&buf).and_then(|s| {
            // Hue docs say to use "IpBridge" over "hue-bridgeid"
            if s.contains("IpBridge") {
                let bridge = sockread.unwrap().1;
                bridges.insert(bridge);
            }
            Ok(s)
        });
    }
    
    bridges
}
