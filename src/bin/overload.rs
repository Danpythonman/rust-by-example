macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{} and {} is {}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{} or {} is {}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
    ($left:expr; nonsense $right:expr) => {
        println!("this is nonsense");
    };
}

fn main() {
    test!(1 == 2; and 3 == 3);
    test!(1 == 2; or 3 == 3);
    test!(1 == 2; nonsense 3 == 3);
}
