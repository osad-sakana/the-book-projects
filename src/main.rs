// 構造体 意味のあるグループを形成する複数の関連した値をまとめ、名前づけできる独自のデータ型
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(
        String::from("yamada@example.com"),
        String::from("Taro Yamada"),
    );

    user1.email = String::from("anotheremail@example.com");

    println!("ユーザー名: {}", user1.username);
    println!("メールアドレス: {}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // usernameとemail引数は同じ名前なので、フィールド初期化省略記法を使用可能
        email,
        sign_in_count: 1,
    }
}
