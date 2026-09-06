use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("ファイルを作成できませんでした: {:?}", error);
            })
        } else {
            panic!("ファイルを開けませんでした: {:?}", error);
        }
    });
}
