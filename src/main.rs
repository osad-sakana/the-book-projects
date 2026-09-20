fn main() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 1. resultに入る可能性があるのは、&string1か&string2
// 2. println!でresultを使用する時点でstring1は生存しているが、string2はすでにdropされている
// 3. もしresultに入っていたのが&string2であるならば、破棄済みのメモリ領域を参照することになる
// 4. したがって、参照元になりうるデータのうち、短い方の生存期間に合わせてresultのライフタイムを制限するというルールを適用せざるを得ない
