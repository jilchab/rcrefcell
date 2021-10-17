use std::{
    cell::{RefCell,Ref, RefMut},
    fmt,
    ops::Deref,
    rc::{Rc, Weak},
};

pub struct Shared<T> {
    v: Rc<RefCell<T>>
}

impl <T> Shared<T> {
    pub fn new(t: T)-> Shared<T> {
        Shared{v: Rc::new(RefCell::new(t))}
    }
    pub fn new_from(rc: Rc<RefCell<T>>) -> Shared<T> {
        Shared{v: rc}
    }
    pub fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.v.borrow_mut()
    }
    pub fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
    pub fn clone(&self) -> Self {
        Self {v: self.v.clone()}
    }
    pub fn downgrade(&self) -> WeakShared<T> {
        WeakShared::new_from(Rc::downgrade(&self.v))
    }
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.v)
    }
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.v)
    }
}

impl <T: fmt::Display> fmt::Display for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl <T: fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl <'a,T> Deref for Shared<T>{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe {self.as_ptr().as_ref().unwrap()}
    }
}

pub struct WeakShared<T> {
    v: Weak<RefCell<T>>
}

impl <T> WeakShared<T> {
    pub fn new()-> WeakShared<T> {
        WeakShared{v: Weak::new()}
    }
    pub fn new_from(weak: Weak<RefCell<T>>) -> WeakShared<T> {
        WeakShared{v: weak}
    }
    pub fn clone(&self) -> Self {
        Self {v: self.v.clone()}
    }
    pub fn upgrade(&self) -> Option<Shared<T>> {
        if let Some(rc) = self.v.upgrade() {
            Some(Shared::new_from(rc))
        } else {
            None
        }
    }
    pub fn strong_count(&self) -> usize {
        self.v.strong_count()
    }
    pub fn weak_count(&self) -> usize {
        Weak::weak_count(&self.v)
    }
    pub fn as_ptr(&self) -> *const T {
        self.upgrade().unwrap().as_ptr()
    }
}

impl <'a,T> Deref for WeakShared<T>{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe {self.as_ptr().as_ref().unwrap()}
    }
}

impl <T: fmt::Display> fmt::Display for WeakShared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weak_count() > 0 {
            write!(f, "{}", self.deref())
        } else {
            write!(f, "No ref")
        }
        
    }
}

impl <T: fmt::Debug> fmt::Debug for WeakShared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weak_count() > 0 {
            write!(f, "{:?}", self.deref())
        } else {
            write!(f, "No ref")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn clone_shared() {
        let shared1 = Shared::new("Hello".to_string());
        let shared2 = shared1.clone();

        assert_eq!(&*shared2.borrow(), "Hello");
        assert_eq!(shared1.strong_count(), shared2.strong_count());
        assert_eq!(shared1.strong_count(), 2);
    }
    #[test]
    fn clone_weak() {
        let shared = Shared::new("Hello".to_string());
        let weak = shared.downgrade();
        let weak2 = weak.clone();

        assert_eq!(weak.weak_count(), 2);
        
        let shared2 = weak2.upgrade().unwrap();

        assert_eq!(&*shared2.borrow(), "Hello");
        assert_eq!(weak.strong_count(), 2);
    }
    #[test]
    fn downgrade_and_upgrade() {
        let shared = Shared::new("Hello".to_string());
        let weak = shared.downgrade();
        let shared2 = weak.upgrade().unwrap();

        assert_eq!(&*shared2.borrow(), "Hello");
        assert_eq!(weak.strong_count(), 2);
    }
    #[test]
    fn borrow() {
        let shared = Shared::new("Hello".to_string());

        assert_eq!(&*shared.borrow(), "Hello");
    }
    #[test]
    fn borrow_mut() {
        let shared = Shared::new("Hello".to_string());

        *shared.borrow_mut() += ", world!";

        assert_eq!(&*shared.borrow(), "Hello, world!");
    }
}
