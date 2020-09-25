// all that i tested

pub fn test() {
    let user = user::User::new(1400, 16, 0, "Jeffo");

    println!("User: \n  Balance: {}$\n  Age: {} years\n Login: {}",
            user.get_balance(), user.get_age(), user.login);
}