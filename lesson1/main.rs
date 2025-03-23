// create a funtion which takes a string and returns a "muffin" with the concatenation of the string

fn muffinize(s: &str) -> String {
    let mut result = String::from(s);
    result.push_str(" muffins");
    result
}   

fn main() {
    println!("Hello, world!");
    println!("I love muffins, especially {}", muffinize("chocolate"));
}




