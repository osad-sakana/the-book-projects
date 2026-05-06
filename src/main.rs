// 宙に浮いた参照
fn main(){
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s // sはこの関数のスコープを抜けるとドロップされるため、sへの参照は宙に浮いてしまう
}
