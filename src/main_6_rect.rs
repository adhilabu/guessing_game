#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.height * self.width
    }

    fn can_hold(&self, rect: Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn main() {
    // let width: u32 = 5;
    // let height = 8;
    // let rect: (u32, u32) = (5, 3);
    let rect1 = Rectangle {
        width:4,
        height:5
    };

    let rect2 = Rectangle {
        width:2,
        height:2
    };

    let rect3 = Rectangle {
        width:10,
        height:10
    };

    println!("Rectangle 1 is able to hold Rectangle 2 is {}", rect1.can_hold(rect2));
    println!("Rectangle 1 is able to hold Rectangle 3 is {}", rect1.can_hold(rect3));

    println!("{:#?}", rect1);

    print!(
        "Area of the rectangle is {}",
        rect1.area()
    );
}

// fn area (rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
// fn area (dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }