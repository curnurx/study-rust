struct Dummy {
    arr: [i32; 8],
}

fn main() {
    // array copy test
    let arr = [4; 8];
    let arr_address = &arr[0] as *const i32;

    let arr_copy = arr;
    let arr_copy_address = &arr_copy[0] as *const i32;

    // Each of two array has a diffent address. So array is strongly copied.
    // After copy, two arrays are valid.
    assert_ne!(arr_address, arr_copy_address);


    // array in struct move test
    let dummy = Dummy { arr: [3; 8] };
    let dummy_arr_address = &dummy.arr[0] as *const i32;

    let dummy_move = dummy;
    let dummy_move_arr_address = &dummy_move.arr[0] as *const i32;

    // struct is moved, but each of struct has diffent address.
    // Each of array has also diffent address.
    // "dummy" struct and array become invalid.
    // "dummy_move" struct and copied array are valid.
    assert_ne!(dummy_arr_address, dummy_move_arr_address);
     


    let vec = vec![2; 8];
    let vec_address = &vec as *const Vec<i32>;
    let array_address = &vec[0] as *const i32;

    let vec_move = vec;
    let vec_move_address = &vec_move as *const Vec<i32>;
    let array_move_address = &vec_move[0] as *const i32;
    
    // Each of vec has a diffrent address,
    // but each of &[i32] in vec has same address.
    // It means weak copy.
    // And vec is moved.
    assert_ne!(vec_address, vec_move_address);
    assert_eq!(array_address, array_move_address);

    // Conclusion
    // array is copied. vector is moved.
}
