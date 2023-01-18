use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// 节点
#[derive(Debug)]
struct Node<T> {
    // 节点值. 如果存在这个 Node，则该Node中 val 必定是有值的; 如果 val 的值为空，则这个 Node 就应该为 None
    val: T,
    // 前驱
    prev: Option<NonNull<Node<T>>>
    // 后继. Option<T> 表示该节点为空，即不存在 prev 前置节点（整个链表为空时）、或不存在next 后置节点（链表的尾节点）
    next: Option<NonNull<Node<T>>>,
}

/// 双向链表
#[derive(Debug)]
pub struct LinkedList<T> {
    // 链表长度
    length: usize,
    // 头
    head: Option<NonNull<Node<T>>>,
    // 尾
    tail: Option<NonNull<Node<T>>>,
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

impl<T> LinkedList<T> {
    /// 构造空的双向链表
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None
        }
    }
}
