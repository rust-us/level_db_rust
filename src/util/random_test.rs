mod test {
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::util::random::Random;

    fn get_timestamp() -> u32 {
        let now = SystemTime::now();
        now
            .duration_since(UNIX_EPOCH)
            .expect("get_current_unix_err")
            .as_secs() as u32
    }

    fn get_seed(use_timestamp: bool) -> u32 {
        match use_timestamp {
            true => get_timestamp(),
            false => 32
        }
    }

    #[test]
    pub fn test_next() {
        let mut random = Random::new(get_seed(false));
        let num = random.next();
        println!("{}", num);
        assert_eq!(num, 537824);
        let num2 = random.next();
        println!("{}", num2);
        assert_ne!(num, num2);
        assert_eq!(num2, 449273376);
    }

    #[test]
    pub fn test_uniform() {
        let mut random = Random::new(get_seed(false));
        let num = random.uniform(4);
        println!("{}", num);
        assert_eq!(num, 0);

        let num2 = random.uniform(5);
        println!("{}", num2);
        assert_eq!(num2, 1);
    }

    #[test]
    pub fn test_one_in() {
        let mut random = Random::new(get_seed(false));
        let one_in = random.one_in(2);
        println!("{}", one_in);
        assert_eq!(one_in, true);

        let mut random = Random::new(get_seed(false) + 1);
        let one_in = random.one_in(2);
        println!("{}", one_in);
        assert_eq!(one_in, false);
    }

    #[test]
    pub fn test_next_bool() {
        let mut random = Random::new(get_seed(false));
        let num = random.next_bool();
        println!("{}", num);
        assert_eq!(num, true);

        let mut random = Random::new(get_seed(false) + 1);
        let num = random.next_bool();
        println!("{}", num);
        assert_eq!(num, false);
    }

    #[test]
    pub fn test_skesed() {
        let mut random = Random::new(1021);
        let base = random.next();
        let num = random.skewed(31);
        let uniform = random.uniform(31);
        println!("num: {}", num);
        println!("base: {}", base);
        println!("uniform: {}", uniform);
        assert_eq!(num != base, true);
        assert_eq!(num != uniform, true);
    }
}