fn main() {
    println!("Method Syntax!");
    // Methods are similar to functions: we declare them with the fn keyword and a name,
    // they can have parameters and a return value,
    // and they contain some code that’s run when the method is called from somewhere else.
    // methods are defined within the context of a struct and their first parameter is always self,
    // which represents the instance of the struct the method is being called on.

    println!("Defining Methods");
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let result = rect1.area();
    println!("The area of the rectangle is {} square pixels", result);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Methods with More Parameters");
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    println!("Associated Functions");
    // All functions defined within an impl block are called associated functions
    // because they’re associated with the type named after the impl.
    // like String::from("")
    let sq1 = Rectangle::square(30);
    println!("{:?}", sq1); // Rectangle { width: 30, height: 30 }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // We chose &self here : we don’t want to take ownership,
    // and we just want to read the data in the struct, not write to it
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can choose to give a method the same name as one of the struct’s fields.
    // For example, we can define a method on Rectangle that is also named width:
    fn width(&self) -> bool {
        self.width > 0
    }

    // methods with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions
    // like String::from("")
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// multiple impl block
impl Rectangle {
    fn area2(&self) -> u32 {
        return self.width * self.height;
    }
}