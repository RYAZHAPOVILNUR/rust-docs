fn main() {
    println!("Hello, world!");

    let decimal = 98_222; // десятичная
    let hexadecimal = 0xfff; // шестнадцатеричная
    let octal = 0o77; // восьмеричная
    let binary = 0b1111_0000; // двоичная
    let byte = b'A'; // байтовая

    let x: f32 = 2.0100000000000000000000000000000000000000000063636363636363636636363636363636636363636630000001; // f64
    let y: f32 = 3.0; // f32

    let c = "z";
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let test = (123, 5.342, "hello");

    let mut test2 = test.0;
    test2 = 12342334;
    let none = ();

    println!("
        decimal: {decimal},
        hexadecimal: {hexadecimal},
        octal: {octal},
        binary: {binary},
        byte: {byte},
        x: {x},
        y: {y},
        z: {z},
        c: {c},
        {:?},
        {test2},
        {:?}
    ", test, none);
}
