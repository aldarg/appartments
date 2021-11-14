use std::io;

fn main() {
    println!("Enter number of entrances:");
    let mut entrances_count = String::new();
    io::stdin().read_line(&mut entrances_count);

    println!("Enter number of floors per entrance:");
    let mut floors_count = String::new();
    io::stdin().read_line(&mut floors_count);

    println!("Enter number of appartments per floor:");
    let mut appartments_count = String::new();
    io::stdin().read_line(&mut appartments_count);

    println!(
        "So you have {0} entrances, {1} floors and {2} appartments on each floor",
        entrances_count.trim(),
        floors_count.trim(),
        appartments_count.trim()
    );
}
