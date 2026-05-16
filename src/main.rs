enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddr) {
    // ここでIPアドレスに応じた処理をする
    println!("Routing complete!");
}

fn main(){
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}
