use std::mem;

pub fn rust_arrays() {
    // this creates an array of 5 elements and fills it with 0
    let mut array: [i32; 5] = [0; 5]; // or you can assing individually

    // setting elements by index
    array[4] = 232;

    // print last element of array
    println!("{:?}", array[array.len() - 1]);

    // find the amount of bytes an array takes
    println!("array takes {}bytes", mem::size_of_val(&array));

    // when borrowed they are automatically converted into slices
    borrow_array_slice(&array);

    // borrow some section of the array
    borrow_array_slice(&array[1..4]);
}

fn borrow_array_slice(slice: &[i32]) {
    println!("{:?} is a slice of length {}", slice, slice.len());
}
