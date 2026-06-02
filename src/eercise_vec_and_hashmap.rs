


#[test]
fn exercise() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, true);
    map.insert(2, true);
    map.insert(3, true);
    map.insert(4, false);
    map.insert(5, false);
    map.insert(6, false);
    let mut value = Vec::new();
    let mut key = Vec::new();
    for (k, v) in &map {
        value.push(*k);
        key.push(*v);
    }

    println!("{:?}", map);
    println!("{:?}", value);
    println!("{:?}", key);
}



// solution
//
// use std::collections::HashMap;
//
// fn main() {
//     let map = HashMap::from([(1, true), (2, false), (3, true), (4, false)]);
//     let mut keys = Vec::new();
//     let mut values = Vec::new();
//     for (k, v) in &map {
//         keys.push(*k);
//         values.push(*v);
//     }
//     println!("Keys:   {keys:?}");
//     println!("Values: {values:?}");
//
//     // Alternative: use iterators with unzip()
//     let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
//     println!("Keys (unzip):   {keys2:?}");
//     println!("Values (unzip): {values2:?}");
// }