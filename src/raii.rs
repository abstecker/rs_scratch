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
    pub fn create_box(box_id: usize) -> BoxDrop {
        println!("Creating BoxDrop #{box_id}");
        BoxDrop {
            box_id: Box::new(box_id),
        }
    }

    pub fn spew_boxes(count: usize, offset: usize) -> Vec<BoxDrop> {
        let mut v = Vec::new();
        for i in 0usize..count {
            v.push(BoxDrop::create_box(i + offset));
        }
        v
    }
}

impl Display for BoxDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoxDrop #{}", self.box_id)
    }
}

impl Drop for BoxDrop {
    fn drop(&mut self) {
        println!("BoxDrop #{} dropped", self.box_id);
    }
}
