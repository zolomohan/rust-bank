struct Account {
    id: usize,
    balance: isize,
    holder: String,
}

impl Account {
    fn new(id: usize, holder: String) -> Self {
        return Account {
            id,
            holder,
            balance: 0,
        };
    }

    fn deposit(&mut self, amount: isize) -> isize {
        self.balance += amount;
        return self.balance;
    }

    fn withdraw(&mut self, amount: isize) -> isize {
        self.balance -= amount;
        return self.balance;
    }

    fn summary(&self) -> String {
        return format!(
            "{id} - {holder}: {balance}",
            id = self.id,
            holder = self.holder,
            balance = self.balance
        );
    }
}

struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        return Bank { accounts: vec![] };
    }

    fn open_account(&mut self, holder: String) {
        self.accounts
            .push(Account::new(self.accounts.len() + 1, holder));
    }

    fn total_balance(&self) -> isize {
        return self
            .accounts
            .iter()
            .map(|account| return account.balance)
            .sum();
    }

    fn deposit_to_account(&mut self, id: usize, amount: isize) {
        let index = self.accounts.iter().position(|a| a.id == id).unwrap();
        self.accounts[index].deposit(amount);
    }

    fn withdraw_from_account(&mut self, id: usize, amount: isize) {
        let index = self.accounts.iter().position(|a| a.id == id).unwrap();
        self.accounts[index].withdraw(amount);
    }

    fn display(&self) {
        for account in &self.accounts {
            println!("{summary}", summary = account.summary());
        }
    }
}

fn main() {
    let mut bank = Bank::new();
    bank.open_account(String::from("Mohan"));
    bank.open_account(String::from("Sanjana"));

    bank.deposit_to_account(1, 200);
    bank.withdraw_from_account(1, 100);

    bank.deposit_to_account(2, 200);
    bank.withdraw_from_account(2, 400);

    bank.display();

    let total_balance = bank.total_balance();
    println!("Total Balance: {total_balance}");
}
