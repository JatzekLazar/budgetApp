pub mod budget{

    use utils::currency::currency::Euro;
    use utils::currency::currency::Currency;
    use chrono::prelude::DateTime;
    use chrono::prelude::Utc;
    use chrono::prelude::NaiveDateTime;
    use std::str::FromStr;
    use models::transaction::transaction::Transaction;

    #[derive(Queryable)]
    pub struct Budget<T:Currency> {
        id: u64,
        name: String,
        planned: T,
        actual: T,
        transactions: Vec<Transaction<T>>

    }

    impl<T> Budget<T>
        where T: Currency {
        fn new(id: u64, name: &str, planned: T) -> Budget<T>{
            let nam = String::from(name);
            return Budget { id:id,
                name: nam,
                planned: planned,
                actual: T::new(0,0),
                transactions: Vec::new()};
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn budget_creation() {
            let planned = Euro::new(0, 0);
            let t : Budget<Euro> = super::Budget::new(1, "test", planned);
            assert_eq!(t.name, "test");
            assert_eq!(t.id, 1);
            assert_eq!(t.transactions.len(), 0)
        }

    }
}