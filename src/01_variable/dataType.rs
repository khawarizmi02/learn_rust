// data type
// 1. integer
// 2. float
// 3. boolean
// 4. character
// 5. tuple
// 6. array
//

fn main() {
	// integer
	let x = 5;
	println!("The value of x is: {}", x);

	// float
	let y = 2.0;
	println!("The value of y is: {}", y);

	// boolean
	let t = true;
	println!("The value of t is: {}", t);

	// character
	let c = 'c';
	println!("The value of c is: {}", c);

	// tuple
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	println!("The value of tup is: {:?}", tup);

	// array
	let arr = [1, 2, 3, 4, 5];
	println!("The value of arr is: {:?}", arr);
}