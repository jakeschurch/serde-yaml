use value::Yaml;

impl<'a> PartialEq for Yaml<'a> {
    fn eq(&self, other: &Yaml) -> bool {
        match (self, other) {
            (&Yaml::Null, &Yaml::Null) => true,
            (&Yaml::Bool(a), &Yaml::Bool(b)) => a == b,
            (&Yaml::Number(ref a), &Yaml::Number(ref b)) => a == b,
            (&Yaml::String(ref a), &Yaml::String(ref b)) => a == b,
            (&Yaml::Sequence(ref a), &Yaml::Sequence(ref b)) => a == b,
            (&Yaml::Mapping(ref a), &Yaml::Mapping(ref b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq<str> for Yaml {
    /// Compare `str` with YAML value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!(Yaml::Scalar("lorem".into()) == *"lorem");
    /// ```
    fn eq(&self, other: &str) -> bool {
        self.as_str().map_or(false, |s| s == other)
    }
}

impl<'a> PartialEq<&'a str> for Yaml {
    /// Compare `&str` with YAML value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!(Yaml::Scalar("lorem".into()) == "lorem");
    /// ```
    fn eq(&self, other: &&str) -> bool {
        self.as_str().map_or(false, |s| s == *other)
    }
}

impl PartialEq<Yaml> for str {
    /// Compare YAML value with `str`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!(*"lorem" == Yaml::Scalar("lorem".into()));
    /// ```
    fn eq(&self, other: &Yaml) -> bool {
        other.as_str().map_or(false, |s| s == self)
    }
}

impl<'a> PartialEq<Yaml> for &'a str {
    /// Compare `&str` with YAML value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!("lorem" == Yaml::Scalar("lorem".into()));
    /// ```
    fn eq(&self, other: &Yaml) -> bool {
        other.as_str().map_or(false, |s| s == *self)
    }
}

impl PartialEq<String> for Yaml {
    /// Compare YAML value with String
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!(Yaml::Scalar("lorem".into()) == "lorem".to_string());
    /// ```
    fn eq(&self, other: &String) -> bool {
        self.as_str().map_or(false, |s| s == other)
    }
}

impl PartialEq<Yaml> for String {
    /// Compare `String` with YAML value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde_yaml::Yaml;
    /// assert!("lorem".to_string() == Yaml::Scalar("lorem".into()));
    /// ```
    fn eq(&self, other: &Yaml) -> bool {
        other.as_str().map_or(false, |s| s == self)
    }
}

macro_rules! partialeq_numeric {
    ($([$($ty:ty)*], $conversion:ident, $base:ty)*) => {
        $($(
            impl PartialEq<$ty> for Yaml {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| i == (*other as $base))
                }
            }

            impl PartialEq<Yaml> for $ty {
                fn eq(&self, other: &Yaml) -> bool {
                    other.$conversion().map_or(false, |i| i == (*self as $base))
                }
            }

            impl<'a> PartialEq<$ty> for &'a Yaml {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| i == (*other as $base))
                }
            }

            impl<'a> PartialEq<$ty> for &'a mut Yaml {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| i == (*other as $base))
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    [i8 i16 i32 i64 isize], as_i64, i64
    [u8 u16 u32 usize], as_i64, i64
    [f32 f64], as_f64, f64
}
