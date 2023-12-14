fn main() {
    let x = 3.5;

    let y: f32 = 2.7;

    let sum = 5 + 10;

    // 減法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 結果爲 -1

    // 取餘
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // 型別詮釋的方式

    let c = 'z';
    let z: char = 'ℤ'; // 明確標註型別的寫法
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("y 的數值為：{y}");

    println!("{heart_eyed_cat}");
}
