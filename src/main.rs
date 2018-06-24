use std::io;
use std::str::FromStr;

fn main() {
    println!("++++++ Degree Converter ++++++");
    println!("(V)( ㆁᴗㆁ)(V) < Hi, I'm Ferris!");

    loop {
        println!("(V)( ㆁᴗㆁ)(V) < Please input temperature value.");

        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("(V)( ㆁ-ㆁ)(V) < Failed to read line.");

        let _value: f64 = match value.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!("(V)( ㆁᴗㆁ)(V) < Which degree your input value is Fahrenheit or Celsius? (Please Answer F or C.)");

        let mut degree = String::new();
        io::stdin().read_line(&mut degree)
            .expect("(V)( ㆁ-ㆁ)(V) < Failed to read line.");

        let _degree: Degrees = match degree.trim().parse() {
            Ok(degree) => degree,
            Err(_) => continue,
        };

        match _degree {
            Degrees::F => println!("(V)( ㆁᴗㆁ)(V) < Conversion is completed! Before: {}°F, After: {}°C", _value, convert_celsius(_value)),
            Degrees::C => println!("(V)( ㆁᴗㆁ)(V) < Conversion is completed! Before: {}°C, After: {}°F", _value, convert_fahrenheit(_value)),
            Degrees::Else => println!("V)( ㆁ-ㆁ)(V) < Your degree is invalid. (Please Answer F or C.)")
        }
        println!("++++++ Repeat ++++++")
    }
}

enum Degrees {
    F,
    C,
    Else
}

impl FromStr for Degrees {
    type Err = ();

    fn from_str(s: &str) -> Result<Degrees, ()> {
        match s {
            "F" => Ok(Degrees::F),
            "C" => Ok(Degrees::C),
            _ => Ok(Degrees::Else),
        }
    }
}

fn convert_celsius(value: f64) -> f64 {
    (value - 32.0) / 1.8
}

fn convert_fahrenheit(value: f64) -> f64 {
    value * 1.8 + 32.0
}