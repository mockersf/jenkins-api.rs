//! helper traits and macros

/// Trait to implement to match the _class provided by Jenkins
pub trait Class {
    /// Should reply the _class provided by Jenkins for a type
    fn with_class() -> &'static str;
}

macro_rules! register_class {
    ($class:expr => $variant:ty) => {
        impl Class for $variant {
            fn with_class() -> &'static str {
                $class
            }
        }
    };
}

macro_rules! specialize {
    ($common:ty => $trait:path) => {
        impl $common {
            #[doc = "Read the object as one of it's specialization implementing $trait"]
            pub fn as_variant<T: Class + $trait>(&self) -> Result<T, serde_json::Error>
            where
                for<'de> T: Deserialize<'de>,
            {
                let value = serde_json::to_value(self)?;
                match self.class {
                    Some(ref class) if class == T::with_class() => {
                        serde_json::from_value::<T>(value)
                    }
                    _ => {
                        return Err(serde::de::Error::custom(&format!(
                            r"invalid _class '{}', expected '{}'",
                            self.class
                                .clone()
                                .ok_or(serde::de::Error::custom("missing _class"))?,
                            T::with_class()
                        )))
                    }
                }
            }
        }
    };
}
