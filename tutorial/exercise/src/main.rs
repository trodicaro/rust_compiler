// Code from "Types and Values" of "Comprehensive Rust"
// https://google.github.io/comprehensive-rust/types-and-values.html

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b + b*c + c*a;
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn fib(n: u32) -> u32 {
    if n < 2 {
	return 1;
	// todo!("Implement this")
    } else {
	return fib(n-1) + fib(n-2);
	// todo!("Implement this")
    }
}

fn main() {
    println!("Hello cruel world!");
    let x: i32 = 10;
    println!("x: {x}");

    // Rust variables are immutable by default. This will generate an
    // error unless I use "let mut x: i32 = 10;" above.
    // x = 20;
    println!("result: {}", interproduct(120, 100, 248));

    // Rust will infer types from usage.
    let a = 10;
    let b = 20;
    takes_u32(a);     // --> a inferred to be u32
    takes_i8(b);      // --> b inferred to be i8
    // takes_u32(b);  // --> Error. b is an i8

    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
