use std::io; //not sure this is necessary, but I can't tell now cause I'm testing a very large number

fn main() {
    const n: u64 = 2305843008139952128; //eventually planning to change to mut u64
    let mut s = 0;
    
    //will worry about input later
    //println!("Input the number: ");

    //io::stdin()
        //.read_line(&mut n)
        //.expect("Failed to read input");

    for i in 1..n {
        if (n % i) == 0 { //this checks for all of a number's divisors
            s = s + i; //this adds the divisor to the total
        }
    }

    if s == n { //output for perfect numbers
        println!("That is a perfect number");
    }

    else { //output for non-perfect numbers
        println!("That is not a perfect number");
    }
}