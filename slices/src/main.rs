fn main() {
    // String slice from a String
    let action_hero: String = String::from("Jackie Chan");
    let first_name: &str = &action_hero[..6];
    println!("{first_name}");

    let last_name: &str = &action_hero[6..];
    println!("{last_name}");

    // String Slices and String Literals
    let second_first_name: &str = {
        let action_hero_2: &str = "Bruce Lee";
        &action_hero_2[0..5]
    };

    println!("{second_first_name}");

    // String Slices Length
    let food: &str = "pizza"; // emojis: 🍕⚡😂✅ usually have a bite size of 4 but a character usually has a byte size of 1
    let pizza_slice: &str = &food[0..3];
    println!("{}", food.len());
    println!("{}", pizza_slice.len());

    // String slices as function parameters
    do_hero_stuff(&action_hero);

    //Array Slices
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let mut my_slice: &[i32] = &values[..4];
    println!("{my_slice:?}");

    my_slice = &values[2..4];
    println!("{my_slice:?}");

    my_slice = &values[3..];
    println!("{my_slice:?}");

    my_slice = &values[..];  // this is still a slice but represents the whole original entity
    println!("{my_slice:?}");

    my_slice = &values;  // this is a refrence to a 6 element array of i32 and not a slice
    println!("{my_slice:?}");

    // Deref Coercion with array slices
    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[..3];
    print_length(slice_of_three);

    // Mutable Array Slices
    let mut my_array = [10, 15, 25, 30, 35];
    let sliced_array = &mut my_array[2..4];
    println!("Sliced Array: {:?}", sliced_array);

    sliced_array[0] = 100;
    println!("New Sliced Array: {:?}", sliced_array);
    println!("My New Array: {:?}",  my_array);
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

fn print_length(reference: &[i32]) {  // we can move from str -> String and not String -> str
    println!("{}", reference.len());
}