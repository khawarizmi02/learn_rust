// fn main(){
// 	let number : i32 = 5;
// 	let string = String::from("hello world");
// 	hello_function(number, string); // function call
// }

// // function declaration
// fn hello_function(number: i32, text: String){
// 	println!("this is function");
// 	println!("data: {}", number);
// 	println!("string: {}", text);
// }

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
