extern crate ipnetwork;

use std::net::{
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    IpAddr
};

use std::process::{
    Command,
    Stdio
};

use ipnetwork::{
    Ipv4Network,
    Ipv6Network
};

fn main() {

    let ip4 = Ipv4Addr::new(192, 168, 0, 1);
    let network = Ipv4Network::new(ip4, 24).unwrap();

    for addr in network.iter() {

        let mut child = Command::new("ping")
            .arg("-c 1")
            .arg("-W 10")
            .arg(format!("{}", addr))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("error");

        let ecode = child.wait()
            .expect("error");

        if !ecode.success() {

            println!("{:?}", addr);

            break;

        }

    }

}
