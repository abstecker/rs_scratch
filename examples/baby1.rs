fn main() {
    let vec_to_6 = vec![1, 2, 3, 4, 5, 6];
    let vec_from_9 = vec![9, 8, 7, 6, 5, 4];

    let vec1 = &vec_to_6[0..3];
    let vec2 = &vec_from_9[0..3];

    let zipped: i32 = vec1.iter().zip(vec2).map(|(i, j)| i * j).sum();

    println!("{:?} {:?} {zipped}", vec1, vec2);
}
