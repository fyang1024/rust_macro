macro_rules! print_expr {
    ($expr:expr) => {
        println!("{:?} = {:?}", stringify!($expr), $expr);
    };
}

fn main() {
    let x = 5;
    let y = 10;

    print_expr!(x);
    print_expr!(y);
    print_expr!(x + y);
}
