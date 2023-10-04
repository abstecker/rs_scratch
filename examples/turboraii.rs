use rs_scratch::raii::BoxDrop;
use rayon::prelude::*;

/// Example of Rust `resource acquisition is initialization` memory management adapted from
/// [Rust by Example](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
///
/// `cargo run --example raii2`
fn main() {
    let original = BoxDrop::spew_boxes(100, 0);

    let from_ref: Vec<BoxDrop> = turbo_boxit(&original, 1_000);
    let from_value: Vec<BoxDrop> = boxit_simple_value(original, 10_000);

    for bd in from_value {
        println!("{bd}");
    }

    for bd in from_ref {
        println!("{bd}");
    }
}

fn boxit_simple_ref(v: &Vec<BoxDrop>, offset: usize) -> Vec<BoxDrop> {
    v
        .iter()
        .map(|b| BoxDrop::create_box(*b.box_id + offset))
        .collect()
}

fn boxit_simple_value(v: Vec<BoxDrop>, offset: usize) -> Vec<BoxDrop> {
    v
        .iter()
        .map(|b| BoxDrop::create_box(*b.box_id + offset))
        .collect()
}

fn turbo_boxit(v: &Vec<BoxDrop>, offset: usize) -> Vec<BoxDrop> {
    v
        .par_iter()
        .map(|b| BoxDrop::create_box(*b.box_id + offset))
        .collect()
}
