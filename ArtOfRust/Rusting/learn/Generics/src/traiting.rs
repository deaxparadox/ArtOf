use std::fmt::Debug;
use crate::generic::{Dog};

pub trait Rectangular {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_area(&self) -> i32;
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl Rectangular for Rectangle {
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_area(&self) -> i32 {
        self.width * self.height
    }
}


struct Square {
    length: i32,
}
impl Square {
    pub fn new(length: i32) -> Self {
        Self { length }
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

impl Rectangular for Square {
    fn get_width(&self) -> i32 {
        return self.length;
    }
    fn get_height(&self) -> i32 {
        self.length
    }
    fn get_area(&self) -> i32 {
        return self.length * self.length;
    }
    
}

pub fn square_and_rectangle() {
    let rect = Rectangle::new(2, 3);
    let square = Square::new(5);


    println!(
        "rect has width {}, height {}, the area {}",
        rect.get_width(), 
        rect.get_height(),
        rect.get_area()
    );
    println!(
        "square has length {} and area {}",
        square.get_length(),
        square.get_area()
    );
}



pub trait SelfDescribing {
    fn describe() -> String;
}

fn describe_type<T>() -> String
where 
    T: SelfDescribing
{
    T::describe()
}


#[derive(Clone, Debug)]
struct Pumpkin {
    mass: f64,
    diameter: f64,
}

impl Default for Pumpkin {
    fn default() -> Self {
        Self  {
            mass: 13.,
            diameter: 13.
        }
    }
}

pub fn main() {
    let big_pumpkin = Pumpkin {
        mass: 50.,
        diameter: 50.
    };

    println!("Big pumpkin: {:?}", big_pumpkin);
    println!("Cloned big pumpkin: {:?}", big_pumpkin.clone());
    println!("Default pumpkin: {:?}", Pumpkin::default());
}