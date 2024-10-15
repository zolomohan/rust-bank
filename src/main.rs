#[derive(Debug)]
struct Account {
    id: usize,
    balance: isize,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

fn main() {
    println!("Hello, world!");
}
