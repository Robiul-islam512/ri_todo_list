
pub mod login{

    #[derive(Debug)]
    pub struct UserLogin{
        pub username_or_email:String,
        pub password:String,
    }
    
    #[derive(Debug)]
    pub struct Status{
        pub is_logged_in: bool,
        pub message: String,
    }

    #[derive(Debug)]
    pub enum Loginstatus {
        SuccessLogin(Status),
        WrongPassword(Status),
        UserNotFound(Status),
    }

    impl UserLogin{
        pub fn login(&self,username:&str,email:&str,password:&str)->Loginstatus{
            let input = self.username_or_email.as_str();
            let valid_user = input == email || input == username;
            let valid_password = self.password == password;

            if !valid_user{
                Loginstatus::UserNotFound(
                    Status { 
                        is_logged_in: false, 
                        message: String::from("User or email not found.") 
                    }
                )
            }
            else if !valid_password{
                Loginstatus::WrongPassword(
                    Status { 
                        is_logged_in: false, 
                        message:String::from("Wrong password.Try again.")
                    }
                )
            }
            else
            {
                Loginstatus::SuccessLogin(
                    Status { 
                        is_logged_in: true, 
                        message: String::from("Logged in successfull.") 
                    }
                )
                
            }
        }
    }

}