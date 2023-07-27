#[derive(Debug)]
pub struct Color(i32, i32, i32);

#[derive(Debug)]
pub struct Point(i32, i32, i32);

#[derive(Debug)]
pub struct Mixed(Color, Point);

fn tuple_parameter(color: Color, point: Point) -> Mixed {
    Mixed(color, point)
}

pub fn main() {
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    let mixed: Mixed = dbg!(Mixed(Color(1, 2, 3), Point(0, 0, 0)));

    // let mixed: Mixed = tuple_parameter(Color(1, 2, 3), Point(0, 0, 0));

    println!(
        "Mixed Color: {}, {} and {}",
        mixed.0 .0, mixed.0 .1, mixed.0 .2
    )
}

pub mod tuple_information {

    #[derive(Debug)]
    pub struct Information {
        firstname: String,
        lastname: String,
        age: u32,
    }

    fn build_information(firstname: String, lastname: String, age: u32) -> Information {
        Information {
            firstname,
            lastname,
            age,
        }
    }

    pub fn main() {
        let person1 = build_information("Nitish".to_string(), "Kushwaha".to_string(), 25);

        let same_person: &Information = dbg!(&person1);



        
    }
}
