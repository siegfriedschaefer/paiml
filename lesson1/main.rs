// create a funtion which takes a string and returns a "muffin" with the concatenation of the string

fn muffinize(s: &str) -> String {
    let mut result = String::from(s);
    result.push_str(" muffins");
    result
}   

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for n in numbers {
        result += n;
    }
    result
}

fn own_integer(x: i32) {
    let y = x + 1;
    println!("own_integer(): x is {}", y);
}

fn main() {

    panic!("I'm panicking!");

    println!("Hello, world!");
    println!("I love muffins, especially {}", muffinize("chocolate"));

    let numbers = [1, 2, 3, 4, 5];
    println!("The sum of the numbers is {}", sum(&numbers));

    let mut x = 5;
    own_integer(x);
    println!("x is {}", x);

}




