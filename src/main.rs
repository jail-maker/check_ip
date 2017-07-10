extern crate ipnetwork;
#[macro_use]
extern crate clap;

use clap::App;

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

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let v4 = matches.value_of("ipv4");
    let v6 = matches.value_of("ipv6");

    println!("{:?}", v4);
    println!("{:?}", v6);

    // let network: Ipv4Network = "192.168.0.1/24".parse().unwrap();
    // let network_addr = network.network();
    // let broadcast = network.broadcast();

    // for addr in network.iter() {

    //     if addr == broadcast || addr == network_addr {
    //         continue; 
    //     }

    //     let mut child = Command::new("ping")
    //         .arg("-c 1")
    //         .arg("-i 0")
    //         .arg("-W 10")
    //         .arg("-o")
    //         .arg(format!("{}", addr))
    //         .stdin(Stdio::null())
    //         .stdout(Stdio::null())
    //         .stderr(Stdio::null())
    //         .spawn()
    //         .expect("error");

    //     let ecode = child.wait()
    //         .expect("error");

    //     if ecode.success() { continue; }

    //     match ecode.code().unwrap() {

    //         2 => println!("{:?}", addr),
    //         _ => continue

    //     }

    //     break;

    // }

}
