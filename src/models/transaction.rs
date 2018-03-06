pub mod transaction{

    use utils::currency::currency::Euro;
    use utils::currency::currency::Currency;
    use chrono::prelude::DateTime;
    use chrono::prelude::Utc;
    use chrono::prelude::NaiveDateTime;
    use std::str::FromStr;

    pub struct Transaction<T: Currency> {
        id: u64,
        amount: T,
        explanation: String,
        date: DateTime<Utc>
    }

    impl<T> Transaction<T>
        where T: Currency {
        fn new(id: u64, amount: T, explanation: &str, date: DateTime<Utc>) -> Transaction<T>{
            let expl = String::from(explanation);
            return Transaction { id:id, amount: amount, explanation: expl , date:date};
        }
    }

    trait TransactionTrait {

    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn transaction_creation() {
            let e1 = Euro::new(10, 88);
            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
            let t = super::Transaction::new(1, e1, "test", dt);
            assert_eq!(t.amount.main_currency(), 10);
        }

    }
}