fn main() {
    let mut test = 0;

    let result = loop {
        test += 1;
        if test == 10 {
            break test * 3;
        }
    };

    let mut counter = 10;

    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }

    let array = [1, 2, 3, 1000, 5, 6, 7, 8, 9];

    for el in array.iter() {
        println!("el: {el}")
    }

    for number in (1..100).rev() {
        println!("{number}");
    }

    let s1 = String::from("hello");
    test_fn(&s1);

    println!("{s1}");

    // let s = String::from("hello");
    //
    // change(&s);
    //
    // fn change(string: &String) {
    //     string.push_str(", World!");
    // }

    let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // let s3 = &mut s1;
    // let s3 = &mut s1;
    // let s3 = &mut s1;
    // let s5 = &mut s1;
    // let s4 = &mut s1;

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    // change1(&mut s1);
    //
    // fn change1(string: &mut String) {
    //     string.push_str(", World!");
    // }

    let test = function1();

    let a = [1, 2 ,3 ,4 ,5];
    let slice = &a[1..3];
    println!("{:?}", slice)
}

fn function1() -> String {
    let string = String::from("hello");

    string
}

fn test_fn(string: &String) {
    // do nothing
}

