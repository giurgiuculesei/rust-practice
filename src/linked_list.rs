#[test]
fn it_works() {
    let node = MyNode::new("aa");
    assert_eq!(node.element, "aa");
    //assert_eq!(node.next, None);

    let empty_list: MyLinkedList<&str> = MyLinkedList::new();
    assert_eq!(empty_list.len(), 0);
    assert_eq!(empty_list.peek(), None);

    let mut list = MyLinkedList::new();
    list.push("bbb");
    list.push("ccc");
    assert_eq!(list.len(), 2);

    assert_eq!(list.peek(), Some(&"ccc"));
    assert_eq!(list.pop(), Some("ccc"));
    assert_eq!(list.pop(), Some("bbb"));
    assert_eq!(list.pop(), None);

    let mut delete_list = MyLinkedList::new();
    delete_list.push(1);
    delete_list.push(2);
    delete_list.push(3);
    delete_list.push(4);

    assert_eq!(delete_list.delete(4), true);
    assert_eq!(delete_list.len(), 3);

    assert_eq!(delete_list.delete(2), true);
    assert_eq!(delete_list.len(), 2);

    delete_list.iterate(|node| {println!("{:?}", node)})
}

#[derive(Debug, PartialEq)]
struct MyNode<T> {
    next: Option<Box<MyNode<T>>>,
    element: T,
}

impl<T> MyNode<T> {
    fn new(element: T) -> Self {
        MyNode { next: None, element }
    }
}

#[derive(Debug)]
struct MyLinkedList<T> {
    head: Option<Box<MyNode<T>>>,
}

impl<T: std::cmp::PartialEq> MyLinkedList<T> {
    fn new() -> Self {
        MyLinkedList { head: None }
    }

    fn push(&mut self, element: T) {
        let mut node = MyNode::new(element);
        node.next = self.head.take();

        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        /*let head = self.head.take();
        match head {
            Some(head) => {
                self.head = head.next;

                Some(head.element)
            },
            None => None
        }*/

        self.head.take().map(|node| {
            self.head = node.next;

            node.element
        })
    }

    fn len(&self) -> usize {
        let mut len = 0;
        let mut p = &self.head;

        while let Some(ref node) = *p {
            len += 1;

            p = &node.next;
        }

        len
    }

    fn peek(&self) -> Option<&T> {
        /*match self.head {
            Some(ref head) => Some(&head.element),
            None => None
        }*/

        self.head.as_ref().map(|node| &node.element)
    }

    fn delete(&mut self, element: T) -> bool {
        let mut head = &mut self.head;

        //head
        if let Some(ref mut node) = *head {
            if node.element == element {
                self.head = node.next.take();

                println!("root found");

                return true;
            }
        }

        while let Some(ref mut node) = *head {
            if let Some(ref mut next) = node.next {
                if next.element == element {
                    println!("parent found");

                    node.next = next.next.take();

                    return true;
                }
            }

            head = &mut node.next;
        }

        false
    }

    fn iterate<F>(&self, mut f: F)
        where
            F: FnMut(&MyNode<T>),
    {
        let mut current = &self.head;
        while let Some(ref node) = *current {
            f(node);
            current = &node.next;
        }
    }
}