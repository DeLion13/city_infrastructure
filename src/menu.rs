use crate::communal::Communal;
use std::collections::*;

pub struct MenuOption<'a> {
    pub description : &'a str,
    // pub command : Command
}

pub struct Menu<'a> {
    pub hint : &'a str,
    // pub options : <Communal<'a>, i32>, // <communal, int>
    // pub inventory : HashMap<Communal<'a>, i32>, // <communal, int>
    pub options : Vec<MenuOption<'a>>,
    pub additional_options : HashMap<&'a str, MenuOption<'a>>
}

impl<'a> Menu<'a> {
    pub fn print_men(self : &Self) {
        println!("{}", self.hint);
        for i in 0..self.options.len() {
            println!("{}. {}", i + 1, self.options[i].description)
        }
        println!("");
    }
}

// impl MainMenu for Menu {

// }

// impl CommunalMenu for Menu {
    
// }

// impl CartMenu for Menu {
    
// }

// impl InfrastructureMenu for Menu {
    
// }

// impl CommunalObjectsMenu for Menu {
    
// }

// impl CommercialObjectsMenu for Menu {
    
// }

// impl LivingObjectsMenu for Menu {
    
// }

// impl LivingObjectsMenu for Menu {
    
// }