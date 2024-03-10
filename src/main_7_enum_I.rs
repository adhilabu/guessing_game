
// enum IpvAddrKind { // enums
//     V4(String), // variants can store variables inside the variants
//     V5(String)
// }

enum IpvAddrKind { // enums
    V4(u8, u8, u8, u8), // variants can store variables inside the variants
    V5(u8, u8, u8, u8)
}

struct IpAddr {
    kind: IpvAddrKind,
    address: String
}

fn main() {
    let my_ip = IpvAddrKind::V4;
    let new_ip = IpvAddrKind::V5;
    
    let localhost: IpAddr = IpAddr {
        kind: IpvAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1")
    };
    
    // let localhost = IpAddr {
    //     kind: IpvAddrKind::V4(String::from("127.0.0.1")),
    //     address: String::from("127.0.0.1")
    // };
}