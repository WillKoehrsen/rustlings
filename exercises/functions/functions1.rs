// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me();
}

fn call_me() {
    println("Hello, World!");
}

fn println(s: &str) {
    // println! is a macro while println is a function
    println!("{}", s);
}