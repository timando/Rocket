//! UUID parameter and form value parsing support.
//!
//! See the [`Uuid`] type for further details.
//!
//! # Enabling
//!
//! This module is only available when the `uuid` feature is enabled. Enable it
//! in `Cargo.toml` as follows:
//!
//! ```toml
//! [dependencies.rocket_contrib]
//! version = "0.4.5"
//! default-features = false
//! features = ["uuid"]
//! ```
#[cfg(feature="uuid")]
pub extern crate uuid as uuid_crate;
#[cfg(feature="uuid_08")]
pub extern crate uuid_08;

use std::fmt;
use std::str::FromStr;
use std::ops::Deref;

use rocket::request::{FromParam, FromFormValue};
use rocket::http::RawStr;

#[cfg(feature="uuid")]
pub use self::uuid_crate::parser::ParseError;
#[cfg(feature="uuid_08")]
pub use self::uuid_08::Error as Error08;


/// Implements [`FromParam`] and [`FromFormValue`] for accepting UUID values.
///
/// # Usage
///
/// To use, add the `uuid` feature to the `rocket_contrib` dependencies section
/// of your `Cargo.toml`:
///
/// ```toml
/// [dependencies.rocket_contrib]
/// version = "0.4.5"
/// default-features = false
/// features = ["uuid"]
/// ```
///
/// You can use the `Uuid` type directly as a target of a dynamic parameter:
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// # #[macro_use] extern crate rocket_contrib;
/// use rocket_contrib::uuid::Uuid;
///
/// #[get("/users/<id>")]
/// fn user(id: Uuid) -> String {
///     format!("We found: {}", id)
/// }
/// ```
///
/// You can also use the `Uuid` as a form value, including in query strings:
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// # #[macro_use] extern crate rocket_contrib;
/// use rocket_contrib::uuid::Uuid;
///
/// #[get("/user?<id>")]
/// fn user(id: Uuid) -> String {
///     format!("User ID: {}", id)
/// }
#[cfg(feature="uuid")]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uuid(uuid_crate::Uuid);

#[cfg(feature="uuid")]
impl Uuid {
    /// Consumes the Uuid wrapper, returning the underlying `Uuid` type.
    ///
    /// # Example
    /// ```rust
    /// # extern crate rocket_contrib;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// use rocket_contrib::uuid::{uuid_crate, Uuid};
    ///
    /// let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
    /// let real_uuid = uuid_crate::Uuid::from_str(uuid_str).unwrap();
    /// let my_inner_uuid = Uuid::from_str(uuid_str)
    ///     .expect("valid UUID string")
    ///     .into_inner();
    ///
    /// assert_eq!(real_uuid, my_inner_uuid);
    /// # }
    /// ```
    #[inline(always)]
    pub fn into_inner(self) -> uuid_crate::Uuid {
        self.0
    }
}

#[cfg(feature="uuid")]
impl fmt::Display for Uuid {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature="uuid")]
impl<'a> FromParam<'a> for Uuid {
    type Error = ParseError;

    /// A value is successfully parsed if `param` is a properly formatted Uuid.
    /// Otherwise, a `ParseError` is returned.
    #[inline(always)]
    fn from_param(param: &'a RawStr) -> Result<Uuid, Self::Error> {
        param.parse()
    }
}

#[cfg(feature="uuid")]
impl<'v> FromFormValue<'v> for Uuid {
    type Error = &'v RawStr;

    /// A value is successfully parsed if `form_value` is a properly formatted
    /// Uuid. Otherwise, the raw form value is returned.
    #[inline(always)]
    fn from_form_value(form_value: &'v RawStr) -> Result<Uuid, &'v RawStr> {
        form_value.parse().map_err(|_| form_value)
    }
}

#[cfg(feature="uuid")]
impl FromStr for Uuid {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Uuid, Self::Err> {
        Ok(Uuid(try!(s.parse())))
    }
}

#[cfg(feature="uuid")]
impl Deref for Uuid {
    type Target = uuid_crate::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature="uuid")]
impl PartialEq<uuid_crate::Uuid> for Uuid {
    #[inline(always)]
    fn eq(&self, other: &uuid_crate::Uuid) -> bool {
        self.0.eq(other)
    }
}


/// Implements [`FromParam`] and [`FromFormValue`] for accepting UUID values.
///
/// # Usage
///
/// To use, add the `uuid` feature to the `rocket_contrib` dependencies section
/// of your `Cargo.toml`:
///
/// ```toml
/// [dependencies.rocket_contrib]
/// version = "0.4.5"
/// default-features = false
/// features = ["uuid"]
/// ```
///
/// You can use the `Uuid` type directly as a target of a dynamic parameter:
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// # #[macro_use] extern crate rocket_contrib;
/// use rocket_contrib::uuid::Uuid;
///
/// #[get("/users/<id>")]
/// fn user(id: Uuid) -> String {
///     format!("We found: {}", id)
/// }
/// ```
///
/// You can also use the `Uuid` as a form value, including in query strings:
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// # #[macro_use] extern crate rocket_contrib;
/// use rocket_contrib::uuid::Uuid;
///
/// #[get("/user?<id>")]
/// fn user(id: Uuid) -> String {
///     format!("User ID: {}", id)
/// }
#[cfg(feature="uuid_08")]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uuid08(pub uuid_08::Uuid);

#[cfg(feature="uuid_08")]
impl Uuid08 {
    /// Consumes the Uuid wrapper, returning the underlying `Uuid` type.
    ///
    /// # Example
    /// ```rust
    /// # extern crate rocket_contrib;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// use rocket_contrib::uuid::{uuid_crate, Uuid};
    ///
    /// let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
    /// let real_uuid = uuid_crate::Uuid::from_str(uuid_str).unwrap();
    /// let my_inner_uuid = Uuid::from_str(uuid_str)
    ///     .expect("valid UUID string")
    ///     .into_inner();
    ///
    /// assert_eq!(real_uuid, my_inner_uuid);
    /// # }
    /// ```
    #[inline(always)]
    pub fn into_inner(self) -> uuid_08::Uuid {
        self.0
    }
}

#[cfg(feature="uuid_08")]
impl fmt::Display for Uuid08 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature="uuid_08")]
impl<'a> FromParam<'a> for Uuid08 {
    type Error = Error08;

    /// A value is successfully parsed if `param` is a properly formatted Uuid.
    /// Otherwise, a `ParseError` is returned.
    #[inline(always)]
    fn from_param(param: &'a RawStr) -> Result<Uuid08, Self::Error> {
        param.parse()
    }
}

#[cfg(feature="uuid_08")]
impl<'v> FromFormValue<'v> for Uuid08 {
    type Error = &'v RawStr;

    /// A value is successfully parsed if `form_value` is a properly formatted
    /// Uuid. Otherwise, the raw form value is returned.
    #[inline(always)]
    fn from_form_value(form_value: &'v RawStr) -> Result<Uuid08, &'v RawStr> {
        form_value.parse().map_err(|_| form_value)
    }
}

#[cfg(feature="uuid_08")]
impl FromStr for Uuid08 {
    type Err = Error08;

    #[inline]
    fn from_str(s: &str) -> Result<Uuid08, Self::Err> {
        Ok(Uuid08(try!(s.parse())))
    }
}

#[cfg(feature="uuid_08")]
impl Deref for Uuid08 {
    type Target = uuid_08::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature="uuid_08")]
impl PartialEq<uuid_08::Uuid> for Uuid08 {
    #[inline(always)]
    fn eq(&self, other: &uuid_08::Uuid) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod test {
    use super::uuid_crate;
    use super::Uuid;
    use super::FromParam;
    use super::FromStr;

    #[test]
    fn test_from_str() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_str(uuid_str).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_from_param() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str.into()).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_into_inner() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str.into()).unwrap();
        let real_uuid: uuid_crate::Uuid = uuid_str.parse().unwrap();
        let inner_uuid: uuid_crate::Uuid = uuid_wrapper.into_inner();
        assert_eq!(real_uuid, inner_uuid)
    }

    #[test]
    fn test_partial_eq() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str.into()).unwrap();
        let real_uuid: uuid_crate::Uuid = uuid_str.parse().unwrap();
        assert_eq!(uuid_wrapper, real_uuid)
    }

    #[test]
    #[should_panic(expected = "InvalidLength")]
    fn test_from_param_invalid() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2p";
        Uuid::from_param(uuid_str.into()).unwrap();
    }
}
