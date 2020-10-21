use crate::user::*;
use crate::communal::*;
use crate::infrastructure::*;

pub fn test() {
    // user creating
    let mut user = User::new(1400, 16, "Jeffo");

    println!("User: \n  Balance: {}$\n  Age: {} years\n  Login: {}",
            user.balance, user.age, user.login);

    // cart testing
    let com = Communal::new("Electro", 23, CommunalType::Electricity);
    user.add_to_cart(com);
    user.add_to_cart(com);

    println!("{}", user.cart[&com]);
    user.remove_from_cart(com);
    println!("{}", user.cart[&com]);
    user.remove_from_cart(com);

    // infrastructure testing
    let mut district = District::new("Sheva", "District");

    let mut communal_buildings = Type::new("Communal Buildings", "Types");
    let mut living_buildings = Type::new("Living Buildings", "Types");
    let mut commercial_buildings = Type::new("Commercial Buildings", "Types");

    let chemistry_chip = Building::new("Chemistry 'Rozumniy Vybir'", "Commercial", Address::new("Derevlyanska", "10"));
    let chemistry_simple = Building::new("Chemistry 'World'", "Commercial", Address::new("Biloruska", "17"));
    let chemistry_expensive = Building::new("Chemistry 'Manufacture'", "Commercial", Address::new("Shevchenka", "1"));
    let shop_fora = Building::new("Fora", "Commercial", Address::new("Derevlyanska", "10"));
    let shop_selling_centre = Building::new("Kvadrat", "Commercial", Address::new("Melnykova", "139"));

    let zhek = Building::new("ZHEK", "Communal", Address::new("Zoologychna", "7"));
    let post = Building::new("NovaPoshta #24", "Communal", Address::new("Bilorusska", "21"));
    let syayvo = Building::new("ECP 'Syayvo'", "Communal", Address::new("Melnykova", "2"));

    let b1 = Building::new("Build 1", "Living", Address::new("Biloruska", "23"));
    let b2 = Building::new("Build 2", "Living", Address::new("Biloruska", "21"));
    let b3 = Building::new("Build 3", "Living", Address::new("Dorogozhycka", "5A"));
    let b4 = Building::new("Build 4", "Living", Address::new("Simiy Hohlovyh", "2"));
    let b5 = Building::new("Build 5", "Living", Address::new("Kosmonavtyv", "17"));

    communal_buildings.add(zhek);
    communal_buildings.add(post);
    communal_buildings.add(syayvo);

    commercial_buildings.add(chemistry_chip);
    commercial_buildings.add(chemistry_simple);
    commercial_buildings.add(chemistry_expensive);
    commercial_buildings.add(shop_fora);
    commercial_buildings.add(shop_selling_centre);

    living_buildings.add(b1);
    living_buildings.add(b2);
    living_buildings.add(b3);
    living_buildings.add(b4);
    living_buildings.add(b5);

    district.add(communal_buildings);
    district.add(commercial_buildings);
    district.add(living_buildings);
}