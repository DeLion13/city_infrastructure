pub mod user;
pub mod communal;
pub mod menu;
pub mod implementations;
pub mod menu_factory;
pub mod infrastructure;
pub mod terminal;
pub mod events;

// testing modules
pub mod test;

fn main() {
    test::test();
}
