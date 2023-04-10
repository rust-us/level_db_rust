
mod test {
    use std::borrow::Borrow;
    use crate::util::linked_list::{LinkedList, LinkedListBuilder};
    use crate::util::slice::Slice;

    #[test]
    fn test_() {
        let list_i32: LinkedList<i32> = LinkedList::default();
        let list_i32_1: LinkedList<i32> = LinkedList::new();
        println!("linked list_i32: {:#?}", list_i32);
        println!("linked list_i32_1: {:#?}", list_i32_1);

        let mut list:LinkedList<Slice> = LinkedList::new();

        println!("linked list: {:#?}", list);
    }

    #[test]
    fn test_push() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(8);
        list.push(9);
        println!("linked list: {:#?}", list);

        assert_eq!(list.length(), 2);

        let first = list.get(0).unwrap().unwrap();
        assert!(first.eq(9.borrow()));

        let second = list.get(1).unwrap().unwrap();
        assert!(second.eq(8.borrow()));
    }

    #[test]
    fn test_add_first() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.add_first(8).unwrap();
        list.add_first(9).unwrap();
        println!("linked list: {:#?}", list);

        assert_eq!(list.length(), 2);

        let first = list.get(0).unwrap().unwrap();
        assert!(first.eq(9.borrow()));

        let second = list.get(1).unwrap().unwrap();
        assert!(second.eq(8.borrow()));
    }

    #[test]
    fn test_add() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.add(8);
        list.add(9);
        println!("linked list: {:#?}", list);

        assert_eq!(list.length(), 2);

        let first = list.get(0).unwrap().unwrap();
        assert!(first.eq(8.borrow()));

        let second = list.get(1).unwrap().unwrap();
        assert!(second.eq(9.borrow()));
    }

    #[test]
    fn test_add_last() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.add_last(8).unwrap();
        list.add_last(9).unwrap();
        println!("linked list: {:#?}", list);

        assert_eq!(list.length(), 2);

        let first = list.get(0).unwrap().unwrap();
        assert!(first.eq(8.borrow()));

        let second = list.get(1).unwrap().unwrap();
        assert!(second.eq(9.borrow()));
    }

    #[test]
    fn test_add_by_position() {
        // 头部添加
        let mut list_first: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list_first.add_last(i).unwrap();
        }
        list_first.add_by_position(0, 100).expect("TODO: panic message");
        assert_eq!(list_first.length(), 11);

        let first = list_first.get(0).unwrap().unwrap();
        assert!(first.eq(100.borrow()));
        let second = list_first.get(1).unwrap().unwrap();
        assert!(second.eq(0.borrow()));

        // 尾部添加
        let mut list_second: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list_second.add_last(i).unwrap();
        }
        list_second.add_by_position(10, 100).expect("TODO: panic message");
        assert_eq!(list_second.length(), 11);

        let first = list_second.get(0).unwrap().unwrap();
        assert!(first.eq(0.borrow()));
        let mut second = list_second.get(1).unwrap().unwrap();
        assert!(second.eq(1.borrow()));
        second = list_second.get(9).unwrap().unwrap();
        assert!(second.eq(9.borrow()));
        let thrid = list_second.get(10).unwrap().unwrap();
        assert!(thrid.eq(100.borrow()));
        let thrid1 = list_second.get(list_second.length()-1).unwrap().unwrap();
        assert!(thrid1.eq(100.borrow()));

        // 中间位置添加
        let mut list_random: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list_random.add_last(i).unwrap();
        }
        list_random.add_by_position(7, 100).expect("TODO: panic message");
        assert_eq!(list_random.length(), 11);

        let first = list_random.get(0).unwrap().unwrap();
        assert!(first.eq(0.borrow()));
        let second = list_random.get(1).unwrap().unwrap();
        assert!(second.eq(1.borrow()));
        let thrid = list_random.get(7).unwrap().unwrap();
        assert!(thrid.eq(100.borrow()));
        let dt = list_random.get(8).unwrap().unwrap();
        assert!(dt.eq(7.borrow()));
        let mut fo = list_random.get(10).unwrap().unwrap();
        assert!(fo.eq(9.borrow()));
        fo = list_random.get(list_random.length()-1).unwrap().unwrap();
        assert!(fo.eq(9.borrow()));
    }
}