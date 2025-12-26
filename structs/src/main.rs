struct Hotel {
    name: String,
    price: f64,
    is_booked: bool,
}

fn main() {
    // Defining a Struct
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    // Creating a Struct Instance
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };

    // Accessing the Struct fileds
    println!("My {} this morning cost ${}. It is {} that is was hot", mocha.name, mocha.price, mocha.is_hot);

    // Overwritting Struct fields
    let mut espresso = Coffee{
        name: String::from("Espresso"),
        price: 7.55,
        is_hot: true,
    };
    println!("My {} this morning cost ${}. It is {} that is was hot", espresso.name, espresso.price, espresso.is_hot);

    espresso.name = String::from("Macchiato");
    espresso.price = 6.99;
    espresso.is_hot = false;
    println!("My {} this morning cost ${}. It is {} that is was hot", espresso.name, espresso.price, espresso.is_hot);

    // Transfer of ownership to a variable 
    let hotel_room = book_hotel(String::from("Rickie"), 24.99, false);
    println!("I booked a space at {} hotels for about ${}. It was {} thst is was already booked", hotel_room.name, hotel_room.price, hotel_room.is_booked);
}

// Creating Structs in a Function
fn book_hotel(name: String, price: f64, is_booked: bool) -> Hotel {
    Hotel {
        name: name,
        price: price,
        is_booked: is_booked,
    }
}
