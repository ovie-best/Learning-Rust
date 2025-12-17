fn main() {
    // Call the functions
    open_store("Lagos"); // Argument
    bake_pizza(20, "Pepperoni"); // Positional arguments
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Abuja");
    bake_pizza(15, "Mushroom");

   let result = square(5);
   println!("The square of 5 is {}", result);

   let result_cube = cube(3);
   println!("The cube of 3 is {}", result_cube);

   // unit type
   let _results = (); // a unit is an empty tuple

   let multiplier = 3;

   // block expression
  let calcuation: i32 = {
        let value = 5 + 4;
        value * multiplier
  };

   println!("The calculation result is {}", calcuation);
}

// Function definitions
fn open_store(neighborhood: &str) { // Parameter
    println!("Opening my pizza Store in {}!",neighborhood)
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {} {} pizzas!", number, topping);
}

fn swim_in_profit() {
    println!("Swimming in pizza profit!");
}

// explicit return
fn square(number: i32) -> i32 {
    return number * number;
}

// implicit return
fn cube(number: i32) -> i32 {
    number * number * number
}