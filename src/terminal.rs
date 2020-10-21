use crate::communal::*;
use crate::infrastructure::*;
use crate::user::*;

pub struct Terminal<'a> {
    pub communals : Vec<Communal<'a>>,
    pub infrastructure : District<'a>,
    pub user : User<'a>
}

impl<'a> Terminal<'a> {
    pub fn new(infrastructure : District<'a>, user : User<'a>) -> Terminal<'a> {
        Terminal {communals : Vec::new(),
            infrastructure : infrastructure,
            user : user}
    }
    pub fn add_to_shop(self : &mut Self, communal : Communal<'a>) {
        self.communals.push(communal);
    }
    pub fn remove_from_shop(self : &mut Self, index : usize) {
        self.communals.remove(index);
    }
}