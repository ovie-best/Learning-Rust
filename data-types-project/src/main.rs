/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/
fn main() {
    let number: i32 = 1_337; 
    let number_2 = number as i16;
    println!("Casted Number: {}", number_2);

    let float_number = 78.5690;
    println!("Float number to 3dp: {:.3}", float_number);

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    println!("Is my type of coffee: {}", is_my_type_of_coffee);

    let is_acceptable_coffee = with_milk || with_sugar;
    println!("Is an acceptable coffee: {}", is_acceptable_coffee);

    let scores: [i8; 4] = [19, 99, 8, 74];
    dbg!(scores); // or
    println!("{:?}", scores);

    let my_tuple = (18, 9.678, scores);
    dbg!(my_tuple);
  
}
