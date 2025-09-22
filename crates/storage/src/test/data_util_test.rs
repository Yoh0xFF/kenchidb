use crate::data_util::get_fletcher32;

#[test]
    fn test_fletcher32_basic() {
        let data = b"hello world";
        let checksum = get_fletcher32(data, 0, data.len());
        println!("Fletcher32 checksum: 0x{:08x}", checksum);
    }

    #[test]
    fn test_fletcher32_odd_length() {
        let data = b"hello"; // 5 bytes (odd)
        let checksum = get_fletcher32(data, 0, data.len());
        println!("Fletcher32 checksum (odd): 0x{:08x}", checksum);
    }

    #[test]
    fn test_fletcher32_with_offset() {
        let data = b"xxhello world";
        let checksum1 = get_fletcher32(data, 2, 11); // skip "xx"
        let checksum2 = get_fletcher32(b"hello world", 0, 11);
        assert_eq!(checksum1, checksum2);
    }

    #[test]
    fn test_fletcher32_empty() {
        let data = b"";
        let checksum = get_fletcher32(data, 0, 0);
        assert_eq!(checksum, 0xffff_ffff);
    }