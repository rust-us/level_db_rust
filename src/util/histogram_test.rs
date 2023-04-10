
mod test{
    use crate::util::histogram::Histogram;

    #[test]
    fn test_add() {
        let mut histogram = Histogram::default();
        histogram.add(2.0);
        let mut other = Histogram::default();
        other.add(20.0);
        print!("{}", histogram.to_string());
        print!("{}", other.to_string());
    }
}
