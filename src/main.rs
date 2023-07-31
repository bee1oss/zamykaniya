fn main() {
    let is_even = |a| -> bool{
        println!("{}", a);
        a % 2 == 0
    };

    println!("{}", is_even(8));
}
