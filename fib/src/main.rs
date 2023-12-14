use std::io;
fn main() {
    loop{
        println!("請輸入項數:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("讀取該行失敗");

        let number: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        println!("fib({}) = {}", number, fib(number));
        if number == 0{
            break;
        }
    }
}

fn fib(n: i32) -> i32{
    if n == 0{
        return 1;
    }

    if n <= 2{
        return n;
    }
    else{
        return fib(n-1) + fib(n-2);
    }
}
