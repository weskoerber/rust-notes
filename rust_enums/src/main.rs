fn main() {
    {
        // Defining an enum...
        enum IpAddrKind {
            V4,
            V6,
        }

        // Assigning an enum to a value
        let _four = IpAddrKind::V4;
        let _six = IpAddrKind::V6;

        struct IpAddr {
            kind: IpAddrKind,
            addr: String,
        }

        let _ip = IpAddr {
            kind: IpAddrKind::V4,
            addr: String::from("127.0.0.1"),
        };

        // This code can be improved... See below
    }

    {
        // Enums can contain data
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        // Each enum can contain different data types
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));

        // Enums can contain any type: slices, structs, other enums... and more
    }

    {
        // This enum has 4 variants with different types
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8),
        }

        // Each enum can be written as an individual struct like this,
        // but we lose out on all variants living under the same enum;
        // for instance, we could easily pass a `Message` enum to a
        // function, but it's harder with the four individual structs
        // below.
        struct QuitMessage;
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String);
        struct ChangeColorMessage(u8, u8, u8);

        // We can define methods on enums!
        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("Hello, world!"));
        m.call();
    }

    {
        // The Option enum encodes a scenario in which a value could
        // be something or nothing; all while being safer than `null`.
        // It looks like this:
        //   enum Option<T> {
        //       None,
        //       Some(T),
        //   }

        // Some means the enum contains some valid data; None
        // essentially means the same thing as `null`
        let some_number = Some(5);
        let some_char = Some('w');
        let absent_number: Option<i32> = None;

        println!("{:?}", some_number);
        println!("{:?}", some_char);
        println!("{:?}", absent_number);

        let x: i8 = 5;
        let y: Option<i8> = Some(8);

        // This won't work. Even though the Option contains data of the
        // correct type to add the two values together, it's an Option
        // type; not the type it holds.
        // let sum = x + y;
    }
}
