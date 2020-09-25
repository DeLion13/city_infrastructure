use std::collections::HashMap;

pub struct User <'a> {
    balance : i32,
    age : i8,
    state_level : i8,

    pub login : &'a str,

    cart : HashMap<&'a str, &'a str>,
    inventory : HashMap<&'a str, &'a str>,
}

impl User<'_> {
    // pub fn new<'a>(balance : i32,
    //            age : i8, state_level : i8,
    //            login : &'a str) -> User<'a> {
        
    //     return User {balance : balance, age : age, state_level : state_level, login : login};
    // }

    pub fn get_balance(self: &Self) -> i32 {
        return self.balance;
    }

    pub fn set_balance(self: &mut Self, new : i32) {
        self.balance = new;
    }

    pub fn get_age(self: &Self) -> i8 {
        return self.age;
    }

    pub fn set_age(self: &mut Self, new : i8) {
        self.age = new;
    }

    pub fn get_state_level(self: &Self) -> i8 {
        return self.state_level;
    }

    pub fn set_state_level(self: &mut Self, new : i8) {
        self.state_level = new;
    }
}

