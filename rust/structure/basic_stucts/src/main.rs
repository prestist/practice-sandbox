fn main() {

    let new_user= User { //init new struct instance
        email: String::from("this_is_how_to_make_an_instance@basic.com"),
        username: String::from("this_is_annoying"),
        active: true,
        sign_in_count: 0
    };

    let new_user1 = User::new("this_is_preferred","much_better@email.com");

    println!("email: {}, username: {}, active: {}, sign_in_count {}",new_user.email,new_user.username,new_user.active,new_user.sign_in_count); //print attributes
    println!("email: {}, username: {}", new_user1.email, new_user1.username);


    }


struct User { //define your structs name and attributes.
    active: bool,
    username:String,
    email:String,
    sign_in_count: u64,
    
}

impl User { // You can implement functions based on a stuct's name. 

    fn new(_username: &str,_email: &str) -> User{
        User {
            active: true,
            username:_username.to_string(),
            email:_email.to_string(),
            sign_in_count: 0,
        }

    }
    fn reset_sign_in_count(&mut self){ //&self is similar to 'this' in helper functions in c#. Self needs a mut, if its going to be updated by the function. 
        self.sign_in_count = 0;
    }
    fn change_username(&mut self, new_name:&str){// same as helper functions, you can reference the instance and other arguments to adjust attributes. 
        self.username= new_name.to_string();
    }
    
}