
mod test{
    use crate::util::logging::Logging;
    use crate::util::slice::Slice;

    #[test]
    fn tes_append_number_to() {
        let mut s = String::from("18446744073709551615");
        Logging::append_number_to(&mut s, u64::MAX);
        println!("s = {}", s)
    }

    #[test]
    fn tes_append_escaped_string_to() {

        let mut s = String::from("18446744073709551615");
        let sli = Slice::from("18446744073709551615");
        Logging::append_escaped_string_to(&mut s, &sli);
        println!("s = {}", s)
    }

    #[test]
    fn tes_consume_decimal_number() {
        let mut sin = Slice::from("19103912");
        //let mut sin = Slice::from("18446744073709551616");
        //let mut sin = Slice::from("18446744073709551615");

        let start = *sin.first().unwrap();


        println!("{}", start);

        let mut num: u64 = 0;

        let r = Logging::consume_decimal_number(&mut sin, &mut num);

        //sin.remove_prefix(20);
        println!("转换结果：{}, sin = {}, num = {}", r, String::from(sin), num)
    }

}