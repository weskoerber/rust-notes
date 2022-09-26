struct Rect {
    width: f32,
    height: f32,
}

// Methods are defined using the impl keyword
impl Rect {
    // The first argument for a method is &self; self is an alias for the type we're implementing
    // the method on - in this case Rect
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0.0
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an associated function, not a method; it doesn't take a reference to the struct;
    // they are commonly used as constructors
    fn square(size: f32) -> Self {
        Rect {
            height: size,
            width: size,
        }
    }
}

// Multiple impl blocks are allowed
impl Rect {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r1 = Rect {
        width: 0.21,
        height: 99.887,
    };

    println!("The area of r1 is {}", r1.area());

    // Methods and fields can have the same name
    if r1.width() {
        println!("r1 has a non-zero width: {}", r1.width);
    }

    let r2 = Rect {
        width: 0.01,
        height: 100.5,
    };

    if r1.can_hold(&r2) {
        println!("r1 can hold r2");
    } else {
        println!("r1 cannot hold r2");
    }

    let s1 = Rect::square(1.0);
    println!("s1 dimensions: {} x {}", s1.width, s1.height);

    if r1.is_square() {
        println!("r1 is square");
    }

    if s1.is_square() {
        println!("s1 is square");
    }
}
