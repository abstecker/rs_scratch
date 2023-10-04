use std::fmt::{Display, Formatter};

/// Example of Rust `resource acquisition is initialization` memory management adapted  from
/// [Rust by Example](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
///
/// References:
/// * [Using Box<T> to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html).
/// * [The Stack and the Heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BoxDrop {
    pub box_id: Box<usize>,
}

impl BoxDrop {
    /// Allocates an integer on the heap
    #[must_use]
    pub fn create_box(box_id: usize) -> BoxDrop {
        println!("BoxDrop #{box_id} is alive! It's alive!!!!");
        BoxDrop {
            box_id: Box::new(box_id),
        }
    }

    #[must_use]
    pub fn spew_boxes(count: usize, offset: usize) -> Vec<BoxDrop> {
        let mut v = Vec::new();
        for i in 0usize..count {
            v.push(BoxDrop::create_box(i + offset));
        }
        v
    }

    #[must_use]
    pub fn spew_boxed_boxes(count: usize, offset: usize) -> Vec<Box<BoxDrop>> {
        let mut v = Vec::new();
        for i in 0usize..count {
            let bbox = Box::new(BoxDrop::create_box(i + offset));
            v.push(bbox);
        }
        v
    }
}

impl Display for BoxDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello, my name is BoxDrop #{}", self.box_id)
    }
}

impl Drop for BoxDrop {
    fn drop(&mut self) {
        println!("BoxDrop #{} has fallen and can't get up.", self.box_id);
    }
}
