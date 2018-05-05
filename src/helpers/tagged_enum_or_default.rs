macro_rules! tagged_enum_or_default {
    ($(#[$attr:meta])* pub enum $name:ident {
        $($(#[$variant_attr:meta])* $variant:ident (_class = $key:expr) {
            $($(#[$field_attr:meta])* $field:ident: $type:ty,)*
        },)*
    }) => {
        $(#[$attr])*
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
            Default {
                /// _class provided by Jenkins
                class: Option<String>,
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
                                                )?
                                            );
                                        },)*
                                        _ => {
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
                    x => Ok($name::Default { class: x.tag }),
                }
            }
        }
    }
}
