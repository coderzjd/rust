pub trait Run {
    fn run(&self) {
        println!("run trait")
    }
}
pub struct Dog {
    pub name: String,
}
impl Run for Dog {}
