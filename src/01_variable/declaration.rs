// simple variable
// fn main() {
// 		let x = 5;
// 		println!("The value of x is: {}", x);
// }

// shadowing method
// fn main() {
// 		let x = 5;

// 		{
// 				let x = 10;
// 				println!("The value of x + 10 is: {}", x);
// 		}

// 		let x = x + 1;
// 		println!("The value of x + 1 is: {}", x);
// }

// cant combine two different data type
// fn main() {
//     let number = "T-H-R-E-E"; // don't change this line
//     println!("Spell a Number : {}", number);
//     number = 3; // don't rename this variable
//     println!("Number plus two is : {}", number + 2); // error can happen here
// }

// global variable
const NUMBER : i32 = 3;
fn main() {
		println!("Spell a Number : {}", NUMBER);
		println!("Number plus two is : {}", NUMBER + 2);
}