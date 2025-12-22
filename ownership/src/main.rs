

// fn main() {
//     let age: i32 = 33;
//     let is_handsome: bool = true;


//     println!("{}", age);
//     println!("{}", is_handsome)
//     // age variable exists here
// } // is_handsome goes out of scope, then age variable goes out of scope here

fn main() {
    let time: i32 = 2025;
    let year: i32 = time;
    println!("The time is {} and the year is {}", time, year);

    // The string types
    let _food: &str = "pasta"; // string slice type
    let _text: String = String::new(); // String type
    let _candy: String = String::from("Kitkat");

    let mut name: String = String::from("Ovie");
    name.push_str(" Nathaniel");
    println!("Hello, my name is {}", name);

    // Ownership and Move Semantics
    let person: String = String::from("John");
    
    let genius: String = person.clone();

    println!("Cloned Person: {}", person);

    // println!("Person: {}", person); // This will cause an error because `person` has been moved to `genius`
    println!("Genius: {}", genius);

    let my_stack_value: i32 = 2; // stack value
    let my_integer_reference: &i32 = &my_stack_value; // reference to the stack value
    println!("{}", *my_integer_reference); // dereferencing the stack reference

    let my_heap_value: String = String::from("Toyota"); // heap value
    let my_heap_reference: &String = &my_heap_value; // reference to the heap value
    println!("{}", *my_heap_reference); // dereferencing the heap reference

    // there are two copies or owners of the string "Cookies and Cream"
    // i.e no movement of ownership occurs here
    let ice_cream: &str = "Cookies and Cream";
    let dessert: &str = ice_cream;
    println!("{} {}", ice_cream, dessert);

    // no movement of ownership occurs here because i32 implements the Copy trait
    let apples: i32 = 6;
    print_my_value(apples);
    println!("{apples} is still accessible here because i32 implements the Copy trait");

    // ownership is moved to the function because String does not implement the Copy trait
    let fruit: String = String::from("Mango");
    print_my_fruit(fruit);
    // println!("{}", fruit); // This will cause an error because `fruit` has been moved

    let burger: String = String::from("Burger");
    mutable_parameter(burger);

    let cake: String = bake_cake();
    println!("I now have a {cake} cake!")
} // drop(genius) is called here


fn print_my_value(value: i32) {
    println!("My value is: {}", value)
}

fn print_my_fruit(fruit: String) {
    println!("My fruit is: {}", fruit);
}

fn mutable_parameter(mut meal: String) {
    meal.push_str(" and Fries");
}

fn bake_cake() -> String {
    let cake: String = String::from("Chocolate Mousse");
    return cake;
}