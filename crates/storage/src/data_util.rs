/// Calculate the Fletcher32 checksum.
///
/// # Arguments
/// * `bytes` - The byte slice to calculate checksum for
/// * `offset` - Initial offset into the byte slice
/// * `length` - The message length (if odd, 0 is appended)
///
/// # Returns
/// The 32-bit Fletcher32 checksum as u32
///
/// # Panics
/// Panics if offset + length exceeds the bounds of the byte slice
pub fn get_fletcher32(bytes: &[u8], offset: usize, length: usize) -> u32 {
    let (mut sum1, mut sum2) = (0xffff_u32, 0xffff_u32);
    let (mut i, len) = (offset, offset + (length & !1));

    // Ensure we don't go out of bounds
    assert!(len <= bytes.len(), "offset + length exceeds byte slice bounds");

    while i < len {
        // reduce after 360 words (each word is two bytes)
        let end = std::cmp::min(i + 720, len);
        while i < end {
            let x = ((bytes[i] as u32) << 8) | (bytes[i + 1] as u32);
            i += 2;
            sum1 += x;
            sum2 += sum1;
        }
        sum1 = (sum1 & 0xffff) + (sum1 >> 16);
        sum2 = (sum2 & 0xffff) + (sum2 >> 16);
    }

    // Handle odd length: append 0
    if (length & 1) != 0 {
        assert!(i < bytes.len(), "odd length handling: index out of bounds");
        let x = (bytes[i] as u32) << 8;
        sum1 += x;
        sum2 += sum1;
    }

    sum1 = (sum1 & 0xffff) + (sum1 >> 16);
    sum2 = (sum2 & 0xffff) + (sum2 >> 16);

    (sum2 << 16) | sum1
}
