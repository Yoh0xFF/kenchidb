use crate::chunk::{ChunkFooter, ChunkHeader};

#[test]
fn test_header_roundtrip() {
    let original = ChunkHeader {
        magic: *b"KNCH",
        id: 123,
        length: 456,
        version: 789,
        time: 1234567890,
        max_length: 999,
        page_count: 10,
        pin_count: 5,
        table_of_content_position: 100,
        layout_root_position: 200,
        map_id: 42,
        next: 300,
    };

    let serialized = original.serialize_header();
    let deserialized = ChunkHeader::deserialize_header(&serialized).unwrap();

    assert_eq!(original.magic, deserialized.magic);
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.length, deserialized.length);
    assert_eq!(original.version, deserialized.version);
    assert_eq!(original.time, deserialized.time);
    assert_eq!(original.max_length, deserialized.max_length);
    assert_eq!(original.page_count, deserialized.page_count);
    assert_eq!(original.pin_count, deserialized.pin_count);
    assert_eq!(
        original.table_of_content_position,
        deserialized.table_of_content_position
    );
    assert_eq!(
        original.layout_root_position,
        deserialized.layout_root_position
    );
    assert_eq!(original.map_id, deserialized.map_id);
    assert_eq!(original.next, deserialized.next);
}

#[test]
fn test_footer_roundtrip() {
    let original = ChunkFooter {
        id: 123,
        length: 456,
        version: 789,
        checksum: 0, // Will be calculated during serialization
    };

    let serialized = original.serialize_footer();
    let deserialized = ChunkFooter::deserialize_footer(&serialized).unwrap();

    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.length, deserialized.length);
    assert_eq!(original.version, deserialized.version);
    // checksum will be different as it's calculated during serialization
    assert!(ChunkFooter::verify_footer(&serialized));
}
