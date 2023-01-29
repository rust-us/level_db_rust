use std::fmt::{Display, Formatter};
use std::ptr::NonNull;
use crate::util::Result;
use crate::util::slice::Slice;
use crate::util::status::{LevelError, Status};

/// 节点
#[derive(Debug)]
struct Node<T> {
    // 节点值. 如果存在这个 Node，则该Node中 val 必定是有值的; 如果 val 的值为空，则这个 Node 就应该为 None
    val: T,
    // 前驱
    // 因为会出现一个节点同时存在多个可变引用的情况，因此需要使用裸指针(裸指针的包装 NonNull)
    prev: Option<NonNull<Node<T>>>,
    // 后继. Option<T> 表示该节点为空，即不存在 prev 前置节点（整个链表为空时）、或不存在next 后置节点（链表的尾节点）
    next: Option<NonNull<Node<T>>>,
}

/// 双向链表
#[derive(Debug)]
pub struct LinkedList<T> {
    // 双向链表的当前长度
    length: usize,
    // 头
    head: Option<NonNull<Node<T>>>,
    // 尾
    tail: Option<NonNull<Node<T>>>,
}

pub trait LinkedListBuilder<T>: Default {
    /// 构造函数, 构造空的双向链表
    fn new() -> Self;

    fn length(&self) -> usize;

    /// 链表末尾添加元素
    ///
    /// 等同于 add_last
    ///
    /// # Arguments
    ///
    /// * `val`: 插入的元素
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.add(8);
    /// list.add(9);
    /// ```
    fn add(&mut self, val: T);

    /// 在链表头部压入元素，等同于  add_first
    /// 在将一个元素压入双向链表时，需要注意：我们需要获取元素完整的所有权
    ///
    /// # Arguments
    ///
    /// * `val`: 插入数据
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push(8);
    /// list.push(9);
    /// ```
    fn push(&mut self, val: T);

    /// 元素添加到头部，返回是否成功，成功为 true，失败为 false。
    ///
    /// # Arguments
    ///
    /// * `val`: 插入数据
    ///
    /// returns: Result<bool, Status>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.add_first(8).unwrap();
    /// list.add_first(9).unwrap();
    /// ```
    fn add_first(&mut self, val: T) -> Result<bool>;

    /// 在列表结尾添加元素，返回是否成功，成功为 true，失败为 false。
    fn add_last(&mut self, val: T) -> Result<bool>;

    /// 向指定位置插入元素。
    ///
    /// # Arguments
    ///
    /// * `position`: 插入位置。 从0开始，小于等于 len
    /// * `data`:  插入的数据
    ///
    /// returns: Result<bool, Status>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let mut list_link: LinkedList<i32> = LinkedList::new();
    /// for i in 0..10 {
    ///     list_link.add_last(i).unwrap();
    /// }
    /// // list_link.add_by_position(0, 100).expect("panic message");
    /// // list_link.add_by_position(7, 100).expect("panic message");
    /// list_link.add_by_position(10, 100).expect("panic message");
    /// ```
    fn add_by_position(&mut self, position: usize, data: T) -> Result<bool>;

    /// 删除并返回第一个元素。
    fn remove_first(&mut self) -> Result<Option<&T>>;

    /// 删除并返回最后一个元素。
    fn remove_last(&mut self) -> Result<Option<&T>>;

    /// 删除指定位置的元素并返回。
    fn remove(&mut self, position: usize) -> Result<Option<&T>>;
    // public boolean remove(Object o)	删除某一元素，返回是否成功，成功为 true，失败为 false。

    /// 获取列表开头的元素
    fn get_first(&self) -> Result<Option<&T>>;

    /// 获取列表结尾的元素
    fn get_last(&self) -> Result<Option<&T>>;

    /// 得到第 position 元素, 不可变引用类型
    ///
    /// # Arguments
    ///
    /// * `position`:  插入位置。 从0开始，小于 len
    ///
    /// returns: Result<Option<&T>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get(&self, position: usize) -> Result<Option<&T>>;

    /// 得到第 position 元素, 返回可变引用类型
    fn get_mut(&self, position: usize) -> Result<Option<&mut T>>;

    // jdk LinkedList 中的方法：
    // public E	set(int index, E element)    replace， 用指定的元素替换此列表中指定位置的元素。
    // public boolean addAll(Collection c)	将一个集合的所有元素添加到链表后面，返回是否成功，成功为 true，失败为 false。
    // public boolean addAll(int index, Collection c)	将一个集合的所有元素添加到链表的指定位置后面，返回是否成功，成功为 true，失败为 false。

    // public void clear()	清空链表。
    // public boolean removeAll(Collection<?> c)  删除此集合的所有元素，这些元素也包含在指定的集合中（可选操作）。 此调用返回后，此集合将不包含与指定集合相同的元素。
    // public boolean retainAll(Collection<?> c)  仅保留此集合中包含在指定集合中的元素（可选操作）。 换句话说，从此集合中删除未包含在指定集合中的所有元素。
    // public String toString() 返回此集合的字符串表示形式。 字符串表示由一个集合元素的列表组成，它们的迭代器返回它们的顺序，用方括号括起来（ "[]" ）。 相邻元素由字符", " （逗号和空格）分隔。 元素将转换为字符串，如String.valueOf(Object)所示 。

    // public boolean offer(E e)	向链表末尾添加元素，返回是否成功，成功为 true，失败为 false。
    // public boolean offerFirst(E e)	头部插入元素，返回是否成功，成功为 true，失败为 false。
    // public boolean offerLast(E e)	尾部插入元素，返回是否成功，成功为 true，失败为 false。

    // public E poll()	删除并返回第一个元素。
    // public E pollFirst()    检索并删除此列表的第一个元素，如果此列表为空，则返回 null 。
    // public E	pollLast()  检索并删除此列表的最后一个元素，如果此列表为空，则返回 null 。

    // public E	pop()  弹出此列表所代表的堆栈中的元素。(将元素从链表中删除，并且返回)
    // public E	popFirst()
    // public E	popLast()

    // public E element()	返回第一个元素。
    // public E peek()	返回第一个元素。不可变引用类型
    // public E peek_mut()	返回第一个元素。可变引用类型
    // public E peekFirst()	返回头部元素。
    // public E peekLast()	返回尾部元素。

    // public Iterator descendingIterator()	返回倒序迭代器。
    // public ListIterator listIterator(int index)	返回从指定位置开始到末尾的迭代器。
    // public Object[] toArray()	返回一个由链表元素组成的数组。
    // public T[] toArray(T[] a)	返回一个由链表元素转换类型而成的数组。

    // public boolean isEmpty() 如果此集合不包含任何元素，则返回 true 。
    // public boolean contains(Object o)	判断是否含有某一元素。

    // public int indexOf(Object o)	查找指定元素从前往后第一次出现的索引。
    // public int lastIndexOf(Object o)	查找指定元素最后一次出现的索引。
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            prev: None,
            next: None,
        }
    }

    /// 用于将 Box 中的 Node<T> 转为含有所有权的 T 类型
    fn into_val(self: Box<Self>) -> T {
        self.val
    }
}

impl<T> Default for LinkedList<T> {
    /// 构造空的双向链表
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedListBuilder<T> for LinkedList<T> {
    #[inline]
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    #[inline]
    fn length(&self) -> usize {
        self.length
    }

    #[inline]
    fn add(&mut self, val: T) {
        self.add_last(val).unwrap();
    }

    #[inline]
    fn push(&mut self, val: T) {
        self.add_first(val).unwrap();
    }

    #[inline]
    fn add_first(&mut self, val: T) -> Result<bool> {
        // 使用入参中的 val 创建一个链表节点Node，为了方便后续直接从 Box 获取到 raw ptr 裸指针， 使用 Box 包装
        let mut node = Box::new(Node::new(val));

        node.next = self.head;
        // 新节点node 的上一个元素是空
        node.prev = None;

        // 使用 Box::into_raw(node)，将 node 转为裸指针
        let node = NonNull::new(Box::into_raw(node));

        // 判断当前链表的头节点是否为空
        match self.head {
            // 如果为空，则将链表的尾节点指向这个新节点
            None => self.tail = node,
            // 如果头节点不为空，则需要将当前链表头节点的前一个元素赋值为新的节点
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
        }

        // 将新的节点设为链表的头节点，链表长度加一
        self.head = node;
        self.length += 1;

        Ok(true)
    }

    #[inline]
    fn add_last(&mut self, val: T) -> Result<bool> {
        let mut node = Box::new(Node::new(val));

        node.next = None;
        node.prev = self.tail;
        let node = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node,
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
        }

        self.tail = node;
        self.length += 1;

        Ok(true)
    }

    fn add_by_position(&mut self, position: usize, data: T) -> Result<bool> {
        let len = self.length;

        if position > len {
            return Err(Status::wrapper_str(LevelError::KInvalidArgument, "IndexOutOfRange"));
        }

        if position == 0 {
            return self.add_first(data);
        } else if position == len {
            return self.add_last(data);
        }

        unsafe {
            // Create Node
            let mut spliced_node = Box::new(Node::new(data));

            let before_node = self.find_by_position_mut(position - 1)?;
            let after_node = before_node.unwrap().as_mut().next;
            spliced_node.prev = before_node;
            spliced_node.next = after_node;
            let spliced_node = NonNull::new(Box::into_raw(spliced_node));

            // Insert Node
            before_node.unwrap().as_mut().next = spliced_node;
            after_node.unwrap().as_mut().prev = spliced_node;
        }

        self.length += 1;

        Ok(true)
    }

    fn remove_first(&mut self) -> Result<Option<&T>> {
        todo!()
    }

    fn remove_last(&mut self) -> Result<Option<&T>> {
        todo!()
    }

    fn remove(&mut self, position: usize) -> Result<Option<&T>> {
        todo!()
    }

    fn get_first(&self) -> Result<Option<&T>> {
        todo!()
    }

    fn get_last(&self) -> Result<Option<&T>> {
        todo!()
    }

    fn get(&self, position: usize) -> Result<Option<&T>> {
        let len = self.length;

        if position >= len {
            return Err(Status::wrapper_str(LevelError::KInvalidArgument, "IndexOutOfRange"));
        }

        // Iterate towards the node at the given index, either from the start or the end,
        // depending on which would be faster.
        let offset_from_end = len - position - 1;
        let mut cur;
        if position <= offset_from_end {
            // Head to Tail
            cur = self.head;
            for _ in 0..position {
                match cur.take() {
                    None => {
                        cur = self.head;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                }
            }
        } else {
            // Tail to Head
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    None => {
                        cur = self.tail;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        }

        unsafe { Ok(cur.as_ref().map(|node| &node.as_ref().val)) }
    }

    fn get_mut(&self, position: usize) -> Result<Option<&mut T>> {
        let mut cur = self.find_by_position_mut(position)?;
        unsafe { Ok(cur.as_mut().map(|node| &mut node.as_mut().val)) }
    }
}

impl<T> LinkedList<T> {
    fn find_by_position_mut(&self, position: usize) -> Result<Option<NonNull<Node<T>>>> {
        let len = self.length;

        if position >= len {
            return Err(Status::wrapper_str(LevelError::KInvalidArgument, "IndexOutOfRange"));
        }

        // Iterate towards the node at the given index, either from the start or the end,
        // depending on which would be faster.
        let offset_from_end = len - position - 1;
        let mut cur;
        if position <= offset_from_end {
            // Head to Tail
            cur = self.head;
            for _ in 0..position {
                match cur.take() {
                    None => {
                        cur = self.head;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                }
            }
        } else {
            // Tail to Head
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    None => {
                        cur = self.tail;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        }

        Ok(cur)
    }
}
