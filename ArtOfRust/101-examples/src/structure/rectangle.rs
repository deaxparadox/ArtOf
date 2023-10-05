pub mod rectangle {
    pub fn main() {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The read of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}


pub mod rectangle_with_tuples {
    pub fn main() {
        let rect1 = (30, 50);

        println!(
            "The read of the rectangle is {} square pixels.",
            area(rect1)
        );
    }

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}


pub mod rectangle_with_struct {
    struct Reactangle {
        width: u32,
        height: u32
    }

    pub fn main() {
        let rect1 = Reactangle {height: 30, width: 50};

        println!(
            "The read of the rectangle is {} square pixels.",
            area(&rect1)
        );

        // this works because struct `rect1` is passed as reference
        // if we don't use reference we will get `move` error
        println!("Height: {}, Width: {}", rect1.height, rect1.width);
    }

    fn area(dimensions: &Reactangle) -> u32 {
        dimensions.width * dimensions.height
    }
}