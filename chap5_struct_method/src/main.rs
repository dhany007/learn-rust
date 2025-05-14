fn main() {
    println!("Method Syntax!");
    // Methods are similar to functions: we declare them with the fn keyword and a name,
    // they can have parameters and a return value,
    // and they contain some code that’s run when the method is called from somewhere else.
    // methods are defined within the context of a struct and their first parameter is always self,
    // which represents the instance of the struct the method is being called on.

    println!("Defining Methods");
    let rect1 = Rectangle {
        width: 50,
        height: 30
    };
    let result = rect1.area();
    println!("The area of the rectangle is {} square pixels", result);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
