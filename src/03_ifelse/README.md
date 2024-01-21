An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

simple if else statement

```rust
fn main(){

	let number = 3;

	if number <= 5 {
		println!("number is less then or equal to 5");
	} else {
		println!("number is more then 5");
	}
}
```

### Using if in a let statement

Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.

example:

```rust
fn main () {

	let condition = true
	let number = if condition { 5 } else { 6 }

	println!("the number is { number }", number)
}
```
