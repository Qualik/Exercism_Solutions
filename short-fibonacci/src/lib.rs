/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
       vec![]
}
/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut v = create_buffer(5);
// assign 1st & 2nd num
    v[0] = 1;
    v[1] = 1; 
// loop over remaining elements and calc by sum of the prev 2
    for i in 2..5 {
        v[i] = v[i-2] + v[i-1]

        // vec![1, 1, 2, 3, 5]
    }
    
    v
}


