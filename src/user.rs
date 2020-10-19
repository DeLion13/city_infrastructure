use crate::communal::*;
use crate::implementations::*;

use std::collections::HashMap;

pub struct User <'a> {
    pub balance : i32,
    pub age : i8,
    pub login : &'a str,

    state_level : i8,

    pub cart : HashMap<Communal<'a>, i32>,
    pub inventory : HashMap<Communal<'a>, i32>,
}

impl<'a> User<'a> {
    pub fn new(balance : i32,
               age : i8,
               login : &'a str) -> User<'a> {
        let cart = HashMap::new();
        let inventory = HashMap::new();
        
        return User {balance : balance, age : age,
            state_level : 1, login : login,
            cart : cart, inventory : inventory};
    }

    pub fn get_state_level(self: &Self) -> i8 {
        return self.state_level;
    }

    pub fn set_state_level(self: &mut Self, new : i8) {
        self.state_level = new;
    }

    pub fn add_to_cart(self: &mut Self, communal : Communal<'a>) {
        if self.cart.contains_key(&communal) {
            *self.cart.entry(communal).or_insert(0) += 1;
        } else {
            self.cart.insert(communal, 1);
        }
    }

    pub fn remove_from_cart(self: &mut Self, communal : Communal<'a>) {
        if self.cart[&communal] > 1 {
            *self.cart.entry(communal).or_insert(0) -= 1;
        } else {
            self.cart.remove(&communal);
        }
    }

    pub fn pay_communal(self: &mut Self) -> bool {
        if self.cart.len() != 0 {
            let mut to_pay = 0;
            for (key, value) in self.cart.iter() {
                to_pay += key.cost * value;
            }
            if self.balance < to_pay {return false;}
            else {
                self.balance -= to_pay;
                for (key, value) in self.cart.iter() {
                    self.inventory.insert(*key, *value);
                }
                self.cart.clear();
                return true;
            }
        }
        return false;
    }

    pub fn print_inventory(self: &Self) {
        println!("\n=====================
        \nYour balance: {}$
        \nPaid month:", self.balance);
        if self.inventory.len() == 0 {
            println!("    Didn't pay yet...");
        }
        else {
            for (key, value) in self.cart.iter() {
                self.print_communal(key.communal_type, *value);
            }
        }
    }

    pub fn print_communal(self: &Self, tp : CommunalType, num : i32) {
        if tp == CommunalType::ELECTRICITY {
            println!("    Electricity: {} month", num);
        }
        else if tp == CommunalType::WARMING {
            println!("    Warming: {} month", num);
        }
        else if tp == CommunalType::WATER {
            println!("    Water: {} month", num);
        }
        else if tp == CommunalType::GAS {
            println!("    Gas: {} month", num);
        }
    }

    pub fn print_user_info(self: &Self) {
        println!("=======USER=======");
        println!("Username: {}", self.login);
        println!("Balance:  {}", self.balance);
        println!("==================\n");
    }
}

