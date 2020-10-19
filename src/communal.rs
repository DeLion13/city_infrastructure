pub enum CommunalType {
    Water,
    Warming,
    Gas,
    Electricity
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