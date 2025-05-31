use std::error::Error;

use foo::add;
use num::rational::Ratio;


pub fn test() -> Result<(), Box<dyn Error>> {
    println!("Result: {}", add(1, 5));
    println!("Result: {}", add(1.1, 2.1));
    println!("Result: {}", add(1f32, 2f32));
    println!("Result: {}", add(add(Ratio::new(1, 3), Ratio::new(1, 3)), Ratio::new(1, 3)));

    Ok(())
}

