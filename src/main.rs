macro_rules! print_expr {
    ($expr:expr) => {
        println!("{:?} = {:?}", stringify!($expr), $expr);
    };
}

macro_rules! unless {
    ($cond:expr, $expr:expr) => {
        if !$cond {
            $expr;
        }
    };
    ($cond:expr, $expr:block) => {
        if !$cond {
            $expr
        }
    };
    ($cond:expr, $expr:stmt) => {
        if !$cond {
            $expr
        }
    };
}

fn main() {
    let x = 5;
    let y = 10;

    print_expr!(x);
    print_expr!(y);
    print_expr!(x + y);
    unless!(x >= y, print_expr!(x < y));
}
