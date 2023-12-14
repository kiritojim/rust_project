use std::io;

fn main() {
    loop {
        println!("請選擇轉換方向，輸入0為C=>F，輸入1為F=>C");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("讀取該行失敗");

        let command: u32 = match command.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        println!("請輸入溫度");
        let mut input_temperature = String::new();
        io::stdin()
            .read_line(&mut input_temperature)
            .expect("讀取該行失敗");

        let temperature: i32 = match input_temperature.trim().parse() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if command == 0 {
            let f = c_to_f(temperature);
            println!("攝氏: {}C 轉換為華氏: {}F", temperature, f);
            break;
        } else if command == 1 {
            let c = f_to_c(temperature);
            println!("華氏: {}F 轉換為攝氏: {}C", temperature, c);
            break;
        } else {
            println!("請重新輸入");
        }
    }
}

fn c_to_f(c: i32) -> i32 {
    c * 9 / 5 + 32
}

fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}
