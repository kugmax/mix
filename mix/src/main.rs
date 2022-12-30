pub mod memory;

fn main() {
    let content: i32 = 0b0_111111_111111_111111_111111_111111;

    let byte_1: i32 = 0b0_111111_000000_000000_000000_000000;
    let byte_2: i32 = 0b0_000000_111111_000000_000000_000000;
    let byte_3: i32 = 0b0_000000_000000_111111_000000_000000;
    let byte_4: i32 = 0b0_000000_000000_000000_111111_000000;
    let byte_5: i32 = 0b0_000000_000000_000000_000000_111111;

    println!("    {content} ");
    println!("1 = {byte_1} ");
    println!("2 = {byte_2} ");
    println!("3 = {byte_3} ");
    println!("4 = {byte_4} ");
    println!("5 = {byte_5} ");

    let access = content & byte_3;
    println!("get byte 3 {access} ");
    
    let minus = content * -1;
    let plus = minus * -1;
    
    println!("{minus} {plus}");
    
    println!("bits: {:#b}", minus);

    println!("{:#010b}", 1i8);  // 0b00000001
    println!("{:#018b}", 1i16); // 0b0000000000000001
    println!("{:#034b}", 1i32); // 0b00000000000000000000000000000001


    println!("{}, {}", 11 % 8, 11 - 8)
}
