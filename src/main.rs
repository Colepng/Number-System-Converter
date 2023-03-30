use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let mut input_base = String::new();
    let mut input_base2 = String::new();
    let mut verbouse = String::new();
    
    print!("Would you like the program to be verbouse (y/n): ");
    let _ = stdout().flush();
    stdin().read_line(&mut verbouse)?;
    print!("Enter the number you want to convet: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input)?;
    print!("Enter the base of the number you are converting from: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input_base)?;
    print!("Enter the base you want to convert to: "); 
    let _ = stdout().flush();
    stdin().read_line(&mut input_base2)?;

    let input: &str = input.trim();
    let input_base: &str = input_base.trim();
    let input_base2: &str = input_base2.trim();
    let verbouse: &str = verbouse.trim();

    println!("Converting from {input} in base {input_base} to base {input_base2}");
    let mut input_digits: Vec<isize> = Vec::new();
    let x = input_base.parse::<u32>().unwrap();

    let input_decimal: isize;
    
    if verbouse == "y" {

        // check if base is not 10 because if it is there is no need to convet to base 10
        if input_base.parse::<usize>() != Ok(10) { 
            for i in input.chars() {
                input_digits.push(i.to_digit(x).expect("NO number stupid humab") as isize);
            }

            // a massive one liner to convet all the nums in input_digits and find what they are in base
            // 10, then sums then all 
            input_decimal = input_digits.iter().rev().enumerate().map(|(index, digit)| { digit * input_base.parse::<isize>().expect("input_base is not an valid isize").pow(index as u32) }).sum();
            println!("{:#?}", input_digits);
            println!("input as decimal: {input_decimal}");
        } else {
            input_decimal = input.parse::<isize>().expect("not an iszie");
        }

        let mut temp: isize = input_decimal;
        let mut list_of_remainders: Vec<isize> = Vec::new();
        let input_base2: isize = input_base2.parse::<isize>().unwrap();
        while temp != 0 {
            list_of_remainders.push(temp%input_base2);
            temp /= input_base2;
            println!("{temp}");
        }
        println!("{:#?}", list_of_remainders);

        let mut output: String = String::new();
        for i in list_of_remainders {
           output.push(char::from_digit(i as u32, input_base2 as u32).unwrap());
        }
        let output: String = output.chars().rev().collect::<String>();
        println!("The output is {output}");
    } else {
       // check if base is not 10 because if it is there is no need to convet to base 10
        if input_base.parse::<usize>() != Ok(10) { 
            for i in input.chars() {
                input_digits.push(i.to_digit(x).expect("NO number stupid humab") as isize);
            }

            // a massive one liner to convet all the nums in input_digits and find what they are in base
            // 10, then sums then all 
            input_decimal = input_digits.iter().rev().enumerate().map(|(index, digit)| { digit * input_base.parse::<isize>().expect("input_base is not an valid isize").pow(index as u32) }).sum();
        } else {
            input_decimal = input.parse::<isize>().expect("not an iszie");
        }

        let mut temp: isize = input_decimal;
        let mut list_of_remainders: Vec<isize> = Vec::new();
        let input_base2: isize = input_base2.parse::<isize>().unwrap();
        while temp != 0 {
            list_of_remainders.push(temp%input_base2);
            temp /= input_base2;
        }

        let mut output: String = String::new();
        for i in list_of_remainders {
           output.push(char::from_digit(i as u32, input_base2 as u32).unwrap());
        }
        let output: String = output.chars().rev().collect::<String>();
        println!("The output is {output}");
 
    }

    Ok(())
}
