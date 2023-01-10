fn main() {
    println!("Hello, world!");
    another_function(5);
    test(5, 'h');

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn another_function(x: i32) -> () {
    println!("Another function, {x}");
}

fn test(value: i32, label: char) -> () {
    println!("The measurement is: {value}{label}");
    5;
}
