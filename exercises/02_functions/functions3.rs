// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    call_me(23);
}

fn call_me(num: u32) {
    // short note iterate with (`initial`..`terminal`){...}
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
