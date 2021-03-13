fn main() {
    println!("Hello, world!");
}

fn defang_ip_addr(ip_addr: String) -> String {
    ip_addr.replace(".", "[.]")
}

#[test]
fn defang_ip_addr_test() {
    let ip_addr: String = String::from("1.1.1.1");
    assert_eq!("1[.]1[.]1[.]1", defang_ip_addr(ip_addr));
}
