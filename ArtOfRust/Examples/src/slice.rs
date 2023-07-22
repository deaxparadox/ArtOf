pub fn main() {
    let s = String::from("Hello world!");

    // let hello = &s[0..5];
    // // let world = &s[6..11];          // does not include space
    // let world = &s[5..11];          // include space

    // println!("{}{}", hello, world);

    let rs = first_word(&s);
    println!("{}", rs);

    println!("{}", first_word(&String::from("paradox")));
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


pub fn slice_array() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}