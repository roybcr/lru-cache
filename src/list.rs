use std::{cell::RefCell, rc::Rc};

use crate::node::Conn;

#[derive(Debug)]
pub struct List<T>
where T: Clone
{
        head: Conn<T>,
        tail: Conn<T>,
        size: usize,
        cap:  usize,
}

impl<T> List<T>
where T: Clone
{
        pub fn new(cap: usize) -> Self {
                List {
                        cap,
                        size: 0,
                        head: Rc::new(RefCell::new(None)),
                        tail: Rc::new(RefCell::new(None)),
                }
        }

        pub fn reorder(&self, first: Conn<T>, secnd: Conn<T>) {
                let first_ptr = Rc::clone(&first);
                let secnd_ptr = Rc::clone(&secnd);

                let first_mut = &mut *first_ptr.borrow_mut();
                let secnd_mut = &mut *secnd_ptr.borrow_mut();
                let first_mut = first_mut.as_mut().unwrap();
                let secnd_mut = secnd_mut.as_mut().unwrap();

                let fnext_ptr = Rc::clone(&((*first_mut).next));
                let fprev_ptr = Rc::clone(&((*first_mut).prev));
                let snext_ptr = Rc::clone(&((*secnd_mut).next));
                let sprev_ptr = Rc::clone(&((*secnd_mut).prev));

                let fnext_mut = &mut *fnext_ptr.borrow_mut();
                let fprev_mut = &mut *fprev_ptr.borrow_mut();
                let snext_mut = &mut *snext_ptr.borrow_mut();
                let sprev_mut = &mut *sprev_ptr.borrow_mut();

                match (first_mut.is_head(), first_mut.is_tail()) {
                        (true, true) => {}
                        (true, false) => {}
                        (false, true) => {}
                        (false, false) => {}
                }
        }
}
