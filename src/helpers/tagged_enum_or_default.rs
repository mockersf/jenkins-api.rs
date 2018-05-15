macro_rules! tagged_enum_or_default {
    // entry point when no common fields specified
    // exit point implementing deserialization
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident (_class = $key:expr) {
                    $(
                        $(#[$field_attr:meta])*
                        $field:ident: $type:ty
                    ),* $(,)*
                }
            ),* $(,)*
        }
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "cargo-clippy", allow(large_enum_variant))]
        #[derive(Debug)]
        pub enum $name {
            $(
                $(#[$variant_attr])*
                $variant {
                    $(
                        $(#[$field_attr])*
                        $field: $type,
                    )*
                },
            )*
            /// Default case used when none other matched
            Unknown {
                /// _class provided by Jenkins
                class: Option<String>,
            }
        }

        impl $name {
            #[allow(dead_code)]
            fn variant_name(&self) -> String {
                match *self {
                    $($name::$variant { .. } => stringify!($variant).to_string(),)*
                    $name::Unknown { class: Some(ref class), .. } => format!("Unknown({})", class),
                    $name::Unknown { .. } => "Unknown".to_string(),

                }
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let tag = deserializer
                    .deserialize_any(
                        ::helpers::from_serde::TaggedContentVisitor::<String>::new("_class")
                    )?;

                match tag {
                    $(::helpers::from_serde::TaggedContent {
                        ref tag,
                        ref content,
                    } if tag == &Some($key.to_string()) =>
                    {
                        let sub_deserializer =
                            ::helpers::from_serde::ContentDeserializer::<D::Error>::new(
                                content.clone()
                            );

                        #[derive(Debug)]
                        struct VariantVisitor<'de> {
                            marker: ::std::marker::PhantomData<$name>,
                            lifetime: ::std::marker::PhantomData<&'de $name>,
                        }

                        impl<'de> ::serde::de::Visitor<'de> for VariantVisitor<'de> {
                            type Value = $name;

                            fn expecting(
                                &self,
                                formatter: &mut ::serde::export::Formatter,
                            ) -> ::serde::export::fmt::Result {
                                ::serde::export::Formatter::write_str(formatter, "$variant")
                            }

                            #[inline]
                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                            where
                                A: ::serde::de::MapAccess<'de>,
                            {
                                $(let mut $field: Option<$type> = None;)*

                                while let Some(key) =
                                    try!(::serde::de::MapAccess::next_key::<String>(&mut map))
                                {
                                    match ::helpers::to_snake_case(&key).as_ref() {
                                        $(stringify!($field) => {
                                            $field = Some(
                                                ::serde::de::MapAccess::next_value::<$type>(
                                                    &mut map,
                                                ).map_err(|err| ::serde::de::Error::custom(
                                                    format!("{}.{}", stringify!($field), err)
                                                ))?
                                            );
                                        },)*
                                        "this_value_should_never_match_directly" | _ => {
                                            let _ = try!(::serde::de::MapAccess::next_value::<
                                                ::serde::de::IgnoredAny,
                                            >(&mut map));
                                        }
                                    }
                                }

                                Ok($name::$variant {
                                    $($field: $field.ok_or(
                                        ::serde::de::Error::missing_field(stringify!($field))
                                    )?,)*
                                })
                            }
                        }
                        sub_deserializer.deserialize_any(VariantVisitor {
                            marker: ::std::marker::PhantomData,
                            lifetime: ::std::marker::PhantomData,
                        })
                    },)*
                    x => Ok($name::Unknown { class: x.tag }),
                }
            }
        }
    };

    // entry point for no common fields
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            common_fields {};
            $(
                $(#[$variant_attr:meta])*
                $variant:ident (_class = $key:expr) $variant_fields:tt
            ),* $(,)*
        }
    ) => {
        tagged_enum_or_default!(
            $(#[$attr])*
            pub enum $name {
                $(
                    $(#[$variant_attr])*
                    $variant (_class = $key) $variant_fields,
                )*
            }
        );
    };

    // entry point for one common field
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            common_fields {
                #[doc=$first_common_doc:expr]
                $first_common_field:ident: $first_common_type:ty $(,)*
            };
            $(
                $(#[$variant_attr:meta])*
                $variant:ident (_class = $key:expr) $variant_fields:tt
            ),* $(,)*
        }
    ) => {
        tagged_enum_or_default!(
            $(#[$attr])*
            pub enum $name {
                common_fields {};
                $(
                    $(#[$variant_attr])*
                    $variant (_class = $key) $variant_fields {
                        #[doc=$first_common_doc]
                        $first_common_field: $first_common_type
                    },
                )*
            }
        );
    };

    // entry point for multiple common fields
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            common_fields {
                #[doc=$first_common_doc:expr]
                $first_common_field:ident: $first_common_type:ty,
                $(
                    #[doc=$common_doc:expr]
                    $common_field:ident: $common_type:ty
                ),+ $(,)*
            };
            $(
                $(#[$variant_attr:meta])*
                $variant:ident (_class = $key:expr) $variant_fields:tt
            ),* $(,)*
        }
    ) => {
        tagged_enum_or_default!(
            $(#[$attr])*
            pub enum $name {
                common_fields {
                    $(
                        #[doc=$common_doc]
                        $common_field: $common_type,
                    )*
                };
                $(
                    $(#[$variant_attr])*
                    $variant (_class = $key) $variant_fields {
                        #[doc=$first_common_doc]
                        $first_common_field: $first_common_type
                    },
                )*
            }
        );
    };

    // internal part of the macro, called to add common fields to each variant one by one
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            common_fields $common_fields:tt;
            $(
                $(#[$variant_attr:meta])*
                $variant:ident (_class = $key:expr) {
                    $(
                        $(#[$field_attr:meta])*
                        $field:ident: $type:ty
                    ),* $(,)*
                } {
                    #[doc=$adding_doc:expr]
                    $adding_field:ident: $adding_type:ty
                },
            )*
        }
    ) => {
        tagged_enum_or_default!(
            $(#[$attr])*
            pub enum $name {
                common_fields $common_fields;
                $(
                    $(#[$variant_attr])*
                    $variant (_class = $key) {
                        $(
                            $(#[$field_attr])*
                            $field: $type,
                        )*
                        #[doc=$adding_doc]
                        $adding_field: $adding_type,
                    },
                )*
            }
        );
    };
}

#[cfg(test)]
mod tests {
    use serde::Deserializer;

    #[test]
    fn enum_can_be_debugged() {
        tagged_enum_or_default!(
            pub enum Test {
                common_fields {
                    /// my first common field
                    c1: u8,
                };
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8,
                },
            }
        );

        let t1 = Test::Variant1 {
            v1: 0,
            v2: 1,
            c1: 2,
        };
        assert_eq!(format!("{:?}", t1), "Variant1 { v1: 0, v2: 1, c1: 2 }");
    }

    #[test]
    fn enum_no_common_fields() {
        tagged_enum_or_default!(
            pub enum Test {
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8,
                },
            }
        );

        Test::Variant1 {
            v1: 0,
            v2: 1,
        };
    }

    #[test]
    fn enum_empty_common_fields() {
        tagged_enum_or_default!(
            pub enum Test {
                common_fields {};
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8,
                },
            }
        );

        Test::Variant1 {
            v1: 0,
            v2: 1,
        };
    }

    #[test]
    fn enum_one_common_field() {
        tagged_enum_or_default!(
            pub enum Test {
                common_fields {
                    /// my first common field
                    c1: u8,
                };
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8,
                },
            }
        );

        Test::Variant1 {
            v1: 0,
            v2: 1,
            c1: 2,
        };
    }

    #[test]
    fn enum_many_common_fields() {
        tagged_enum_or_default!(
            pub enum Test {
                common_fields {
                    /// my first common field
                    c1: u8,
                    /// my second common field
                    c2: u8,
                };
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8,
                },
            }
        );

        Test::Variant1 {
            v1: 0,
            v2: 1,
            c1: 2,
            c2: 3,
        };
    }

    #[test]
    fn enum_no_trailing_commas() {
        tagged_enum_or_default!(
            pub enum Test {
                common_fields {
                    /// my first common field
                    c1: u8,
                    /// my second common field
                    c2: u8
                };
                Variant1 (_class = "variant1") {
                    v1: u8,
                    v2: u8
                }
            }
        );

        Test::Variant1 {
            v1: 0,
            v2: 1,
            c1: 2,
            c2: 3,
        };
    }
}
