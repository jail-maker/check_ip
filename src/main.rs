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
    Stdio,
    ExitStatus
};

use ipnetwork::{
    Ipv4Network,
    Ipv6Network
};

fn ping4(ip: Ipv4Addr) -> ExitStatus {

    let mut child = Command::new("ping")
        .arg("-c 1")
        .arg("-i 0")
        .arg("-W 10")
        .arg("-o")
        .arg(format!("{}", ip))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("error");

    let ecode = child.wait()
        .expect("error");

    ecode

}

fn ping6(ip: Ipv6Addr) -> ExitStatus {

    let mut child = Command::new("ping6")
        .arg("-c 1")
        .arg("-i 0")
        .arg("-X 10")
        .arg("-o")
        .arg(format!("{}", ip))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("error");

    let ecode = child.wait()
        .expect("error");

    ecode

}

fn get_free4(ip: &str) {

    let network: Ipv4Network = ip.parse().unwrap();
    let network_addr = network.network();
    let broadcast = network.broadcast();

    for addr in network.iter() {

        if addr == broadcast || addr == network_addr {
            continue; 
        }

        let ecode = ping4(addr);

        if ecode.success() { continue; }

        match ecode.code().unwrap() {

            2 => println!("free ipv4: {:?}", addr),
            _ => break

        }

        break;

    }

}

fn get_free6(ip: &str) {

    let network: Ipv6Network = ip.parse().unwrap();

    for addr in network.iter() {

        let ecode = ping6(addr);

        if ecode.success() { continue; }

        match ecode.code().unwrap() {

            2 => println!("free ipv6: {:?}", addr),
            _ => break

        }

        break;

    }

}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let v4 = matches.value_of("ipv4");
    let v6 = matches.value_of("ipv6");

    println!("{:?}", v4);
    println!("{:?}", v6);

    match (v6) {
        Some(ip) => get_free6(ip),
        None => ()
    }

    match (v4) {
        Some(ip) => get_free4(ip),
        None => ()
    }


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
