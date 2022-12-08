mod user; // ref to file user.rs
use user::UserModel; // ref to module

fn main() {
    UserModel::Create::insert_and_hello(String::from("email@gmail.com"));
    println!("Hello, world!");
}
