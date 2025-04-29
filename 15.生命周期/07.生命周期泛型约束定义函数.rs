use std::fmt::Display;

fn main() {
    fn foo<'a, T>(s1: &'a &str, s2: &'a str, any: T) -> &'a str
    where
        T: Display,
    {
        println!("any is{any}");
        if s1.len() >= s2.len() {
            return s1;
        } else {
            return s2;
        }
    }
}
