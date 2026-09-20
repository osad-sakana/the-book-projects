fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    // 返しているstrのライフタイムがこの段階ではxに依存するのか、yに依存しているのかコンパイラにはわからないため、ライフタイムを明示する必要がある
    if x.len() > y.len() { x } else { y }
}
