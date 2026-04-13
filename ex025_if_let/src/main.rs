/// if let: Rewrite match statements using if let where appropriate
enum Address {
    IPV4(u8,u8,u8,u8),
    IPV6(String)
}

fn match_fn(addr: Address) -> String {
    if let Address::IPV4(_,_,_,_) = addr {
        "ipv4 address".to_string()
    }else if let Address::IPV6(_) = addr {
        "ipv6 address".to_string()
    }else {
        "not an address".to_string()
    }
}

fn main() {
    let addr = Address::IPV4(127, 4,56,6);
    let addr_6 = Address::IPV6("::1".to_string());
    let res_string = match_fn(addr);
    let res_string_6 = match_fn(addr_6);
    println!("resulted string: {}", res_string);
    println!("resulted 2nd string: {}", res_string_6)
}



