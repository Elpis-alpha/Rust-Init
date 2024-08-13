fn main() {
    let rect1 = Rectangle::square(40);

    let rect2 = Rectangle {
        width: 10,
        height: 35,
    };

    let rect3 = Rectangle::new_rectangle(60, 45);

    println!(
        "The area of the rectangle is {} square pixels. and it's validity is {}",
        rect1.area(),
        rect1.is_valid()
    );

    dbg!(&rect1);

    println!("Rect1 is {rect1:#?}");

    println!(
        "Width of rect1 is {}",
        rect1.width(String::from("password"))
    );

    if rect1.can_fit_in_me(&rect2) {
        println!("rect1 can eat rect2");
    } else {
        println!("rect1 can't eat rect2");
    }
    if rect1.can_fit_in_me(&rect3) {
        println!("rect1 can eat rect3");
    } else {
        println!("rect1 can't eat rect3");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }
    fn width(&self, password: String) -> u32 {
        if password == "password" {
            self.width
        } else {
            0
        }
    }
    fn can_fit_in_me(&self, food: &Rectangle) -> bool {
        (self.width > food.width) && (self.height > food.height)
    }
    fn new_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
