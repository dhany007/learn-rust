fn main() {
    println!("Closures!");
    /*
        If the person chosen for a free shirt has their favorite color set,
        they get that color shirt.
        If the person hasn’t specified a favorite color,
        they get whatever color the company currently has the most of.
     */

    let store = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
        ]
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1,
    );


    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2,
    );

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 7 },
    ];

    // list.sort_by_key(|r| r.width);
    // println!("{list:#?}");

    let mut num_sort_operation = 0;
    list.sort_by_key(|r| {
        num_sort_operation += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operation} operations");
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts : Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

