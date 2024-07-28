#[derive(Debug)]
struct Account {
    id: usize,
    balance: isize,
    holder: String,
}
impl Account {
    fn new(id: usize, holder: &str) -> Self {
        Account {
            id,
            holder: holder.to_string(),
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}
// the & is a reference or a "borrow", it points to the already existing pointer, without moving it
// white a borrow exists, the original value CANNOT be modified
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let mut bank = Bank::new();
    println!("{:#?}", bank);
    let mut account = Account::new(0, "Yo");
    print_account(&account);
    // account.id = 1; // we can modify the value
    // let reference = &account; // we create the borrow
    // //account.id = 3; // we try to edit the original
    // println!("{:#?}", reference); // if previous line is uncommented, code fails
    bank.accounts.push(Account::new(account.id, account.holder.as_str()));
    println!("{:#?}", bank);
}
