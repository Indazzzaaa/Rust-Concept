#[allow(warnings)]

#[derive(Debug)] // It enables struct to used inside the println!.
struct User {
    active: bool,
    username: String,
}

impl User {
    // Associative methods : Active like static method in other languages.
    // when the names are same we can omit the left side.
    fn new(active: bool, username: String) -> Self {
        Self { active, username }
    }

    // we can create user from other user object.
    // rust support destruct property but its must be in the end only.
    fn from(otheruser: Self, username: String) -> Self {
        Self {
            username,
            ..otheruser
        }
    }

    fn get_active_status(&self) -> bool {
        self.active
    }

    fn set_active_status(&mut self, active: bool){
        self.active = active;
    }

    fn get_username(&self) -> &str {
        &self.username
    }


}

// Other types of structs.

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Place-Holder Structs
struct AlwaysEqual;



pub fn test(){

    let user1 = User::new(true, String::from("sumant@patel"));
    // have to use {:?} or {:#?} for printing the struct or we can implement the display trait which we will look later.
    println!("{:?}",user1); 

    let color1 = Color(1,1,1);
    println!("{}",color1.0);

}