fn main() {
    let mut x = 1024;
    loop {
        if x < 0 {
            break;
        }

        println!("{} no more runs to go", x);
        x -= 1;
    }
}
