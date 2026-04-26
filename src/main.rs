const NUMBER: i32 = 5;

fn main(){
    let mut x = 5;
    println!("The value is {x}");

    x = x * NUMBER;
    println!("The value is {x}");

    {
        let x = 2;
        println!("The value is {x}");
    }

    println!("The value is {x}");
}
