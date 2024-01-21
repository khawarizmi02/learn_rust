// simple if else statement function

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("#############################");

    // if else statement in let statement
    println!("if else statement in let statement");
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
