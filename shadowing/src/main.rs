fn main() {
    let mut x = 5;
    x = x + 2;
    {
        let x = x * 4;
        println!("內部x = {x}");
    }
    println!("外部x = {x}");
}
