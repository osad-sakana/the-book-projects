use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    // ?演算子はエラーが発生した場合に即座に関数からエラーを返すため、前のmatch文と同じ効果を持つ
    // ただし、?演算子はResult型の値に対してのみ使用できるため、Result型の値を返す関数でのみ使用可能。
    Ok(username)
}
