trait Currency {
    fn main_currency(&self) -> u64;
    fn sub_currency(&self) -> u64;
}

struct Euro {
    amount: u64
}

impl Currency for Euro {
    fn main_currency(&self) -> u64 {
        return self.amount/100;
    }

    fn sub_currency(&self) -> u64 {
        return self.amount%100;
    }
}

impl Euro {
    pub fn new(euro: u64, cent: u64) -> Euro{
        return Euro {amount: 100*euro+cent};
    }
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
    }


    #[test]
    fn euro_wraps_cent() {
        let e1 = super::Euro::new(10, 256);
        assert_eq!(e1.sub_currency(), 56);
        assert_eq!(e1.main_currency(), 12);
    }
}




