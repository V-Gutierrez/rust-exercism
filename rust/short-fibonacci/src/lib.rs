/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let buffer = vec![0 as u8; count];

    buffer
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.

/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fibonacci_first_five = create_buffer(5);

    for number in 0..5 {
        match number {
            0 | 1 => {
                fibonacci_first_five[number] = 1;
            }
            _ => {
                fibonacci_first_five[number] =
                    &fibonacci_first_five[number - 1] + &fibonacci_first_five[number - 2];
            }
        }
    }

    fibonacci_first_five
}