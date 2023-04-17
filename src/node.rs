use std::{
        cell::{Cell, RefCell},
        rc::Rc,
};

pub type Link<T> = Option<Box<Node<T>>>;
pub type Conn<T> = Rc<RefCell<Link<T>>>;

#[derive(Clone, Debug)]
pub struct Node<T>
where T: Clone
{
        pub elem: T,
        pub freq: Cell<u32>,
        pub next: Conn<T>,
        pub prev: Conn<T>,
}

impl<T> Node<T>
where T: Clone
{
        pub fn new(elem: T) -> Self {
                Node {
                        elem,
                        freq: Cell::new(0u32),
                        next: Rc::new(RefCell::new(None)),
                        prev: Rc::new(RefCell::new(None)),
                }
        }

        pub fn get(&self) -> T {
                // every time a node is retrieved from the cache,
                // we update it's frequency.
                self.freq.set(self.freq.get() + 1);
                self.elem.clone()
        }

        pub fn is_tail(&self) -> bool {
                self.prev.borrow().is_none()
        }

        pub fn is_head(&self) -> bool {
                self.next.borrow().is_none()
        }
}
