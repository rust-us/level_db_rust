
mod test{
    use crate::util::histogram::Histogram;

    #[test]
    fn test_add() {
        let mut histogram = Histogram::default();
        histogram.add(1.0);
        histogram.add(0.2);
        histogram.add(6.0);
        print!("{}", histogram.to_string());
    }
}
