fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}", tup);

    let tup = (499, 5.4, 2);

    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);


    println!("x: {}, y: {}, z: {}", tup.1, tup.2, z);

    // array
    let a = [1, 3, 5, 6, 6, 10];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    for month in months {
        println!("{}", month);
    }

    let test = [1; 10];
    for x in test {
        println!("{}", x);
    }

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("index: {}", index);
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

