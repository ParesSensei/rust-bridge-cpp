fn main() {
    println!("Hello, world!");
}

#[test]
fn slice_operations() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let full_slice = &data[..];        // [1,2,3,4,5,6,7,8]
    let partial_slice = &data[2..6];   // [3,4,5,6]
    let from_start = &data[..4];       // [1,2,3,4]
    let to_end = &data[3..];           // [4,5,6,7,8]

    println!("full_slice: {:?},\npartial_slice: {:?}", full_slice, partial_slice);
    println!("from_start: {:?},\nto_end: {:?}", from_start, to_end);
}
