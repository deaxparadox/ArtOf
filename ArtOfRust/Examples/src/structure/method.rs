struct Rectangle {
    width: u32,
    height: u32,
}

pub mod normal {
    use crate::st::method::Rectangle;

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn double_width(&mut self) {
            self.width *= 2;
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            // self.width > other.width && self.height > other.height
            // or 
            self.area() > other.area()
        }
    }

    pub fn main() {
        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        // rect1.double_width();

        println!("Width: {}", rect1.width);

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}


pub mod assciated_functions {
    use crate::st::method::Rectangle;

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size, height: size
            }
        }
    }
}


pub mod multple_impl_blocks {
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
}