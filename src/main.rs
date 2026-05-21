use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        name: "アスパラガス",
    };
    println!("This is an asparagus: {:?}", plant);
}
