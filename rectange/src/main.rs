#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,

    some_method: fn() -> u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size, some_method: || 20 }
    }
}

fn main() {
    let height = 30;
    let width = 50;

    let dimensions = (30, 50);
    let rect = Rectangle {width: 30, height: 50, some_method: || 10 };
    let rect1 = Rectangle { width: 25, height: 40, some_method: || 10 };
    let hold = rect.can_hold(&rect1);
    let square = Rectangle::square(25);

    let area = get_area(width, height);
    let area1 = get_area_tuple(dimensions);
    let area2 = get_area_struct(&rect);
    println!("{:#?}", rect);
    println!("{:#?}", square);
    println!("{area}, {area1}, {area2}, {}, {}", rect.area(), hold);
}

fn get_area(width: u32, height: u32) -> u32 {
    width * height
}

fn get_area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn get_area_struct(dimensions: &Rectangle) -> u32 {
    return dimensions.width * dimensions.height
}
