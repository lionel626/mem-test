use std::{
    alloc::{realloc, Layout},
    ptr::NonNull,
};

struct Contact {
    name: String,
    age: u32,
}

struct ContactStorage {
    store: NonNull<Contact>,
    length: usize,
    capacity: usize,
}

impl ContactStorage {
    pub fn init() -> Self {
        return ContactStorage {
            store: NonNull::dangling(),
            length: 0,
            capacity: 0,
        };
    }

    pub fn push(&mut self, elem: Contact) {
        if self.capacity() == 0 {
            // Initialize Heap
            let layout =
                std::alloc::Layout::array::<Contact>(4).expect("could not allocate layout");
            let ptr: *mut Contact = unsafe { std::alloc::alloc(layout) } as *mut Contact;
            let ptr = NonNull::new(ptr).expect("could not alocate ptr");
            unsafe { ptr.as_ptr().write(elem) };
            self.store = ptr;
            self.capacity = 4;
            self.length = 1;
        } else if self.length < self.capacity {
            let offset = self
                .len()
                .checked_mul(std::mem::size_of::<Contact>())
                .expect("Cannot reach memory");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            let ptr = unsafe { self.store.as_ptr().add(self.length) };
            unsafe { ptr.write(elem) };
            self.length += 1;
        } else if self.len() == self.capacity() {
            let new_capacity = self
                .capacity()
                .checked_mul(2)
                .expect("Wrap value on capacity");
            // check if new size is a multiple of align
            let align = std::mem::align_of::<Contact>();
            let size = self.capacity * std::mem::size_of::<Contact>();
            size.checked_add(size % align).expect("Cant allocate");
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<Contact>() * new_capacity;
                let ptr = std::alloc::realloc(self.store.as_ptr() as *mut u8, layout, new_size)
                    as *mut Contact;
                let ptr = NonNull::new(ptr).expect("Could not reallocated");
                self.store = ptr;
            }
            self.length += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn get(&self, i:usize) -> Option<&Contact>{
        if i >= self.len() {
            return None;
        }

        Some(unsafe {
            &self.store.as_ptr().add(i).read()
        })
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }
}

impl Drop for ContactStorage {
    fn drop(&mut self) {
        unsafe { 
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.store.as_ptr(), self.len()));
            let layout = Layout::from_size_align_unchecked(std::mem::size_of::<Contact>(), std::mem::align_of<Contact>())
            std::alloc::dealloc(self.store.as_ptr() as *mut u8, layout)
        };
    }
}

fn main() {
    let contacts: ContactStorage = ContactStorage::init();
    assert_eq!(contacts.capacity(), 0);
    assert_eq!(contacts.len(), 0);
    println!("Hello, world!");
}
