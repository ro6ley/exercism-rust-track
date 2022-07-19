/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut vec = create_empty();
    for _ in 0..count {
        vec.push(0);
    }
    vec
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut res: Vec<u8> = create_empty();

    let mut sum;
    let mut a = 0;
    let mut b = 1;
    for _i in 0..5 {
        sum = a + b;
        a = b;
        b = sum;

        res.push(a);
    }
    res
}
