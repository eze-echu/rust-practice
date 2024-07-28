#[derive(Debug, Clone)]
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
    fn add_balance(&mut self, amount: isize) {
        self.balance += amount;
    }
    fn remove_balance(&mut self, amount: isize) {
        self.balance -= amount;
    }
    fn account_summary(&self) -> String {
        format!("Summary of Account({:?}) of {:?}: {:?}",
                self.id, self.holder.as_str(), self.balance).to_string()
    }
}

#[derive(Debug, Clone)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
    fn add_account(&mut self, acc: Account) {
        self.accounts.push(acc);
    }
    fn calculate_total(&self) -> isize {
        // let mut sum: isize = 0;
        // for acc in &self.accounts {
        //     sum += acc.balance;
        // }
        // return sum;
        self.accounts.iter().map(|account: Account| account.balance).sum() //lambda my beloved
    }
    fn summarize_all_accounts(&self) -> Vec<String> {
        // let mut temp = vec![];
        // for acc in &self.accounts {
        //     temp.push(acc.account_summary());
        // }
        // temp
        self.accounts.iter().map(|x: Account| x.account_summary()).collect::<Vec<String>>()
        // what the actual heck?
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
    let mut account = Account::new(0, "Vos");
    bank.add_account(account);
    let mut account = Account::new(1, "Yo");
    bank.add_account(account);
    bank.accounts[0].add_balance(442);
    bank.accounts[1].add_balance(2506);
    // account.id = 1; // we can modify the value
    // let reference = &account; // we create the borrow
    // //account.id = 3; // we try to edit the original
    // println!("{:#?}", reference); // if previous line is uncommented, code fails
    //bank.accounts.push(Account::new(account.id, account.holder.as_str()));
    println!("{:#?}", bank);
    println!("Total of accounts: {:?}", bank.calculate_total());
    println!("List of Balances: {:#?}", bank.summarize_all_accounts());
}
