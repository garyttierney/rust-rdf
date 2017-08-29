
pub trait TableValue: Ord {
    /// Encode this table value to a vector of bytes.
    fn encode(&self) -> Vec<u8>;

    /// Decode an input vector of bytes into an value struct.
    fn decode(data: &[u8]) -> Self;
}

fn cmp_encoded<T: TableValue>(a: &[u8], b: &[u8]) -> Ordering {
    let decoded_a = T::decode(a);
    let decoded_b = T::decode(b);

    decoded_a.cmp(&decoded_b)
}
