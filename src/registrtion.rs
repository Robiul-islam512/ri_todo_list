use chrono::Utc;

pub mod registration{
    use chrono::{DateTime, Utc};
    use serde::{Serialize,Deserialize};
    
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
            let now  = Utc::now().to_rfc3339();
            UserRegistration {
                username,
                email,
                password,
                is_registered,
                created_at:now.clone(),
                updated_at:now,
            }
        }

        // pub fn is_registered(&self) -> bool{
        //     self.is_registered
        // }
    }
}



