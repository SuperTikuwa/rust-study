macro_rules! Abs {
    ($n:expr) => {
        if $n < 0 {
            -$n
        } else {
            $n
        }
    };
}

fn main() {
    let n = Abs!(-2);

    println!("{}", n);
}
