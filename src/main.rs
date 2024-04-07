// using method for area
//
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height > other.height
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

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect3 = Rectangle::square(25);
println!("the are no are in this ract1 {} ", rect3;
    println!("the are no are in this ract1 {} ", rect.can_hold(&rect1));
    println!("the are no are in this ract2 {} ", rect2.can_hold(&rect2));
    println!("the are no are in this land {} ", rect.area());
}

// // That's the dimensions
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the ractangle is {} square pixels.",
//         area(&rect)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// // That is simple function for multiply by
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "the are in my own proparty the ractangle is {} square miter",
//         are(width1, height1)
//     )
// }

// fn are(width: u32, height: u32) -> u32 {
//     width * height
// }
