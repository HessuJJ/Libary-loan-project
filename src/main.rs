fn main() {
    let mut loan = BookLoan::new("Rust Book", "Iines");
    loan.summary();

    loan.pass_day();
    loan.pass_day();
    loan.pass_day();
    loan.summary();
}

struct BookLoan {
    title: String,
    borrower: String,
    days_remaining: u32,
}

impl BookLoan {
    fn new(title: &str, borrower: &str) -> Self {
        Self {
            title: String::from(title),
            borrower: String::from(borrower),
            days_remaining: 14,
        }
    }

    fn summary(&self) {
        print!(
            "\nThe {} is borrowed by {} for {} more days.",
            self.title, self.borrower, self.days_remaining
        )
    }

    fn pass_day(&mut self) {
        self.days_remaining = self.days_remaining.saturating_sub(1)
    }
}
