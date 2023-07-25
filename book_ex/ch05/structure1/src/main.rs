fn main() {
    let user1 = User{
        active:true,
        username:String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User{
        active:true,
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
    };

    let user3 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count:user1.sign_in_count
    };

    let user4 = User{
        email:String::from("another@example.com"),
        ..user1
    };

    user2.email = String::from("anothereamil@example.com")
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String)->User{
    User{
        active:true,
        username:username,
        email:email,
        sign_in_count:1,
    }
}

fn build_user2(email: String, username: String)->User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}
