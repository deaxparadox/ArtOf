pub mod define {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    fn main() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from(":;1"),
        };
    }
}

pub mod enum_variable {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub fn main() {
        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));

        // match home {
        //     IpAddr::V4(a, b, c, d) => {
        //         println!("a: {a}, b: {b}, c: {c}, d: {d}")
        //     },
        //     IpAddr::V6(value) => {
        //         println!("Version: {value}")
        //     }
        // }
    }
}


pub mod Example {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
}

pub mod Methods {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            
        }
    }

    fn main() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }
}