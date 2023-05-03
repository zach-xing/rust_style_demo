/// 一个最小可行单链栈
use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl Drop for List {
    fn drop(&mut self) {
        // 这步就是将 self.head 设置为 None
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // 这个循环就是将链表中的节点置为 None
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /**
    * 在链表首部插入元素
    * @desc 为什么要用 mem:replace?
       因为 Rust 的所有权机制要求，一个值只能被绑定到一个变量上。
       如果我们直接将 self.head 的值赋值给 new_node.next，那么在下
       一行代码我们再次将 self.head 的值赋值为 Link::More(new_node)
       的时候，就会出现借用冲突的问题。为了避免这种情况，
       我们必须在赋值之前将 self.head 的值替换为一个初始的空枚举变量 Link::Empty，
       这样就可以安全地获取到 self.head 的旧值了。
    */
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    /**
     * 删除首个元素
     */
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        // 应该就是 1 -> 2 -> 3

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        // 应该是 5 -> 4 -> 1

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
