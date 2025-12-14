#![allow(unused_variables)] // Compiler directive to suppress warnings for unused variables

type Meters = i32; // Type alias for better code readability

// Constant declaration
const TAX_RATE: f64 = 7.25; 

fn main() {
   // Immutable variables
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    
    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples!",
        apples, oranges
    );

    // Mutable variable
    let mut gym_reps = 10;
    println!("I plan to do {} reps", gym_reps);

    gym_reps = 15;
    println!("I now plan to do {} reps", gym_reps);

    // Shadowing
    let _grams_of_protein = "100.345";
    let _grams_of_protein = 100.345;
    let mut  _grams_of_protein  = 100;
    _grams_of_protein = 105;

      println!("I need {} grams of protein", _grams_of_protein);

   // Variable scope
   let coffee_price = 5.99; // outer scope
   {
      println!("The price of coffee is ${}", coffee_price);

      let cookie_price = 1.99; // inner scope
      println!("The price of a cookie is ${}", cookie_price);
   }

   println!("The price of coffee is still ${}", coffee_price);

   // Constant
   let income = 100000;
   println!("The tax rate is {} and income is {}", TAX_RATE, income);

   // type alias
   let mile_race_length: Meters = 1600;
   let two_mile_race_length: Meters = 3200;
   println!("A one mile race is {} meters long and a two mile race is {} meters long", mile_race_length, two_mile_race_length);

   // Compiler directive
  ////////// #[allow(unused_variables)]
   let three_mile_race_length: Meters = 1600;
   let four_mile_race_length: Meters = 3200;
   
}
