// 这是一个简单的宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()`表示此红接受任何参数
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!()
}
