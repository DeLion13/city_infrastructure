pub struct Address<'a> {
    pub street : &'a str,
    pub build_number : &'a str,
}

pub struct InfrastructureComponent<'a> {
    pub name : &'a str,
    pub infrastructure_type : &'a str,
    pub address : Address<'a>
}

pub trait InfrastructureComponentInterface<T> {
    fn add(self : &mut Self, c : T);
    fn remove(self : &mut Self, index : usize);
    fn update(self : &mut Self, index : usize);
    fn show_it(self : &Self);
    fn show_children(self : &Self);
}

pub struct District<'a> {
    pub info : InfrastructureComponent<'a>,
    children : Vec<Type<'a>>
}

pub struct Type<'a> {
    pub info : InfrastructureComponent<'a>,
    children : Vec<Building<'a>>
}

pub struct Building<'a> {
    pub info : InfrastructureComponent<'a>
}

impl<'a> InfrastructureComponentInterface<Type<'a>> for District<'a> {
    fn add(self : &mut Self, c : Type<'a>) {
        self.children.push(c);
    }
    fn remove(self : &mut Self, index : usize) {
        self.children.remove(index);
    }
    //@todo
    fn update(self : &mut Self, index : usize) {
        let component : &Type = self.children.get(index).unwrap();
    }
    fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\nAddress: {}, #{}\n", self.info.name,
            self.info.infrastructure_type,
            self.info.address.street,
            self.info.address.build_number);
    }
    fn show_children(self : &Self) {
        for infra in self.children.iter() {
            println!("--{}", infra.info.name);
        }
    }
}

impl<'a> InfrastructureComponentInterface<Building<'a>> for Type<'a> {
    fn add(self : &mut Self, c : Building<'a>) {
        self.children.push(c);
    }
    fn remove(self : &mut Self, index : usize) {
        self.children.remove(index);
    }

    //@todo
    fn update(self : &mut Self, index : usize) {
        let component : &Building = self.children.get(index).unwrap();
    }
    fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\nAddress: {}, #{}\n", self.info.name,
            self.info.infrastructure_type,
            self.info.address.street,
            self.info.address.build_number);
    }
    fn show_children(self : &Self) {
        for infra in self.children.iter() {
            println!("--{}", infra.info.name);
        }
    }
}

impl<'a> Building<'a> {
    fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\nAddress: {}, #{}\n", self.info.name,
            self.info.infrastructure_type,
            self.info.address.street,
            self.info.address.build_number);
    }
}