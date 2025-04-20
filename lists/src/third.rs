use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 实现 IntoIter
// pub struct IntoIter<T> {
//     list: List<T>,
// }

// // 实现IterMut
// pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>,
// }


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem:T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    // remove the first element and return the rest of the list
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {next: self.head.as_deref()}
    }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter { list: self }
    // }

    // pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     IterMut { next: self.head.as_ref().map(|node| node.borrow_mut()) }
    // }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

// 实现 IntoIter
// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.list.head.take().and_then(|node| {
//             match Rc::try_unwrap(node) {
//                 Ok(node) => {
//                     self.list.head = node.next;
//                     Some(node.elem)
//                 }
//                 Err(_) => None,
//             }
//         })
//     }
// }

// 实现IterMut
// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|mut node| {
//             self.next = node.next.as_ref().map(|next| next.borrow_mut());
//             &mut node.elem
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // make suer empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    // 测试 IntoIter
    // #[test]
    // fn into_iter() {
    //     let list = List::new().prepend(1).prepend(2).prepend(3);

    //     let mut iter = list.into_iter();
    //     assert_eq!(iter.next(), Some(3));
    //     assert_eq!(iter.next(), Some(2));
    //     assert_eq!(iter.next(), Some(1));
    // }

    // #[test]
    // fn iter_mut() {
    //     let mut list = List::new().prepend(1).prepend(2).prepend(3);

    //     let mut iter = list.iter_mut();
    //     assert_eq!(iter.next(), Some(&mut 3));
    //     assert_eq!(iter.next(), Some(&mut 2));
    //     assert_eq!(iter.next(), Some(&mut 1));
    // }
}

