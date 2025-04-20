use std::mem;

pub struct  List {
    head: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
struct  Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }
    // 由于push会改变链表，因为使用&mut self的方法签名
    pub fn push(&mut self, elem: i32) {
        // 首先创建一个Node存放元素
        let new_node = Box::new(Node {
            elem: elem,
            // 从借用self中偷出了它的值head并赋予给next字段，同时将一个新值Link::Empty放入到head中
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            // 如果链表为空，则返回None
            Link::Empty => None,
            //如果链表不为空，返回第一个元素，并将head指向下一个节点node.nextssss
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被drop
            // 由于它的‘next'字段拥有的‘node'被设置为link::empty,
            // 因为这里并不会有无边界的递归发生
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4)); 

        // check exhaution
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);               
    }
}