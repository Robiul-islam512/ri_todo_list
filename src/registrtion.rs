pub mod registration{
    use chrono::{Local};
    use serde::{Deserialize};
    
    #[derive(serde::Serialize,Deserialize,Debug)]
    pub struct UserRegistration{
        pub username: String,
        pub email:String,
        pub password: String,
        pub is_registered: bool,
        pub created_at:String,
        pub updated_at:String,
    }

    impl UserRegistration{
        pub fn new(username: String, email: String, password: String,is_registered: bool) -> Self {
            let now  = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            UserRegistration {
                username,
                email,
                password,
                is_registered,
                created_at:now.clone(),
                updated_at:now,
            }
        }
    }
}



