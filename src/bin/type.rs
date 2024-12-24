use crate::primitive_type::boolean_type::boolean_type;
use crate::primitive_type::textual_type::{char_type, str_type};

fn main() {
    char_type();
    str_type();
}

pub mod primitive_type {
    pub mod boolean_type {
        /**
        布尔类型只有两个值：true 和 false。Rust 中的布尔类型使用 bool 表示。除了逻辑运算，布尔类型还可以用于比较操作，
        可以把 true 当作 1，false 当作 0，这样就可以进行大小比较了。
         */
        pub fn boolean_type() {
            let x = true;
            let y: bool = false;
            println!("x > y is {}", x > y);
            println!("x xor y is {}", x ^ y);
        }
    }

    pub mod numric_type {
        /**
        整数分为有符号和无符号两种类型。每种类型又分为不同的长度，包括：
        - 有符号：i8, i16, i32, i64, i128, isize
        - 无符号：u8, u16, u32, u64, u128, usize

        ### 指定常量类型
        默认情况下的数字常量类型为 i32，要改变这个类型，可以在数字后面加上类型标识符或者使用类型推断。

        ### Machine-dependent integer types
        isize 和 usize 类型依赖机器位数，往往其大小与指针大小相同，也就是可以通过这些类型来表示所有的内存地址。
         */
        pub fn integer_type() {
            let _default_integer = 1;
            let _default_integer_suffix = 1i64;
            let _default_integer_type: i64 = 1;

            let iseze_val = 1isize;
            println!("size of usize: {} bytes", size_of::<usize>());
            println!("size of isize value {} is {} bytes", iseze_val, size_of_val(&iseze_val));
        }

        /**
        浮点数类型默认为 f64，如果需要使用 f32 类型，需要在数字后面加上 f32 标识符。
        另外，因为浮点数采用 IEEE-754 标准使用二进制存储，所以可能会出现精度问题，不要直接比较浮点数是否相等，而是比较它们的差值是否在一个很小的范围内。
         */
        pub fn float_type() {
            let _default_float = 1.0; // f64
            let _default_float_suffix = 1.0f32; // f32
            let _default_float_type: f32 = 1.0; // f32

            let a = 0.1 + 0.2;
            let b = 0.3;
            println!("a: {:.20}", a);
            println!("b: {:.20}", b);
            println!("a == b: {}", a == b); // 可能是 false
            let a = 0.1 + 0.2;
            let b = 0.3;
            // let diff = (a - b).abs();
            // println!("diff: {:.20}", diff);
        }
    }

    pub mod textual_type {
        /**
        字符类型使用 char 表示，是一个 Unicode scalar value，占用 4 个字节。字符类型使用单引号表示，例如 'a'。
         */
        pub fn char_type() {
            let a = 'a';
            let 中 = '中';
            println!("the size of char 'a' is {} bytes", size_of_val(&a));
            println!("the size of char '中' is {} bytes", size_of_val(&中));
        }

        /**
        字符串类型使用 str 表示，但其底层实现并不是一个 `[char]` 数组，而是一个 `[u8]` 数组，编码方式为 UTF-8。
         */
        pub fn str_type() {
            let hello = "abcde"; // “abcde” 在 UTF-8 编码中每个字符占用 1 个字节
            let hello_world = "he中eh"; // “中” 在 UTF-8 编码中占用 3 个字节
            println!("the size of str '{}' is {} bytes", hello, hello.len()); // 5
            println!("the size of str '{}' is {} bytes", hello_world, hello_world.len()); // 7
        }
    }
}