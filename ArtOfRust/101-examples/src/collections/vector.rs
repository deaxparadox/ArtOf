/// Creating a new empty vector with `Vector::new()` method.
pub mod example_1 {

    /// Creating a new empty vector with `Vector::new()` method.
    pub fn main() {
        let v: Vec<i32> = Vec::new();
        
    }
}

/// Creating vector with initial value, using `vec!` macros
pub mod example_2 {

    /// Creating vector with initial value, using `vec!` macros
    pub fn main() {
        let v = vec![1, 2, 3];
    }
}

/// Updating a vector
pub mod example_3 {

    /// This function creates and update a vector.
    /// For this vector to be updated, we have to create this 
    /// vector as mutable using `mut` keyword.
    pub fn main() {
        let mut v = Vec::new();

        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
    }
}


/// Reading value from vector
pub mod example_4 {
    use std::vec;



    /// This function creates, and reads value from the vector
    /// using index.
    pub fn indexing() {
        let v = vec![1, 2, 3, 4, 5];

        let third = &v[2];

        println!("This third element is {third}");
    }

    /// `indexing_out_of_range` we will create a vector and try to 
    /// access a value from out of range index. 
    pub fn indexing_out_of_range() {
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
    }

    pub fn get_method() {
        let v = vec![1, 2, 3, 4, 5];

        let third = v.get(2);

        if let Some(value) = third {
            println!("This third element using get method: {value}");
        }
    }

    pub fn get_method_out_of_range() {
        let v = vec![1, 2, 3, 4, 5];

        let third = v.get(100);

        // if let Some(value) = third {
        //     println!("This third element using get method: {value}");
        // } else {
        //     println!("Invalid index!")
        // }


        match third {
            Some(value) => println!("This third element using get method: {value}"),
            None => println!("Invalid index!"),
        }

    }
}


/// Creating a new empty vector with `Vector::new()` method.
/// We are creating a variable which will reference a value from vector.
/// After that we try to push another value, and then finally printing the 
/// borrwed value.
/// 
/// This will not work because we are trying to use the borrwed value after `push` method,
/// rust ensure ownership and borroring rules
pub mod example_5 {

    /// Creating a new empty vector with `Vector::new()` method.
    pub fn test_incorrect() {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // v.push(2);

        println!("The first element is: {first}");

    }

    pub fn test_correct() {
        let mut v = vec![1, 2, 3, 4, 5];

        v.push(1);

        let first = &v[0];

        println!("The first element is: {first}");

        v.push(2);
    }
}


/// In this case we are going to iterate over a vector
pub mod example_6 {

    /// In this function we are goint to itereate over a immutable
    /// vector.
    pub fn iterate() {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    
    }

    /// This this function are are going to iterate over a immutable 
    /// function
    pub fn interate_mut() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    
    }
}