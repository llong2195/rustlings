fn main() {
    // You can optionally experiment here.
}

fn slice_array(arr: &[i32], start: usize, end: usize) -> &[i32] {
    &arr[start..end]
}

#[cfg(test)]
mod tests {
    use crate::slice_array;

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        println!("Array a: {:?}", a);
        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = slice_array(&a, 1, 4);
        assert_eq!([2, 3, 4], nice_slice);
    }
}
