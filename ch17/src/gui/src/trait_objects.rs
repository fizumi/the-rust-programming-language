// Listing 17-3: Definition of the Draw trait
pub trait Draw {
    fn draw(&self);
}
//------------------------------------------------------------------------------
// trait objects
//------------------------------------------------------------------------------
// Listing 17-4: Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    //              ↓ a vector of trait objects that implement the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}
// Listing 17-5: A run method on Screen that calls the draw method on each component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
