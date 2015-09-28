extern crate cocoa;

#[cfg(test)]
mod foundation {
    mod nsstring {
        use cocoa::foundation::{NSString};
        use cocoa::base::{nil};
        use std::slice;
        use std::str;

        #[test]
        fn test_utf8() {
            let expected = "Iñtërnâtiônàlizætiøn";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                let bytes = built.UTF8String() as *const u8;
                let objc_string = str::from_utf8(slice::from_raw_parts(bytes, built.len())).unwrap();
                assert!(objc_string.len() == expected.len());
                assert!(objc_string == expected);
            }
        }

        #[test]
        fn test_string() {
            let expected = "Hello World!";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                let bytes = built.UTF8String() as *const u8;
                let objc_string = str::from_utf8(slice::from_raw_parts(bytes, built.len())).unwrap();
                assert!(objc_string.len() == expected.len());
                assert!(objc_string == expected);
            }
        }

        #[test]
        fn test_length() {
            let expected = "Hello!";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                assert!(built.len() == expected.len());
            }
        }

        #[test]
        fn test_append_by_appending_string() {
            let initial_str = "Iñtërnâtiônàlizætiøn";
            let to_append = "_more_strings";
            let expected = concat!("Iñtërnâtiônàlizætiøn", "_more_strings");
            unsafe {
                let built = NSString::alloc(nil).init_str(initial_str);
                let built_to_append = NSString::alloc(nil).init_str(to_append);
                let append_string = built.stringByAppendingString_(built_to_append);
                let bytes = append_string.UTF8String() as *const u8;
                let objc_string = str::from_utf8(slice::from_raw_parts(bytes, append_string.len())).unwrap();
                assert!(objc_string == expected);
            }
        }
    }
}
