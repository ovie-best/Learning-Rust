fn main() {
    // Signed and unsigned integers
    let _eight_bit: i8 = -122;
    let _sixteen_bit_signed: i16 = -32500;
    let _sixteen_bit_unsigned: u16 = 65000;

    let _thirty_two_bit_signed: i32 = -246193803;
    let _thirty_two_bit_unsigned: u32 = 4000000000;

    let _some_value = 20u16;
    
    // You can also use underscores to make large numbers more readable
    let _another_value: u32 = 2_000_000_678;

    // internal data types depending on your computer architecture
    let _days: usize = 55; // this will be u64 on a 64-bit system and u32 on a 32-bit system
    let _years: isize = -15_000; // this will be i64 on a 64-bit system and i32 on a 32-bit system

    // Escape sequences in strings
    println!("Dear Emily,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said, \"I love you Romeo\"");

    // File path with backslashes
    let filepath = "c:\\My Documents\\new\\videos";
    
    // Using raw string literals to avoid escaping backslashes
    let filepath2 = r"c:\My Documents\new\videos";
    println!("File path: {}", filepath);
    println!("File path using raw string: {}", filepath2);

    // Methods
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space: &str = "            my content    ";
    println!("{}", empty_space.trim());

    println!("{}", value.pow(2));
    println!("{}", value.pow(3));

    let pi: f64 = 3.14159267890264838;
    println!("The current value of pi is {pi}");

    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());

    // format specifiers
    println!("The current value of pi is {pi:.4}");
    println!("The current value of pi is {:.3}", pi);

    // casting types with as keyword
    let miles_away = 50;
    let _miles_away_i8 = miles_away as i8;
    let _miles_away_u8 = miles_away as u8;

    let miles_away = 100.329032;
    let _miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;

    println!("miles_away: {}", miles_away_int);

    // Math Operations
    let addition = 5 + 4;
    let subtraction = 10 - 8;
    let multiplication = 3 * 4;
    let division = 5 / 3; 
    let remainder = 5 % 3; // modulus operator

    println!{"Addition: {} .\nSubtraction: {} .\nMultiplication: {} .\nDivision: {} .\nRemainder: {} .", addition, subtraction, multiplication, division, remainder};

    let floor_division = 5/3;
    let decimal_division = 5.0/3.0;
    println!("Floor division: {} . Decimal division: {} .", floor_division, decimal_division);

    // Augmented assignment operators
    let mut year = 2025;
    // year = year + 1;
    year += 1;
    println!("The new year is {}", year);

    year -= 5;
    println!("Five years ago, it was {}", year);

    year *= 2;
    println!("In two centureies, it will be {}", year);

    year /=3;
    println!("After dividing by three, the year is {}", year);

    // Boolean data type
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {}. Silly: {}", is_handsome, is_silly);

    let age: i32 = 21;
    let is_young = age < 35;
    print!("Is young: {}.\n", is_young);
    println!("{} {}", age.is_positive(), age.is_negative());

    // Boolean Inversion
    println!("{}", !true);
    println!("{}", !false);

    let can_see_rated_r_movie = age >= 17;
    let cannot_see_r_movie = !can_see_rated_r_movie;
    println!("I am {age} yaears old. Can I see an R rated movie? {}. Cannot see R rated movie: {}.", can_see_rated_r_movie, cannot_see_r_movie);

    // Equality and Inequality Operators
    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "Coke");
    println!("{}", "Coke" == "coke"); 

    println!("{}", 13 == 13);
    println!("{}", 13 != 13);
    println!("{}", 13 == 14);

    println!("{}", true == false);
    println!("{}", true != false);

    // AND logic
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    // OR Logic
    let user_has_paid_for_subscription = true;
    let user_is_admin = true;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Can this user see my site? {}", user_can_see_premium_experience);

    // character data type
    let first_initial = 'B';
    let emoji: char = '⚡';
    println!("{} {}", first_initial.is_alphabetic(), emoji.is_alphabetic());
    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());

    // Array data type
    let _numbers: [i32; 6] = [4, 8, 15, 16, 32, 42];
    let apples = ["Red Delicious", "Granny Smith", "Fuji", "Gala"];
    println!("Length: {}", apples.len());

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];
    let first_season =seasons[0];
    let second_season = seasons[1];
    println!("First season: {}. Second season: {}.", first_season, second_season);

    seasons[2] = "Autumn";
    println!("Third season: {}", seasons[2]);
    // println!("Updated seasons: {:?}.", seasons);

    // Traits
    // println!("{}", seasons); -- this will throw an error because arrays do not implement the Display trait
    println!("{:?}", seasons); // using the Debug trait to print the array
    println!("{:#?}", seasons); // pretty print the array
    
    // Debugging with dbg! macro
    dbg!(2 + 2);
    dbg!(seasons);

    // Tuple data type
    let employee = ("Molly", 32, "Marketing"); 
    let name = employee.0;
    let age = employee.1;
    let department = employee.2;
    println!("Name: {}. Age: {}. Department: {}.", name, age, department);
    print!("{employee:?}");

    // Destructuring a tuple
    let (name, age, department) = employee;
    println!("\nName: {}. Age: {}. Department: {}.", name, age, department);

    let month_days = 1..31;
    let _leap_month_days = 1..=31;
    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letters = 'b'..'k';
    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Blue", "Yellow"];
    for color in colors {
        println!("{color} is a great color")
    }

    // Generics with ranges
    let alphabets: std::ops::Range<char> = 'a'..'z';    
    for cha in alphabets {
        println!("{cha}");
    }
}
