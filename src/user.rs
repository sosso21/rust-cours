pub mod UserModel {
    pub mod Create {
        pub fn insert(email: String) {
            println!("insert user")
        }
        pub fn insert_and_hello(email: String) {
            super::Create::insert(email);
            println!("hello")
        }
    }
}

// pub use crate::User_Model;
