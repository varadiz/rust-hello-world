fn main() {
    print_labeled_measurement(5, 'h');
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    // expression NOTE: no semicolon
    x + 1
}
