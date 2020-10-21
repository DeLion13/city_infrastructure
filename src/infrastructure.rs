// InfrastructureComponent implementation

pub struct InfrastructureComponent<'a> {
    pub name : &'a str,
    pub infrastructure_type : &'a str,
}

impl<'a> InfrastructureComponent<'a> {
    pub fn new(name : &'a str, infrastructure_type : &'a str) -> InfrastructureComponent<'a> {
            
        InfrastructureComponent{name : name, infrastructure_type : infrastructure_type}
    }
}

// Address implementation

pub struct Address<'a> {
    pub street : &'a str,
    pub build_number : &'a str,
}

impl<'a> Address<'a> {
    pub fn new(street : &'a str, building : &'a str) -> Address<'a> {
        Address{street : street, build_number : building}
    }
}

// District implementation

pub struct District<'a> {
    pub info : InfrastructureComponent<'a>,
    children : Vec<Type<'a>>
}

impl<'a> District<'a> {
    pub fn new(name : &'a str, infrastructure_type : &'a str) -> District<'a> {
        District{info : InfrastructureComponent::new(name, infrastructure_type),
            children : Vec::new()}
    }
    pub fn add(self : &mut Self, c : Type<'a>) {
        self.children.push(c);
    }
    pub fn remove(self : &mut Self, index : usize) {
        self.children.remove(index);
    }
    //@todo
    // fn update(self : &mut Self, index : usize) {
    //     let component : &Type = self.children.get(index).unwrap();
    // }
    pub fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\n",
            self.info.name,
            self.info.infrastructure_type,);
    }
    pub fn show_children(self : &Self) {
        for infra in self.children.iter() {
            println!("--{}", infra.info.name);
        }
    }
}

// Type implementation
pub struct Type<'a> {
    pub info : InfrastructureComponent<'a>,
    children : Vec<Building<'a>>
}

impl<'a> Type<'a> {
    pub fn new(name : &'a str, infrastructure_type : &'a str) -> Type<'a> {
        Type{info : InfrastructureComponent::new(name, infrastructure_type),
            children : Vec::new()}
    }
    pub fn add(self : &mut Self, c : Building<'a>) {
        self.children.push(c);
    }
    pub fn remove(self : &mut Self, index : usize) {
        self.children.remove(index);
    }

    //@todo
    // fn update(self : &mut Self, index : usize) {
    //     let component : &Building = self.children.get(index).unwrap();
    // }
    pub fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\n",
            self.info.name,
            self.info.infrastructure_type,);
    }
    pub fn show_children(self : &Self) {
        for infra in self.children.iter() {
            println!("--{}", infra.info.name);
        }
    }
}

// Building implementation

pub struct Building<'a> {
    pub info : InfrastructureComponent<'a>,
    pub address : Address<'a>
}

impl<'a> Building<'a> {
    pub fn new(name : &'a str, infrastructure_type : &'a str,
        address : Address<'a>) -> Building<'a> {
            Building{info : InfrastructureComponent::new(name, infrastructure_type),
                address : address}
    }

    pub fn show_it(self : &Self) {
        println!("\nName: {}\nType: {}\nAddress: {}, #{}\n", self.info.name,
            self.info.infrastructure_type,
            self.address.street,
            self.address.build_number);
    }
}