use test_util;
use uuid::prelude::*;
use std::prelude::v1::*;

//#[test]
pub fn test_nil() {
    let nil = Uuid::nil();
    let not_nil = test_util::new();
    let from_bytes = Uuid::from_bytes([
        4, 54, 67, 12, 43, 2, 2, 76, 32, 50, 87, 5, 1, 33, 43, 87,
    ]);

    assert_eq!(from_bytes.get_version(), None);

    assert!(nil.is_nil());
    assert!(!not_nil.is_nil());

    assert_eq!(nil.get_version(), Some(Version::Nil));
    assert_eq!(not_nil.get_version(), Some(Version::Random))
}

//#[test]
pub fn test_predefined_namespaces() {
    assert_eq!(
        Uuid::NAMESPACE_DNS.to_hyphenated().to_string(),
        "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_URL.to_hyphenated().to_string(),
        "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_OID.to_hyphenated().to_string(),
        "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
    );
    assert_eq!(
        Uuid::NAMESPACE_X500.to_hyphenated().to_string(),
        "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
    );
}

//#[cfg(feature = "v3")]
//#[test]
pub fn test_get_version_v3() {
    let uuid =
        Uuid::new_v3(&Uuid::NAMESPACE_DNS, "rust-lang.org".as_bytes());

    assert_eq!(uuid.get_version().unwrap(), Version::Md5);
    assert_eq!(uuid.get_version_num(), 3);
}

//#[test]
pub fn test_get_variant() {
    let uuid1 = test_util::new();
    let uuid2 =
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid3 =
        Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
    let uuid4 =
        Uuid::parse_str("936DA01F9ABD4d9dC0C702AF85C822A8").unwrap();
    let uuid5 =
        Uuid::parse_str("F9168C5E-CEB2-4faa-D6BF-329BF39FA1E4").unwrap();
    let uuid6 =
        Uuid::parse_str("f81d4fae-7dec-11d0-7765-00a0c91e6bf6").unwrap();

    assert_eq!(uuid1.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid2.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid3.get_variant().unwrap(), Variant::RFC4122);
    assert_eq!(uuid4.get_variant().unwrap(), Variant::Microsoft);
    assert_eq!(uuid5.get_variant().unwrap(), Variant::Microsoft);
    assert_eq!(uuid6.get_variant().unwrap(), Variant::NCS);
}

//#[test]
pub fn test_parse_uuid_v4() {
    use uuid::adapter;
    use uuid::parser;

    const EXPECTED_UUID_LENGTHS: parser::Expected =
        parser::Expected::Any(&[
            adapter::Hyphenated::LENGTH,
            adapter::Simple::LENGTH,
        ]);

    const EXPECTED_GROUP_COUNTS: parser::Expected =
        parser::Expected::Any(&[1, 5]);

    const EXPECTED_CHARS: &'static str = "0123456789abcdefABCDEF-";

    // Invalid
    assert_eq!(
        Uuid::parse_str(""),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 0,
        })
    );

    assert_eq!(
        Uuid::parse_str("!"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 1
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 37,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 35
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4"),
        Err(parser::ParseError::InvalidCharacter {
            expected: EXPECTED_CHARS,
            found: 'G',
            index: 20,
            urn: parser::UrnPrefix::Optional,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4"),
        Err(parser::ParseError::InvalidGroupCount {
            expected: EXPECTED_GROUP_COUNTS,
            found: 2
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faaFB6BFF329BF39FA1E4"),
        Err(parser::ParseError::InvalidGroupCount {
            expected: EXPECTED_GROUP_COUNTS,
            found: 3,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4"),
        Err(parser::ParseError::InvalidGroupCount {
            expected: EXPECTED_GROUP_COUNTS,
            found: 4,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 18,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4"),
        Err(parser::ParseError::InvalidCharacter {
            expected: EXPECTED_CHARS,
            found: 'X',
            index: 18,
            urn: parser::UrnPrefix::Optional,
        })
    );

    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4"),
        Err(parser::ParseError::InvalidGroupLength {
            expected: parser::Expected::Exact(4),
            found: 3,
            group: 1,
        })
    );
    // (group, found, expecting)
    //
    assert_eq!(
        Uuid::parse_str("01020304-1112-2122-3132-41424344"),
        Err(parser::ParseError::InvalidGroupLength {
            expected: parser::Expected::Exact(12),
            found: 8,
            group: 4,
        })
    );

    assert_eq!(
        Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 31,
        })
    );

    assert_eq!(
        Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c88"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 33,
        })
    );

    assert_eq!(
        Uuid::parse_str("67e5504410b1426f9247bb680e5fe0cg8"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 33,
        })
    );

    assert_eq!(
        Uuid::parse_str("67e5504410b1426%9247bb680e5fe0c8"),
        Err(parser::ParseError::InvalidCharacter {
            expected: EXPECTED_CHARS,
            found: '%',
            index: 15,
            urn: parser::UrnPrefix::Optional,
        })
    );

    assert_eq!(
        Uuid::parse_str("231231212212423424324323477343246663"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 36,
        })
    );

    // Valid
    assert!(Uuid::parse_str("00000000000000000000000000000000").is_ok());
    assert!(Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok());
    assert!(Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4").is_ok());
    assert!(Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8").is_ok());
    assert!(Uuid::parse_str("01020304-1112-2122-3132-414243444546").is_ok());
    assert!(Uuid::parse_str(
        "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8"
    )
    .is_ok());

    // Nil
    let nil = Uuid::nil();
    assert_eq!(
        Uuid::parse_str("00000000000000000000000000000000").unwrap(),
        nil
    );
    assert_eq!(
        Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
        nil
    );

    // Round-trip
    let uuid_orig = test_util::new();
    let orig_str = uuid_orig.to_string();
    let uuid_out = Uuid::parse_str(&orig_str).unwrap();
    assert_eq!(uuid_orig, uuid_out);

    // Test error reporting
    assert_eq!(
        Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
        Err(parser::ParseError::InvalidLength {
            expected: EXPECTED_UUID_LENGTHS,
            found: 31,
        })
    );
    assert_eq!(
        Uuid::parse_str("67e550X410b1426f9247bb680e5fe0cd"),
        Err(parser::ParseError::InvalidCharacter {
            expected: EXPECTED_CHARS,
            found: 'X',
            index: 6,
            urn: parser::UrnPrefix::Optional,
        })
    );
    assert_eq!(
        Uuid::parse_str("67e550-4105b1426f9247bb680e5fe0c"),
        Err(parser::ParseError::InvalidGroupLength {
            expected: parser::Expected::Exact(8),
            found: 6,
            group: 0,
        })
    );
    assert_eq!(
        Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4"),
        Err(parser::ParseError::InvalidGroupLength {
            expected: parser::Expected::Exact(4),
            found: 5,
            group: 3,
        })
    );
}

//#[test]
pub fn test_to_simple_string() {
    let uuid1 = test_util::new();
    let s = uuid1.to_simple().to_string();

    assert_eq!(s.len(), 32);
    assert!(s.chars().all(|c| c.is_digit(16)));
}

//#[test]
pub fn test_to_hyphenated_string() {
    let uuid1 = test_util::new();
    let s = uuid1.to_hyphenated().to_string();

    assert!(s.len() == 36);
    assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
}

//#[test]
pub fn test_upper_lower_hex() {
    use std::fmt::Write;

    let mut buf = String::new();
    let u = test_util::new();

    macro_rules! check {
        ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
            $buf.clear();
            write!($buf, $format, $target).unwrap();
            assert!(buf.len() == $len);
            assert!($buf.chars().all($cond), "{}", $buf);
        };
    }

    check!(buf, "{:X}", u, 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:X}", u.to_hyphenated(), 36, |c| c.is_uppercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:X}", u.to_simple(), 32, |c| c.is_uppercase()
        || c.is_digit(10));

    check!(buf, "{:x}", u.to_hyphenated(), 36, |c| c.is_lowercase()
        || c.is_digit(10)
        || c == '-');
    check!(buf, "{:x}", u.to_simple(), 32, |c| c.is_lowercase()
        || c.is_digit(10));
}

//#[test]
pub fn test_to_urn_string() {
    let uuid1 = test_util::new();
    let ss = uuid1.to_urn().to_string();
    let s = &ss[9..];

    assert!(ss.starts_with("urn:uuid:"));
    assert_eq!(s.len(), 36);
    assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
}

//#[test]
pub fn test_to_simple_string_matching() {
    let uuid1 = test_util::new();

    let hs = uuid1.to_hyphenated().to_string();
    let ss = uuid1.to_simple().to_string();

    let hsn = hs.chars().filter(|&c| c != '-').collect::<String>();

    assert_eq!(hsn, ss);
}

//#[test]
pub fn test_string_roundtrip() {
    let uuid = test_util::new();

    let hs = uuid.to_hyphenated().to_string();
    let uuid_hs = Uuid::parse_str(&hs).unwrap();
    assert_eq!(uuid_hs, uuid);

    let ss = uuid.to_string();
    let uuid_ss = Uuid::parse_str(&ss).unwrap();
    assert_eq!(uuid_ss, uuid);
}

//#[test]
pub fn test_from_fields() {
    let d1: u32 = 0xa1a2a3a4;
    let d2: u16 = 0xb1b2;
    let d3: u16 = 0xc1c2;
    let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1, d2, d3, &d4).unwrap();

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_from_fields_le() {
    let d1: u32 = 0xa4a3a2a1;
    let d2: u16 = 0xb2b1;
    let d3: u16 = 0xc2c1;
    let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields_le(d1, d2, d3, &d4).unwrap();

    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    let result = u.to_simple().to_string();
    assert_eq!(result, expected);
}

//#[test]
pub fn test_as_fields() {
    let u = test_util::new();
    let (d1, d2, d3, d4) = u.as_fields();

    assert_ne!(d1, 0);
    assert_ne!(d2, 0);
    assert_ne!(d3, 0);
    assert_eq!(d4.len(), 8);
    assert!(!d4.iter().all(|&b| b == 0));
}

//#[test]
pub fn test_fields_roundtrip() {
    let d1_in: u32 = 0xa1a2a3a4;
    let d2_in: u16 = 0xb1b2;
    let d3_in: u16 = 0xc1c2;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.as_fields();

    assert_eq!(d1_in, d1_out);
    assert_eq!(d2_in, d2_out);
    assert_eq!(d3_in, d3_out);
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_fields_le_roundtrip() {
    let d1_in: u32 = 0xa4a3a2a1;
    let d2_in: u16 = 0xb2b1;
    let d3_in: u16 = 0xc2c1;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields_le(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.to_fields_le();

    assert_eq!(d1_in, d1_out);
    assert_eq!(d2_in, d2_out);
    assert_eq!(d3_in, d3_out);
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_fields_le_are_actually_le() {
    let d1_in: u32 = 0xa1a2a3a4;
    let d2_in: u16 = 0xb1b2;
    let d3_in: u16 = 0xc1c2;
    let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

    let u = Uuid::from_fields(d1_in, d2_in, d3_in, d4_in).unwrap();
    let (d1_out, d2_out, d3_out, d4_out) = u.to_fields_le();

    assert_eq!(d1_in, d1_out.swap_bytes());
    assert_eq!(d2_in, d2_out.swap_bytes());
    assert_eq!(d3_in, d3_out.swap_bytes());
    assert_eq!(d4_in, d4_out);
}

//#[test]
pub fn test_from_slice() {
    let b = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_slice(&b).unwrap();
    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

    assert_eq!(u.to_simple().to_string(), expected);
}

//#[test]
pub fn test_from_bytes() {
    let b = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_bytes(b);
    let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

    assert_eq!(u.to_simple().to_string(), expected);
}

//#[test]
pub fn test_as_bytes() {
    let u = test_util::new();
    let ub = u.as_bytes();

    assert_eq!(ub.len(), 16);
    assert!(!ub.iter().all(|&b| b == 0));
}

//#[test]
pub fn test_bytes_roundtrip() {
    let b_in: uuid::Bytes = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
        0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    ];

    let u = Uuid::from_slice(&b_in).unwrap();

    let b_out = u.as_bytes();

    assert_eq!(&b_in, b_out);
}

//#[test]
pub fn test_iterbytes_impl_for_uuid() {
    let mut set = std::collections::HashSet::new();
    let id1 = test_util::new();
    let id2 = test_util::new2();
    set.insert(id1.clone());

    assert!(set.contains(&id1));
    assert!(!set.contains(&id2));
}
