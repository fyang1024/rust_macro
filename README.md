# rust_macro

Rust 里的宏定义和 C 预言里的宏定义类似，都是在预编译时处理的，编译的代码都是宏展开后的代码。
下面是运行`cargo expand`的结果

```Rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let x = 5;
    let y = 10;
    {
        ::std::io::_print(format_args!("{0:?} = {1:?}\n", "x", x));
    };
    {
        ::std::io::_print(format_args!("{0:?} = {1:?}\n", "y", y));
    };
    {
        ::std::io::_print(format_args!("{0:?} = {1:?}\n", "x + y", x + y));
    };
}
```
