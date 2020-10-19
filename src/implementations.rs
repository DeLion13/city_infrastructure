use std::hash::{Hash, Hasher};
// use std::ops::{Index,IndexMut};
// use std::collections::HashMap;
use crate::communal::*;
// use crate::user::*;

// CommunalType enum implementations

impl Copy for CommunalType { }

impl Clone for CommunalType {
    fn clone(&self) -> CommunalType {
        *self
    }
}

impl PartialEq for CommunalType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

// Communal struct implementations

impl<'a> Copy for Communal<'a> { }

impl<'a> Clone for Communal<'a> {
    fn clone(&self) -> Communal<'a> {
        *self
    }
}

impl<'a> PartialEq for Communal<'a> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<'a> Eq for Communal<'a> {}

impl<'a> Hash for Communal<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let communal_type : i8 = self.communal_type as i8;
        self.name.hash(state);
        communal_type.hash(state);
    }
}

// HashMap<communal::Communal<'_>, i32> struct implementations

// impl<'a> Index<Communal<'a>> for HashMap<Communal<'a>, i32> {
//     type Output = i32;

//     fn index(&self, index: Communal<'a>) -> &Self::Output {
//         &self[index]
//     }
// }

// impl<'a> IndexMut<Communal<'a>> for HashMap<Communal<'a>, i32> {
//     fn index_mut(&mut self, index: Communal<'a>) -> &mut Self::Output {
//         &mut self[index]
//     }
// }