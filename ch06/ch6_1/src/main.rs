fn main() {
}

fn basic_usage() {
    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind) {}

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn redundant_way() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}

fn concise_way() {

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

fn each_variant_can_have_different_types_and_amounts_of_associated_data() {

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// std::net::IpAddr (https://doc.rust-lang.org/std/net/enum.IpAddr.html)
fn std_net_IpAddr_example() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);
}

//------------------------------------------------------------------------------

struct QuitMessage; // ユニット構造体 unit struct
struct MoveMessage { x: i32, y: i32, } // 構造体 struct
struct WriteMessage(String); // タプル構造体 tuple struct
struct ChangeColorMessage(i32, i32, i32); // タプル構造体 tuple struct

// ↓ enumの場合、これと同じデータを、
// ↓ structキーワードを使わずに定義できる。
// ↓ また、全部の列挙子が Message 型としてグループ化される。

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// ❗ enum にもメソッドを定義することができる
impl Message {
    fn call(&self) {
        // -- snip --
    }
}
