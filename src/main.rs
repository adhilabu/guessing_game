
enum IpvAddrKind {
    V4,
    V5
}

struct IpAddr {
    kind: IpvAddrKind,
    address: String
}

fn main() {
    let my_ip = IpvAddrKind::V4;
    let new_ip = IpvAddrKind::V5;
    
    ip_addr = 
}