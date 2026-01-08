fn main() {
    // You can optionally experiment here.

    let target = "rustlings";
    let optional_target = Some(target);

    if let Some(word) = optional_target {
        println!("There is Some - {word:?} option.");
    }

    struct UserModel { id: u32, admin: bool }
    let test_user1 = UserModel { id: 1, admin: true };
    let test_user2 = UserModel { id: 2, admin: false };
    if let UserModel{id, admin : true} = test_user1 {
        println!("User {id:?} is an admin");
    }
    if let UserModel{id, admin : false} = test_user2 {
        println!("User {id:?} is not an admin");
    }

    enum  UserStatus {
        Move{speed_x:i32, speed_y:i32},
        Stop{pos_x:i32, pos_y:i32},
        Speak(String),
        Dead,
    }
    let user1_status = UserStatus::Stop{pos_x:10, pos_y:10};
    let user2_status = UserStatus::Speak(String::from("Hey All!"));
    if let UserStatus::Stop { pos_x, pos_y } = user1_status {
        println!("User stopped at position: ({pos_x}, {pos_y})");
    }

    if let UserStatus::Speak(speaking) = user2_status {
        println!("User says: {speaking}");
    }

    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];
    for i in 1..=range {
        optional_integers.push(Some(i));
    }
    while let Some(Some(integer)) = optional_integers.pop() {
        println!("while-let check there is integer - {integer}");
    }
    
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);


        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        assert_eq!(cursor, 0);
    }
}
