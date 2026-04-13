/// Enum Methods: Implement methods for an enum
enum Address {
    IPV4 (u8, u8, u8, u8),
    IPV6 (String)
}

impl Address {
    fn is_loopback(&self) -> bool {
        match self {
            Address::IPV4(a,b,c,d) => a == &127,
            Address::IPV6(addr) => addr == "::1"
        }
    } 
}

fn main() {
    let addr_ipv4 = Address::IPV4(127,1,2,4);
    let addr_ipv6 = Address::IPV6("::1".to_string());
    let addr_ipv4_not = Address::IPV4(123, 2,4,5);
    let addr_ipv6_not = Address::IPV6("::4".to_string());

    let mut is_loopback = addr_ipv4.is_loopback();
    println!("Given IPV4 is loopback: {}", is_loopback);
    is_loopback = addr_ipv6.is_loopback();
    println!("Given IPV6 is loopback: {}", is_loopback);
    is_loopback = addr_ipv4_not.is_loopback();
    println!("Given IPv4 is loopback: {}", is_loopback);
    is_loopback = addr_ipv6_not.is_loopback();
    println!("Given IPv6 is loopback: {}", is_loopback);
}
