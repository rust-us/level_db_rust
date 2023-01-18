
mod test {
    use crate::util::linked_list::LinkedList;
    use crate::util::slice::Slice;

    #[test]
    fn test_() {
        let mut list:LinkedList<Slice> = LinkedList::new();

        println!("linked list: {:#?}", list);
    }
}