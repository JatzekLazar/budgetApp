pub mod currency {

pub trait Currency {
    fn new(big:u64, small:u64) -> Self;
    fn main_currency(&self) -> u64;
    fn sub_currency(&self) -> u64;
    fn values(&self) -> (u64, u64);
}

pub struct Euro {
    amount: u64
}

impl Currency for Euro {
    fn new(big: u64, small: u64) -> Euro{
        return Euro {amount: 100*big+small};
    }

    fn main_currency(&self) -> u64 {
        return self.amount/100;
    }

    fn sub_currency(&self) -> u64 {
        return self.amount%100;
    }

    fn values(&self) -> (u64, u64) {
        return (self.main_currency(), self.sub_currency());
    }
}

impl Euro {

}

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn euro_right_creation() {
            let e1 = super::Euro::new(10, 88);
            assert_eq!(e1.amount, 1088);
            assert_eq!(e1.main_currency(), 10);
            assert_eq!(e1.sub_currency(), 88);
            assert_eq!(e1.values(), (10, 88))
        }


        #[test]
        fn euro_wraps_cent() {
            let e1 = super::Euro::new(10, 256);
            assert_eq!(e1.amount, 1256);
            assert_eq!(e1.sub_currency(), 56);
            assert_eq!(e1.main_currency(), 12);
            assert_eq!(e1.values(), (12, 56))
        }
    }

}






