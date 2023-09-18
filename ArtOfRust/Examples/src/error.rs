pub mod error_1 {
    pub fn main() {
        panic!("crash and burn");
    }
}

pub mod error_2 {
    use std::fs::File;

    pub fn main() {
        let greeting_file_result = File::open("hello.txt");
        
    }
}

pub mod error_3 {
    use std::fs::File;

    pub fn main() {
        let greeting_file_result = File::open("hello.txt");
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
}

pub mod error_4 {
    use std::fs::File;
    use std::io::ErrorKind;

    pub fn main() {
        let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    }

}

pub mod unwrap_1 {
    use std::fs::File;
    pub fn main() {
        let greeting_File = File::open("hello.txt").unwrap();
    }
}