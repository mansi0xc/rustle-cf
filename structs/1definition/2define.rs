struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    /*
    Note that the struct update syntax uses = like an assignment;
    this is because it moves the data, just as we saw in the 
    “Variables and Data Interacting with Move” section. In this example, 
    we can no longer use user1 after creating user2 because the String 
    in the username field of user1 was moved into user2. If we had given 
    user2 new String values for both email and username, and thus only
     used the active and sign_in_count values from user1, then user1 
     would still be valid after creating user2. Both active and 
     sign_in_count are types that implement the Copy trait, so the 
     behavior we discussed in the “Stack-Only Data: Copy” section 
     would apply. We can also still use user1.email in this example, 
     because its value was not moved out of user1. */
}