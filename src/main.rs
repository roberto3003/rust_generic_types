fn main() {
    let number_list = vec![34, 50, 100, 65];

    let mut largerst = number_list[0];

    for number in number_list {
        if number > largerst {
            largerst = number;
        }
    }
    println!("The largest number is {}", largerst);
}
