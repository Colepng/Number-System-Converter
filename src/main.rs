use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut input_base = String::new();
    let mut input_base2 = String::new();

    io::stdin().read_line(&mut input)?;
    io::stdin().read_line(&mut input_base)?;
    io::stdin().read_line(&mut input_base2)?;

    let input: &str = input.trim();
    let input_base: &str = input_base.trim();
    let input_base2: &str = input_base2.trim();

    println!("Converting from {input} in base {input_base} to base {input_base2}");
    let mut input_digits: Vec<isize> = Vec::new();
    let x = input_base.parse::<u32>().unwrap();

    let input_decimal: isize;


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

    Ok(())
}

fn to_decimal() {

}
