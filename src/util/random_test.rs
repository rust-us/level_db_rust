mod test {
    use crate::util::random::Random;

    #[test]
    pub fn test_next() {
        let mut random = Random::new(32);
        let num = random.next();
        println!("{}", num);
        let num2 = random.next();
        println!("{}", num2);
        assert_ne!(num, num2);
    }

    #[test]
    pub fn test_uniform() {
        todo!()
    }

    #[test]
    pub fn test_one_in() {
        todo!()
    }

    #[test]
    pub fn test_skesed() {
        todo!()
    }
}