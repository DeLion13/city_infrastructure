pub enum CommunalType {
    WATER = 1,
    WARMING = 2,
    GAS = 3,
    ELECTRICITY = 4
}

pub struct Communal<'a> {
    pub name : &'a str,
    pub cost : i32,
    pub communal_type : CommunalType
}

impl<'a> Communal<'a> {
    pub fn new(name : &'a str, cost : i32,
    communal_type : CommunalType) -> Communal {
        return Communal {name : name, cost : cost, communal_type : communal_type};
    }
}