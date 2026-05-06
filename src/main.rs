// 宙に浮いた参照
fn main(){
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("Hello");

    s // 参照を返すとスコープを抜けた時に本体が消えてしまうので所有権ごとムーブしてしまえばOK
}
