use crate::communal::*;
use crate::infrastructure::*;
use crate::user::*;

pub struct Terminal<'a> {
    pub communals : Vec<Communal<'a>>,
    pub infrastructure : District<'a>,
    pub user : User<'a>
}

impl<'a> Terminal<'a> {
    pub fn add_to_shop(self : &mut Self, communal : Communal<'a>) {
        self.communals.push(communal);
    }
}