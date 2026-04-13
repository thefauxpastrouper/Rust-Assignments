use std::net::IpAddr;

// unwrap vs expect: Demonstrate proper usage of both

fn main() {
    let ip = "1270.0.1".parse::<IpAddr>().unwrap();
    
    println!("{:?}", ip);

    let ip_expected:IpAddr = "1270.1".parse().expect("Hardcoded IP address should be valid!!");

    println!("{:?}", ip_expected);
}
