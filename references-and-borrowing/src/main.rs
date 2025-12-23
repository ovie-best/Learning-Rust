fn main() {
    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);


    // immutable references
    let car: String = String::from("Red Ferrari");
    let ref1: &String = &car;
    let ref2: &String = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // mutable refernces
    let mut bike: String = String::from("Green Jaguar");
    let _bikeref1: &mut String = &mut bike;
    let bikeref2: &String = &bike;
    // println!("{bikeref1} and {bikeref2}"); // both cannont exixt at the same time
    // a mutbale refrence does not implement the copy trait which means we can only have a single refrenc to it at a time
    println!("{bikeref2}");

    
    let coffee: String = String::from("Mocha");
    let a: &String = &coffee;
    let b: &String = a; 
    println!("{} and {}", a, b); // an immutable reference implements the copy trait

    
    let city: String = create_city();
    println!("{city}");

    
}

// immutable and mutable references

fn add_flour(meal: &mut String) {
    meal.push_str("Add Flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal Steps: {meal}");
}

// Dangling Reference
// fn create_city() -> &String {
//     let city: String = String::from("New York");
//     &city // the city variable goes out of scope here
// }


//Solution to Dangling Reference
fn create_city() -> String {
    String::from("New York")
}
