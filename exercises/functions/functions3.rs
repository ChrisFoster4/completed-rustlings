// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 1..num+1 {
        println!("Ring! Call number {}", i);
    }
}
