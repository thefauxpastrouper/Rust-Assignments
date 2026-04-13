/// Pattern Matching Advanced: Use exhaustive pattern matching with enums
enum Address {
    IPV4(u8,u8,u8,u8),
    IPV6(String)
}

fn match_fn(addr: Address) -> String {
    match addr {
        Address::IPV4(_a,_b,_c,_d) => "ipv4 address".to_string(),
        Address::IPV6(_addr) => "ipv6 address".to_string()  
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
