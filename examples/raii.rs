use rs_scratch::raii::BoxDrop;

/// Example of Rust `resource acquisition is initialization` memory management adapted from
/// [Rust by Example](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
///
/// `cargo run --example raii2`
fn main() {
    // Allocate an integer on the heap
    let box0 = BoxDrop::create_box(0);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box999_999 = BoxDrop::create_box(999_999);
        // _box999_999 is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for id in 1usize..100 {
        BoxDrop::create_box(id);
    }

    let _v = BoxDrop::spew_boxes(100, 1000);

    println!("Hello {box0}");
}
