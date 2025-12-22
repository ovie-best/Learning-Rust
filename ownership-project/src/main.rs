/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.
 
The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.
 
In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.
 
Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/



fn main() {
    let is_concert: bool = true;
    let is_event: bool = is_concert;
    println!("{is_concert}, {is_event}"); // ownership is not moved because bool implements the Copy trait

    let sushi: &str = "Salom";
    let dinner: &str = sushi;
    println!("{sushi}, {dinner}"); // ownership is not moved because string literals

    let breakfast: String = String::from("Tea and Bread");
    let meal: String = breakfast; 
    // println!("{breakfast}, {meal}"); // ownership is moved to meal, breakfast is no longer valid because String does not implement the Copy trait

    eat_meal(meal); // ownership of the "Salmon" String is moved to the eat_meal function
}

// function that takes ownership of a String parameter because String does not implement the Copy trait
fn eat_meal(mut meal: String) -> String {
    meal.clear();
    println!("Inside eat_meal function: {}", meal); // meal is cleared here
    return meal;
    // ownership of the cleared String is returned to the caller
}