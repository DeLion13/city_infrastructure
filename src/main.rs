pub mod user;
pub mod communal;
pub mod menu;
pub mod implementations;
pub mod menu_factory;

fn main() {
    let mut us = user::User::new(100, 100, "nes");
    let com = communal::Communal::new("Electro", 23, communal::CommunalType::ELECTRICITY);
    us.add_to_cart(com);
    us.add_to_cart(com);

    println!("{}", us.cart[&com]);

    us.remove_from_cart(com);
    println!("{}", us.cart[&com]);
    us.remove_from_cart(com);
    println!("{}", us.cart[&com]);
}
