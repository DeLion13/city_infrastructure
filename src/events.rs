pub enum EventType {
    AddToCart,
    RemoveFromCart,
    BuyItems,
    ShowItem,
    Add,
    Remove,
    Update
}

pub trait InfrastructureComponentInterface<T> {
    fn update(self : &mut Self, index : usize);
}