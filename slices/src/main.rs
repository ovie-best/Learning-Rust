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
}
