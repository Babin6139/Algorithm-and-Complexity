use lab4::knap_sack;

fn main() {
    let i: i32 = 100;
    let m: i32 = 500;
    knap_sack(&[100, 200, 400, 500, 600, 700, 800, 900], i, m);
    let i: usize = 9;
    println!("{:b}", i);
}
