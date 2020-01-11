use std::io;

fn main() {
    println!("Convert Temp from Fahrenheit to Celcius");
    println!("Input temperature");

    let mut degrees = String::new();

    io::stdin()
        .read_line(&mut degrees)
        .expect("Please input a number");

    let degrees: i32 = degrees.trim().parse().expect("please input a number");

    println!("Input temperature unit F or C");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Please input F or C");

    let result = convert_temp(degrees, &unit);
    if unit.trim() == "F" {
        println!("Temperature in Celsius is {}", result);
    } else {
        println!("Temperature in Fahrenheit is {}", result);
    }
}

fn convert_temp(d: i32, unit: &str) -> i32 {
    if unit.trim() == "F" {
        (d - 32) * 5 / 9
    } else {
        (d * 9 / 5) + 32
    }
}
