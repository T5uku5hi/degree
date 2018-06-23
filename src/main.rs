use std::io;

fn main() {
    println!("▼▽▼ degree converter ▼▽▼");
    println!("(V)( ㆁᴗㆁ)(V) < Hi, I'm Ferris!");
    println!("(V)( ㆁᴗㆁ)(V) < Please input temperature value.");

    let mut value = String::new();
    io::stdin().read_line(&mut value)
        .expect("(V)( ㆁ-ㆁ)(V) < Failed to read line.");

    let _value: f64 = match value.trim().parse() {
        Ok(value) => value,
        Err(_) => panic!("(V)( ㆁ-ㆁ)(V) < Please input number.")
    };

    println!("(V)( ㆁᴗㆁ)(V) < Which degree your input value is Fahrenheit or Celsius? (Please Answer F or C.)");

    let mut degree = String::new();
    io::stdin().read_line(&mut degree)
        .expect("(V)( ㆁ-ㆁ)(V) < Failed to read line.");

    let _degree: String = match degree.trim().parse() {
        Ok(degree) => degree,
        Err(_) => panic!("(V)( ㆁ-ㆁ)(V) < Invalid input.")
    };

    if _degree == "F".to_string() {
        println!("(V)( ㆁᴗㆁ)(V) < Conversion is completed!");
        let celsius_value = convert_celsius(_value);
        println!("(V)( ㆁᴗㆁ)(V) < Before: {}°F, After: {}°C", _value, celsius_value);
    } else if _degree == "C".to_string() {
        println!("(V)( ㆁᴗㆁ)(V) < Conversion is completed!");
        let fahrenheit_value = convert_fahrenheit(_value);
        println!("(V)( ㆁᴗㆁ)(V) < Before: {}°C, After: {}°F", _value, fahrenheit_value);
    } else {
        println!("V)( ㆁ-ㆁ)(V) < Your degree is invalid. (Please Answer F or C.)");
    }
}

fn convert_celsius(value: f64) -> f64 {
    return (value - 32.0) / 1.8;
}

fn convert_fahrenheit(value: f64) -> f64 {
    return value * 1.8 + 32.0;
}