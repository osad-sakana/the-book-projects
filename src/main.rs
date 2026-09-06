use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("ファイルを作成する際に問題が発生しました。 {:?}", e),
            },
            other_error => {
                panic!("ファイルを開く際に問題が発生しました。 {:?}", other_error)
            }
        },
    };
}
