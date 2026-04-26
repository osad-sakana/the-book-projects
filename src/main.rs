const NUMBER: i32 = 30;

fn main(){
    let x = if NUMBER < 10 {
        NUMBER
    } else {
        10
    };

    println!("The value of x is: {x}");
}
