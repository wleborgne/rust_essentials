static VAL: f32 = 3.2;

fn main() {
    let output = format!("{:+07.2}", VAL);
    println!("Print f32 value 3.2 as +003.20");
    println!("{}", output);
}
