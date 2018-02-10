trait Currency {

}

struct Euro {
    euro: u64,
    cent: u64
}

impl Euro {
    pub fn new(euro: u64, cent: u64) -> Euro{
        let euro2 = euro + cent/100;
        print!("{}", cent/100);
        return Euro {euro: euro2, cent: cent % 100};
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn euro_right_creation() {
        let e1 = super::Euro::new(10, 88);
        assert_eq!(e1.euro, 10);
        assert_eq!(e1.cent, 88);
    }


    #[test]
    fn euro_wraps_cent() {
        let e1 = super::Euro::new(10, 256);
        assert_eq!(e1.cent, 56);
        assert_eq!(e1.euro, 12);
    }
}




