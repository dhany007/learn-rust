fn main() {
    println!("Generic Types");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.1, y: 2.2 };

    // don't work because mix integer and float, should be same struct T
    let wont_work = Point { x: 1, y: 2.2 };

    // work because have generic T and U
    let integer_and_float = Point { x: 1, y: 2.2 };

    let p = Point { x: 1, y: 2.2 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    // same is Point<f32>
    let po = Point{x:1.2 , y:3.4};
    let result = po.distance_from_origin();

    let p1 = Point3{x:1.2, y:3.4};
    let p2 = Point3{x:"Hello", y:'c'};

    let p3 = p1.mix(p2);
    println!("p3.x = {}, p3.y = {}", p3.x(), p3.y());
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic in struct definition
// type of T is must be same
struct Point<T> {
    x: T,
    y: T,
}

// can different because have T and U
struct Point2<T, U> {
    x: T,
    y: U,
}

// generic in enum definition
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// generic in method definition
// implement method for struct Point above
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// the type Point<f32> will have a distance_from_origin method
// other instances of Point<T> where T is not of type f32 will not have this method defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point3<X1, Y1> {
    fn mix<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}