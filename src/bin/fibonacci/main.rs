// compute fib. for given value.
// not going to bother with stdin parsing for this one
// should just be easy recursion

const FIB_SEED_ZERO: u64 = 0;
const FIB_SEED_ONE: u64 = 1;

fn fib(i: u64) -> u64 {
    match i {
        0 => FIB_SEED_ZERO,
        1 => FIB_SEED_ONE,
        _ => fib(i - 1) + fib(i - 2),
    }
}

const INPUT_VALUE: u64 = 4;

fn main() {
    println!("{}", fib(INPUT_VALUE));
}
