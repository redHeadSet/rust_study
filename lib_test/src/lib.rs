// 
pub mod client; // 와 주석처리하면 failed to resolve: could not find `client` in `lib_test`
// pub mod network;
// 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::client;
        //crate::client::connect();
        //super::client::connect(); // 형제모듈.
        client::connect(); 
        //assert_eq!(2 + 2, 4);
    }
}

mod outermost {
    pub fn middle_function() {
        //crate::outermost::inside::secret_function(); //불가능
    }
     fn middle_secret_function() {
        crate::outermost::middle_secret_function();
        crate::outermost::pub_inside::inner_function();
        //crate::outermost::pub_inside::secret_function();
     }

     mod inside {
        pub fn inner_function() {
            crate::outermost::middle_secret_function(); //2018 버전에서는 crate 써야한다고 함.
            //https://users.rust-lang.org/t/error-e0433-failed-to-resolve-could-not-find-outermost-in-root/23220
            crate::outermost::inside::secret_function(); 
        }
         fn secret_function () {
            crate::outermost::middle_secret_function();
         }
     }

     pub mod pub_inside {
        pub fn inner_function() {
            //crate::outermost::inside::secret_function();
        }
         fn secret_function () {
            crate::outermost::middle_secret_function();
         }
     }
}

fn try_me() {
   //outermost::middle_function();
   //outermost::middle_secret_function();
   //outermost::inside::inner_function();
   //outermost::inside::secret_function();
   //outermost::pub_inside::secret_function();
}
