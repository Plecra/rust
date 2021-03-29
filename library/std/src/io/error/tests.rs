use super::{Custom, Error, ErrorKind};
use crate::error;
use crate::fmt;
use crate::mem::size_of;
use crate::sys::decode_error_kind;
use crate::sys::os::error_string;

#[test]
fn test_size() {
    assert!(size_of::<Error>() <= size_of::<[usize; 2]>());
}

#[test]
fn test_debug_error() {
    let code = 6;
    let msg = error_string(code);
    let kind = decode_error_kind(code);
    let err = Error {
        repr: box Custom {
            kind: ErrorKind::InvalidInput,
            error: box Error::from_raw_os_error(code),
        },
    };
    let expected = format!(
        "Custom {{ \
         kind: InvalidInput, \
         error: Os {{ \
         code: {:?}, \
         kind: {:?}, \
         message: {:?} \
         }} \
         }}",
        code, kind, msg
    );
    assert_eq!(format!("{:?}", err), expected);
}

#[test]
fn test_downcasting() {
    #[derive(Debug)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("asdf")
        }
    }

    impl error::Error for TestError {}

    // we have to call all of these UFCS style right now since method
    // resolution won't implicitly drop the Send+Sync bounds
    let mut err = Error::new(ErrorKind::Other, TestError);
    assert!(err.get_ref().unwrap().is::<TestError>());
    assert_eq!("asdf", err.get_ref().unwrap().to_string());
    assert!(err.get_mut().unwrap().is::<TestError>());
    let extracted = err.into_inner().unwrap();
    extracted.downcast::<TestError>().unwrap();
}

#[test]
fn test_const() {
    const E: Error = Error::new_const(ErrorKind::NotFound, &"hello");

    assert_eq!(E.kind(), ErrorKind::NotFound);
    assert_eq!(E.to_string(), "hello");
    assert!(format!("{:?}", E).contains("\"hello\""));
    assert!(format!("{:?}", E).contains("NotFound"));
}
