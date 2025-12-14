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
}
