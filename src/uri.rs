use std::borrow::Cow;
use std::ffi::{CStr,CString};
use std::fmt;

use mongo_c_driver_wrapper::bindings;

/// Abstraction on top of MongoDB connection URI format.
/// See: http://api.mongodb.org/c/current/mongoc_uri_t.html

pub struct Uri {
    inner: *mut bindings::mongoc_uri_t
}

impl Uri {
    pub fn new<T: Into<Vec<u8>>>(uri_string: T) -> Uri {
        let uri_cstring = CString::new(uri_string).unwrap();
        let uri = unsafe { bindings::mongoc_uri_new(uri_cstring.as_ptr()) };
        Uri {
            inner: uri
        }
    }

    pub unsafe fn inner(&self) -> *const bindings::mongoc_uri_t {
        assert!(!self.inner.is_null());
        self.inner
    }

    pub fn as_str<'a>(&'a self) -> Cow<'a, str> {
        assert!(!self.inner.is_null());
        unsafe {
            let cstr = CStr::from_ptr(
                bindings::mongoc_uri_get_string(self.inner)
            );
            String::from_utf8_lossy(cstr.to_bytes())
        }
    }

    // TODO add various methods that are available on uri
}

impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Clone for Uri {
    fn clone(&self) -> Uri {
        Uri::new(self.as_str().into_owned())
    }
}

impl Drop for Uri {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        unsafe {
            bindings::mongoc_uri_destroy(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_uri() {
        let uri = super::Uri::new("mongodb://localhost:27017/");
        assert_eq!("mongodb://localhost:27017/", uri.as_str());
    }
}
