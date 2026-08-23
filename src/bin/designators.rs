macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            for i in 0..3 {
                println!("{}: You called {}()", i, stringify!($func_name));
            }
        }
    };
}

macro_rules! print_results {
    ($expression:expr) => {
        println!("{} = {}", stringify!($expression), $expression)
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
    print_results!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}
