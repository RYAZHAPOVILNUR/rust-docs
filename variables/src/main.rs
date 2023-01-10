fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    let y = 6;

    let y = y +1;

    {
        let y = y *2;
        println!("y multiply : {}", y)
    }


    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    println!("y: {}", y);
}

