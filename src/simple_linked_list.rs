#[test]
fn it_works() {
    let mut list = SimpleLinkedList::new();
    list.push("bbb");
    list.push("ccc");
    assert_eq!(list.len(), 2);

    assert_eq!(list.peek(), Some(&"ccc"));
    assert_eq!(list.pop(), Some("ccc"));
    assert_eq!(list.pop(), Some("bbb"));
    assert_eq!(list.pop(), None);
}

struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    fn new() -> Self {
        SimpleLinkedList::<T> { head: None }
    }

    fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut p = &self.head;
        while let Some(ref node) = *p {
            count += 1;
            p = &node.next;
        }
        count
    }

    fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::<T> {
            data: element,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                self.head = node.next.take();
                Some(node.data)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => {
                /*let a  = node;
                let b  = &node;

                let address = format!("test {:p}", a); // this produces something like '0x7f06092ac6d0'
                println!("{}", address);

                let address2 = format!("test {:p}", b); // this produces something like '0x7f06092ac6d0'
                println!("{}", address2);*/

                Some(&node.data)
            }
            None => None,
        }
    }

}