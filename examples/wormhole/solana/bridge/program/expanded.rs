#![feature(prelude_import)]
#![feature(generic_const_exprs, adt_const_params)]
#![allow(non_upper_case_globals)]
#![allow(incomplete_features)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use solitaire::*;
pub const MAX_LEN_GUARDIAN_KEYS: usize = 19;
pub const CHAIN_ID_SOLANA: u16 = 1;
pub mod accounts {
    pub mod bridge {
        //! The Bridge account contains the main state for the wormhole bridge, as well as tracking
        //! configuration options for how the bridge should behave.
        use borsh::{BorshDeserialize, BorshSerialize};
        use serde::{Deserialize, Serialize};
        use solitaire::{AccountOwner, AccountState, Data, Derive, Owned};
        pub type Bridge<'a, const State: AccountState> =
            Derive<Data<'a, BridgeData, { State }>, "Bridge">;
        pub struct BridgeData {
            /// The current guardian set index, used to decide which signature sets to accept.
            pub guardian_set_index: u32,
            /// Lamports in the collection account
            pub last_lamports: u64,
            /// Bridge configuration, which is set once upon initialization.
            pub config: BridgeConfig,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeData {
            #[inline]
            fn clone(&self) -> BridgeData {
                match *self {
                    BridgeData {
                        guardian_set_index: ref __self_0_0,
                        last_lamports: ref __self_0_1,
                        config: ref __self_0_2,
                    } => BridgeData {
                        guardian_set_index: ::core::clone::Clone::clone(&(*__self_0_0)),
                        last_lamports: ::core::clone::Clone::clone(&(*__self_0_1)),
                        config: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgeData {
            #[inline]
            fn default() -> BridgeData {
                BridgeData {
                    guardian_set_index: ::core::default::Default::default(),
                    last_lamports: ::core::default::Default::default(),
                    config: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for BridgeData
        where
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            BridgeConfig: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.guardian_set_index, writer)?;
                borsh::BorshSerialize::serialize(&self.last_lamports, writer)?;
                borsh::BorshSerialize::serialize(&self.config, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for BridgeData
        where
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            BridgeConfig: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    guardian_set_index: borsh::BorshDeserialize::deserialize(buf)?,
                    last_lamports: borsh::BorshDeserialize::deserialize(buf)?,
                    config: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for BridgeData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "BridgeData",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guardian_set_index",
                        &self.guardian_set_index,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "last_lamports",
                        &self.last_lamports,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "config",
                        &self.config,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for BridgeData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "guardian_set_index" => _serde::__private::Ok(__Field::__field0),
                                "last_lamports" => _serde::__private::Ok(__Field::__field1),
                                "config" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"guardian_set_index" => _serde::__private::Ok(__Field::__field0),
                                b"last_lamports" => _serde::__private::Ok(__Field::__field1),
                                b"config" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<BridgeData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = BridgeData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct BridgeData",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct BridgeData with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u64>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct BridgeData with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                BridgeConfig,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct BridgeData with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(BridgeData {
                                guardian_set_index: __field0,
                                last_lamports: __field1,
                                config: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u64> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<BridgeConfig> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guardian_set_index",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "last_lamports",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u64>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "config",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<BridgeConfig>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guardian_set_index")
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("last_lamports") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("config") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(BridgeData {
                                guardian_set_index: __field0,
                                last_lamports: __field1,
                                config: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["guardian_set_index", "last_lamports", "config"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "BridgeData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<BridgeData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[cfg(not(feature = "cpi"))]
        impl Owned for BridgeData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
        pub struct BridgeConfig {
            /// Period for how long a guardian set is valid after it has been replaced by a new one.  This
            /// guarantees that VAAs issued by that set can still be submitted for a certain period.  In
            /// this period we still trust the old guardian set.
            pub guardian_set_expiration_time: u32,
            /// Amount of lamports that needs to be paid to the protocol to post a message
            pub fee: u64,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeConfig {
            #[inline]
            fn clone(&self) -> BridgeConfig {
                match *self {
                    BridgeConfig {
                        guardian_set_expiration_time: ref __self_0_0,
                        fee: ref __self_0_1,
                    } => BridgeConfig {
                        guardian_set_expiration_time: ::core::clone::Clone::clone(&(*__self_0_0)),
                        fee: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgeConfig {
            #[inline]
            fn default() -> BridgeConfig {
                BridgeConfig {
                    guardian_set_expiration_time: ::core::default::Default::default(),
                    fee: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for BridgeConfig
        where
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.guardian_set_expiration_time, writer)?;
                borsh::BorshSerialize::serialize(&self.fee, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for BridgeConfig
        where
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    guardian_set_expiration_time: borsh::BorshDeserialize::deserialize(buf)?,
                    fee: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for BridgeConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "BridgeConfig",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guardian_set_expiration_time",
                        &self.guardian_set_expiration_time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "fee",
                        &self.fee,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for BridgeConfig {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "guardian_set_expiration_time" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "fee" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"guardian_set_expiration_time" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"fee" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<BridgeConfig>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = BridgeConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct BridgeConfig",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct BridgeConfig with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u64>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct BridgeConfig with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(BridgeConfig {
                                guardian_set_expiration_time: __field0,
                                fee: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u64> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guardian_set_expiration_time",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "fee",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u64>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "guardian_set_expiration_time",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("fee") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(BridgeConfig {
                                guardian_set_expiration_time: __field0,
                                fee: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["guardian_set_expiration_time", "fee"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "BridgeConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<BridgeConfig>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod claim {
        //! ClaimData accounts are one off markers that can be combined with other accounts to represent
        //! data that can only be used once.
        use borsh::{BorshDeserialize, BorshSerialize};
        use serde::{Deserialize, Serialize};
        use solitaire::{processors::seeded::Seeded, AccountOwner, AccountState, Data, Owned};
        pub type Claim<'a, const State: AccountState> = Data<'a, ClaimData, { State }>;
        pub struct ClaimData {
            pub claimed: bool,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for ClaimData {
            #[inline]
            fn default() -> ClaimData {
                ClaimData {
                    claimed: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ClaimData {
            #[inline]
            fn clone(&self) -> ClaimData {
                {
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ClaimData {}
        impl borsh::de::BorshDeserialize for ClaimData
        where
            bool: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    claimed: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for ClaimData
        where
            bool: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.claimed, writer)?;
                Ok(())
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ClaimData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ClaimData",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "claimed",
                        &self.claimed,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ClaimData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "claimed" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"claimed" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ClaimData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ClaimData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct ClaimData")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct ClaimData with 1 element",
                                            ),
                                        );
                                    }
                                };
                            _serde::__private::Ok(ClaimData { claimed: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "claimed",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<bool>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("claimed") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ClaimData { claimed: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["claimed"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ClaimData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ClaimData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Owned for ClaimData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
        pub struct ClaimDerivationData {
            pub emitter_address: [u8; 32],
            pub emitter_chain: u16,
            pub sequence: u64,
        }
        impl<'b, const State: AccountState> Seeded<&ClaimDerivationData> for Claim<'b, { State }> {
            fn seeds(data: &ClaimDerivationData) -> Vec<Vec<u8>> {
                return <[_]>::into_vec(box [
                    data.emitter_address.to_vec(),
                    data.emitter_chain.to_be_bytes().to_vec(),
                    data.sequence.to_be_bytes().to_vec(),
                ]);
            }
        }
    }
    pub mod fee_collector {
        //! The FeeCollector is a simple account that collects SOL fees.
        use solitaire::{Derive, Info};
        pub type FeeCollector<'a> = Derive<Info<'a>, "fee_collector">;
    }
    pub mod guardian_set {
        //! GuardianSet represents an account containing information about the current active guardians
        //! responsible for signing wormhole VAAs.
        use crate::types::GuardianPublicKey;
        use borsh::{BorshDeserialize, BorshSerialize};
        use serde::{Deserialize, Serialize};
        use solitaire::{processors::seeded::Seeded, AccountOwner, AccountState, Data, Owned};
        pub type GuardianSet<'b, const State: AccountState> = Data<'b, GuardianSetData, { State }>;
        pub struct GuardianSetData {
            /// Index representing an incrementing version number for this guardian set.
            pub index: u32,
            /// ETH style public keys
            pub keys: Vec<GuardianPublicKey>,
            /// Timestamp representing the time this guardian became active.
            pub creation_time: u32,
            /// Expiration time when VAAs issued by this set are no longer valid.
            pub expiration_time: u32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for GuardianSetData {
            #[inline]
            fn default() -> GuardianSetData {
                GuardianSetData {
                    index: ::core::default::Default::default(),
                    keys: ::core::default::Default::default(),
                    creation_time: ::core::default::Default::default(),
                    expiration_time: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for GuardianSetData
        where
            u32: borsh::ser::BorshSerialize,
            Vec<GuardianPublicKey>: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.index, writer)?;
                borsh::BorshSerialize::serialize(&self.keys, writer)?;
                borsh::BorshSerialize::serialize(&self.creation_time, writer)?;
                borsh::BorshSerialize::serialize(&self.expiration_time, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for GuardianSetData
        where
            u32: borsh::BorshDeserialize,
            Vec<GuardianPublicKey>: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    index: borsh::BorshDeserialize::deserialize(buf)?,
                    keys: borsh::BorshDeserialize::deserialize(buf)?,
                    creation_time: borsh::BorshDeserialize::deserialize(buf)?,
                    expiration_time: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for GuardianSetData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "GuardianSetData",
                        false as usize + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "index",
                        &self.index,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "keys",
                        &self.keys,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "creation_time",
                        &self.creation_time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "expiration_time",
                        &self.expiration_time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for GuardianSetData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "index" => _serde::__private::Ok(__Field::__field0),
                                "keys" => _serde::__private::Ok(__Field::__field1),
                                "creation_time" => _serde::__private::Ok(__Field::__field2),
                                "expiration_time" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"index" => _serde::__private::Ok(__Field::__field0),
                                b"keys" => _serde::__private::Ok(__Field::__field1),
                                b"creation_time" => _serde::__private::Ok(__Field::__field2),
                                b"expiration_time" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<GuardianSetData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GuardianSetData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GuardianSetData",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GuardianSetData with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Vec<GuardianPublicKey>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GuardianSetData with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GuardianSetData with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct GuardianSetData with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GuardianSetData {
                                index: __field0,
                                keys: __field1,
                                creation_time: __field2,
                                expiration_time: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<GuardianPublicKey>> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "index",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "keys",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<GuardianPublicKey>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "creation_time",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "expiration_time",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("index") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("keys") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("creation_time") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("expiration_time") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(GuardianSetData {
                                index: __field0,
                                keys: __field1,
                                creation_time: __field2,
                                expiration_time: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["index", "keys", "creation_time", "expiration_time"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GuardianSetData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GuardianSetData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        /// GuardianSet account PDAs are indexed by their version number.
        pub struct GuardianSetDerivationData {
            pub index: u32,
        }
        impl<'a, const State: AccountState> Seeded<&GuardianSetDerivationData>
            for GuardianSet<'a, { State }>
        {
            fn seeds(data: &GuardianSetDerivationData) -> Vec<Vec<u8>> {
                <[_]>::into_vec(box [
                    "GuardianSet".as_bytes().to_vec(),
                    data.index.to_be_bytes().to_vec(),
                ])
            }
        }
        impl GuardianSetData {
            /// Number of guardians in the set
            pub fn num_guardians(&self) -> u8 {
                self.keys.iter().filter(|v| **v != [0u8; 20]).count() as u8
            }
        }
        impl Owned for GuardianSetData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
    }
    pub mod posted_message {
        use borsh::{BorshDeserialize, BorshSerialize};
        use serde::{Deserialize, Serialize};
        use solana_program::pubkey::Pubkey;
        use solitaire::{AccountOwner, AccountState, Data, Owned};
        use std::{
            io::Write,
            ops::{Deref, DerefMut},
        };
        pub type PostedMessage<'a, const State: AccountState> =
            Data<'a, PostedMessageData, { State }>;
        #[repr(transparent)]
        pub struct PostedMessageData(pub MessageData);
        pub struct MessageData {
            /// Header of the posted VAA
            pub vaa_version: u8,
            /// Level of consistency requested by the emitter
            pub consistency_level: u8,
            /// Time the vaa was submitted
            pub vaa_time: u32,
            /// Account where signatures are stored
            pub vaa_signature_account: Pubkey,
            /// Time the posted message was created
            pub submission_time: u32,
            /// Unique nonce for this message
            pub nonce: u32,
            /// Sequence number of this message
            pub sequence: u64,
            /// Emitter of the message
            pub emitter_chain: u16,
            /// Emitter of the message
            pub emitter_address: [u8; 32],
            /// Message payload
            pub payload: Vec<u8>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for MessageData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MessageData {
                        vaa_version: ref __self_0_0,
                        consistency_level: ref __self_0_1,
                        vaa_time: ref __self_0_2,
                        vaa_signature_account: ref __self_0_3,
                        submission_time: ref __self_0_4,
                        nonce: ref __self_0_5,
                        sequence: ref __self_0_6,
                        emitter_chain: ref __self_0_7,
                        emitter_address: ref __self_0_8,
                        payload: ref __self_0_9,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "MessageData");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "vaa_version",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "consistency_level",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "vaa_time",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "vaa_signature_account",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "submission_time",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "nonce",
                            &&(*__self_0_5),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "sequence",
                            &&(*__self_0_6),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "emitter_chain",
                            &&(*__self_0_7),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "emitter_address",
                            &&(*__self_0_8),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "payload",
                            &&(*__self_0_9),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for MessageData {
            #[inline]
            fn default() -> MessageData {
                MessageData {
                    vaa_version: ::core::default::Default::default(),
                    consistency_level: ::core::default::Default::default(),
                    vaa_time: ::core::default::Default::default(),
                    vaa_signature_account: ::core::default::Default::default(),
                    submission_time: ::core::default::Default::default(),
                    nonce: ::core::default::Default::default(),
                    sequence: ::core::default::Default::default(),
                    emitter_chain: ::core::default::Default::default(),
                    emitter_address: ::core::default::Default::default(),
                    payload: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for MessageData
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            [u8; 32]: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.vaa_version, writer)?;
                borsh::BorshSerialize::serialize(&self.consistency_level, writer)?;
                borsh::BorshSerialize::serialize(&self.vaa_time, writer)?;
                borsh::BorshSerialize::serialize(&self.vaa_signature_account, writer)?;
                borsh::BorshSerialize::serialize(&self.submission_time, writer)?;
                borsh::BorshSerialize::serialize(&self.nonce, writer)?;
                borsh::BorshSerialize::serialize(&self.sequence, writer)?;
                borsh::BorshSerialize::serialize(&self.emitter_chain, writer)?;
                borsh::BorshSerialize::serialize(&self.emitter_address, writer)?;
                borsh::BorshSerialize::serialize(&self.payload, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MessageData
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            [u8; 32]: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    vaa_version: borsh::BorshDeserialize::deserialize(buf)?,
                    consistency_level: borsh::BorshDeserialize::deserialize(buf)?,
                    vaa_time: borsh::BorshDeserialize::deserialize(buf)?,
                    vaa_signature_account: borsh::BorshDeserialize::deserialize(buf)?,
                    submission_time: borsh::BorshDeserialize::deserialize(buf)?,
                    nonce: borsh::BorshDeserialize::deserialize(buf)?,
                    sequence: borsh::BorshDeserialize::deserialize(buf)?,
                    emitter_chain: borsh::BorshDeserialize::deserialize(buf)?,
                    emitter_address: borsh::BorshDeserialize::deserialize(buf)?,
                    payload: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for MessageData {
            #[inline]
            fn clone(&self) -> MessageData {
                match *self {
                    MessageData {
                        vaa_version: ref __self_0_0,
                        consistency_level: ref __self_0_1,
                        vaa_time: ref __self_0_2,
                        vaa_signature_account: ref __self_0_3,
                        submission_time: ref __self_0_4,
                        nonce: ref __self_0_5,
                        sequence: ref __self_0_6,
                        emitter_chain: ref __self_0_7,
                        emitter_address: ref __self_0_8,
                        payload: ref __self_0_9,
                    } => MessageData {
                        vaa_version: ::core::clone::Clone::clone(&(*__self_0_0)),
                        consistency_level: ::core::clone::Clone::clone(&(*__self_0_1)),
                        vaa_time: ::core::clone::Clone::clone(&(*__self_0_2)),
                        vaa_signature_account: ::core::clone::Clone::clone(&(*__self_0_3)),
                        submission_time: ::core::clone::Clone::clone(&(*__self_0_4)),
                        nonce: ::core::clone::Clone::clone(&(*__self_0_5)),
                        sequence: ::core::clone::Clone::clone(&(*__self_0_6)),
                        emitter_chain: ::core::clone::Clone::clone(&(*__self_0_7)),
                        emitter_address: ::core::clone::Clone::clone(&(*__self_0_8)),
                        payload: ::core::clone::Clone::clone(&(*__self_0_9)),
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for MessageData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "MessageData",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "vaa_version",
                        &self.vaa_version,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "consistency_level",
                        &self.consistency_level,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "vaa_time",
                        &self.vaa_time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "vaa_signature_account",
                        &self.vaa_signature_account,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "submission_time",
                        &self.submission_time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "nonce",
                        &self.nonce,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sequence",
                        &self.sequence,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "emitter_chain",
                        &self.emitter_chain,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "emitter_address",
                        &self.emitter_address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "payload",
                        &self.payload,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for MessageData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "vaa_version" => _serde::__private::Ok(__Field::__field0),
                                "consistency_level" => _serde::__private::Ok(__Field::__field1),
                                "vaa_time" => _serde::__private::Ok(__Field::__field2),
                                "vaa_signature_account" => _serde::__private::Ok(__Field::__field3),
                                "submission_time" => _serde::__private::Ok(__Field::__field4),
                                "nonce" => _serde::__private::Ok(__Field::__field5),
                                "sequence" => _serde::__private::Ok(__Field::__field6),
                                "emitter_chain" => _serde::__private::Ok(__Field::__field7),
                                "emitter_address" => _serde::__private::Ok(__Field::__field8),
                                "payload" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"vaa_version" => _serde::__private::Ok(__Field::__field0),
                                b"consistency_level" => _serde::__private::Ok(__Field::__field1),
                                b"vaa_time" => _serde::__private::Ok(__Field::__field2),
                                b"vaa_signature_account" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                b"submission_time" => _serde::__private::Ok(__Field::__field4),
                                b"nonce" => _serde::__private::Ok(__Field::__field5),
                                b"sequence" => _serde::__private::Ok(__Field::__field6),
                                b"emitter_chain" => _serde::__private::Ok(__Field::__field7),
                                b"emitter_address" => _serde::__private::Ok(__Field::__field8),
                                b"payload" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<MessageData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = MessageData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct MessageData",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct MessageData with 10 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct MessageData with 10 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<Pubkey>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<u64>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<u16>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<[u8; 32]>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<Vec<u8>>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct MessageData with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(MessageData {
                                vaa_version: __field0,
                                consistency_level: __field1,
                                vaa_time: __field2,
                                vaa_signature_account: __field3,
                                submission_time: __field4,
                                nonce: __field5,
                                sequence: __field6,
                                emitter_chain: __field7,
                                emitter_address: __field8,
                                payload: __field9,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u8> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u8> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Pubkey> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field6: _serde::__private::Option<u64> =
                                _serde::__private::None;
                            let mut __field7: _serde::__private::Option<u16> =
                                _serde::__private::None;
                            let mut __field8: _serde::__private::Option<[u8; 32]> =
                                _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Vec<u8>> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "vaa_version",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u8>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "consistency_level",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u8>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "vaa_time",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "vaa_signature_account",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Pubkey>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "submission_time",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "nonce",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sequence",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u64>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "emitter_chain",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u16>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "emitter_address",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<[u8; 32]>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "payload",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Vec<u8>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("vaa_version") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("consistency_level")
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("vaa_time") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "vaa_signature_account",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("submission_time") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("nonce") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("sequence") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("emitter_chain") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("emitter_address") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("payload") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(MessageData {
                                vaa_version: __field0,
                                consistency_level: __field1,
                                vaa_time: __field2,
                                vaa_signature_account: __field3,
                                submission_time: __field4,
                                nonce: __field5,
                                sequence: __field6,
                                emitter_chain: __field7,
                                emitter_address: __field8,
                                payload: __field9,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "vaa_version",
                        "consistency_level",
                        "vaa_time",
                        "vaa_signature_account",
                        "submission_time",
                        "nonce",
                        "sequence",
                        "emitter_chain",
                        "emitter_address",
                        "payload",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "MessageData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<MessageData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl BorshSerialize for PostedMessageData {
            fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write(b"msg")?;
                BorshSerialize::serialize(&self.0, writer)
            }
        }
        impl BorshDeserialize for PostedMessageData {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                *buf = &buf[3..];
                Ok(PostedMessageData(
                    <MessageData as BorshDeserialize>::deserialize(buf)?,
                ))
            }
        }
        impl Deref for PostedMessageData {
            type Target = MessageData;
            fn deref(&self) -> &Self::Target {
                unsafe { std::mem::transmute(&self.0) }
            }
        }
        impl DerefMut for PostedMessageData {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { std::mem::transmute(&mut self.0) }
            }
        }
        impl Default for PostedMessageData {
            fn default() -> Self {
                PostedMessageData(MessageData::default())
            }
        }
        impl Clone for PostedMessageData {
            fn clone(&self) -> Self {
                PostedMessageData(self.0.clone())
            }
        }
        #[cfg(not(feature = "cpi"))]
        impl Owned for PostedMessageData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
    }
    pub mod posted_vaa {
        use crate::MessageData;
        use borsh::{BorshDeserialize, BorshSerialize};
        use solitaire::{processors::seeded::Seeded, AccountOwner, AccountState, Data, Owned};
        use std::{
            io::Write,
            ops::{Deref, DerefMut},
        };
        pub type PostedVAA<'b, const State: AccountState> = Data<'b, PostedVAAData, { State }>;
        pub struct PostedVAADerivationData {
            pub payload_hash: Vec<u8>,
        }
        impl<'a, const State: AccountState> Seeded<&PostedVAADerivationData> for PostedVAA<'a, { State }> {
            fn seeds(data: &PostedVAADerivationData) -> Vec<Vec<u8>> {
                <[_]>::into_vec(box ["PostedVAA".as_bytes().to_vec(), data.payload_hash.to_vec()])
            }
        }
        #[repr(transparent)]
        pub struct PostedVAAData(pub MessageData);
        impl BorshSerialize for PostedVAAData {
            fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write(b"vaa")?;
                BorshSerialize::serialize(&self.0, writer)
            }
        }
        impl BorshDeserialize for PostedVAAData {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                *buf = &buf[3..];
                Ok(PostedVAAData(
                    <MessageData as BorshDeserialize>::deserialize(buf)?,
                ))
            }
        }
        impl Deref for PostedVAAData {
            type Target = MessageData;
            fn deref(&self) -> &Self::Target {
                unsafe { std::mem::transmute(&self.0) }
            }
        }
        impl DerefMut for PostedVAAData {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { std::mem::transmute(&mut self.0) }
            }
        }
        impl Default for PostedVAAData {
            fn default() -> Self {
                PostedVAAData(MessageData::default())
            }
        }
        impl Clone for PostedVAAData {
            fn clone(&self) -> Self {
                PostedVAAData(self.0.clone())
            }
        }
        #[cfg(not(feature = "cpi"))]
        impl Owned for PostedVAAData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
    }
    pub mod sequence {
        use borsh::{BorshDeserialize, BorshSerialize};
        use solana_program::pubkey::Pubkey;
        use solitaire::{processors::seeded::Seeded, AccountState, AccountOwner, Data, Owned};
        pub type Sequence<'b> = Data<'b, SequenceTracker, { AccountState::MaybeInitialized }>;
        pub struct SequenceTracker {
            pub sequence: u64,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SequenceTracker {
            #[inline]
            fn default() -> SequenceTracker {
                SequenceTracker {
                    sequence: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SequenceTracker {
            #[inline]
            fn clone(&self) -> SequenceTracker {
                {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for SequenceTracker {}
        impl borsh::de::BorshDeserialize for SequenceTracker
        where
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    sequence: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for SequenceTracker
        where
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.sequence, writer)?;
                Ok(())
            }
        }
        pub struct SequenceDerivationData<'a> {
            pub emitter_key: &'a Pubkey,
        }
        impl<'b> Seeded<&SequenceDerivationData<'b>> for Sequence<'b> {
            fn seeds(data: &SequenceDerivationData) -> Vec<Vec<u8>> {
                <[_]>::into_vec(box [
                    "Sequence".as_bytes().to_vec(),
                    data.emitter_key.to_bytes().to_vec(),
                ])
            }
        }
        impl Owned for SequenceTracker {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
    }
    pub mod signature_set {
        //! PostedMessage
        use borsh::{BorshDeserialize, BorshSerialize};
        use solitaire::{AccountOwner, AccountState, Data, Owned};
        pub type SignatureSet<'b, const State: AccountState> =
            Data<'b, SignatureSetData, { State }>;
        pub struct SignatureSetData {
            /// Signatures of validators
            pub signatures: Vec<bool>,
            /// Hash of the data
            pub hash: [u8; 32],
            /// Index of the guardian set
            pub guardian_set_index: u32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SignatureSetData {
            #[inline]
            fn default() -> SignatureSetData {
                SignatureSetData {
                    signatures: ::core::default::Default::default(),
                    hash: ::core::default::Default::default(),
                    guardian_set_index: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for SignatureSetData
        where
            Vec<bool>: borsh::ser::BorshSerialize,
            [u8; 32]: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.signatures, writer)?;
                borsh::BorshSerialize::serialize(&self.hash, writer)?;
                borsh::BorshSerialize::serialize(&self.guardian_set_index, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for SignatureSetData
        where
            Vec<bool>: borsh::BorshDeserialize,
            [u8; 32]: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    signatures: borsh::BorshDeserialize::deserialize(buf)?,
                    hash: borsh::BorshDeserialize::deserialize(buf)?,
                    guardian_set_index: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl Owned for SignatureSetData {
            fn owner(&self) -> AccountOwner {
                AccountOwner::This
            }
        }
    }
    pub use bridge::*;
    pub use claim::*;
    pub use fee_collector::*;
    pub use guardian_set::*;
    pub use posted_message::*;
    pub use posted_vaa::*;
    pub use sequence::*;
    pub use signature_set::*;
}
pub use accounts::{
    BridgeConfig, BridgeData, Claim, ClaimData, ClaimDerivationData, FeeCollector, GuardianSet,
    GuardianSetData, GuardianSetDerivationData, PostedMessage, PostedMessageData, MessageData,
    PostedVAA, PostedVAAData, Sequence, SequenceTracker, SequenceDerivationData, SignatureSet,
    SignatureSetData,
};
pub mod api {
    pub mod governance {
        use solitaire::*;
        use solana_program::{
            program::invoke_signed,
            pubkey::Pubkey,
            sysvar::{clock::Clock, rent::Rent},
        };
        use solitaire::{processors::seeded::Seeded, CreationLamports::Exempt};
        use crate::{
            accounts::{Bridge, GuardianSet, GuardianSetDerivationData},
            error::Error::{
                InvalidFeeRecipient, InvalidGovernanceKey, InvalidGovernanceWithdrawal,
                InvalidGuardianSetUpgrade,
            },
            types::{
                GovernancePayloadGuardianSetChange, GovernancePayloadSetMessageFee,
                GovernancePayloadTransferFees, GovernancePayloadUpgrade,
            },
            vaa::ClaimableVAA,
            DeserializePayload, CHAIN_ID_SOLANA,
        };
        fn verify_governance<'a, T>(vaa: &ClaimableVAA<'a, T>) -> Result<()>
        where
            T: DeserializePayload,
        {
            let expected_emitter = "test";
            let current_emitter = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&Pubkey::new_from_array(vaa.message.meta().emitter_address),) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            };
            if expected_emitter != current_emitter
                || vaa.message.meta().emitter_chain != CHAIN_ID_SOLANA
            {
                Err(InvalidGovernanceKey.into())
            } else {
                Ok(())
            }
        }
        pub struct UpgradeContract<'b> {
            /// Payer for account creation (vaa-claim)
            pub payer: Mut<Signer<Info<'b>>>,
            /// Bridge config
            pub bridge: Mut<Bridge<'b, { AccountState::Initialized }>>,
            /// GuardianSet change VAA
            pub vaa: ClaimableVAA<'b, GovernancePayloadUpgrade>,
            /// PDA authority for the loader
            pub upgrade_authority: Derive<Info<'b>, "upgrade">,
            /// Spill address for the upgrade excess lamports
            pub spill: Mut<Info<'b>>,
            /// New contract address.
            pub buffer: Mut<Info<'b>>,
            /// Required by the upgradeable uploader.
            pub program_data: Mut<Info<'b>>,
            /// Our own address, required by the upgradeable loader.
            pub own_address: Mut<Info<'b>>,
            pub rent: Sysvar<'b, Rent>,
            pub clock: Sysvar<'b, Clock>,
            pub bpf_loader: Info<'b>,
            pub system: Info<'b>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for UpgradeContract<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bridge: Mut<Bridge<'b, { AccountState::Initialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let vaa: ClaimableVAA<'b, GovernancePayloadUpgrade> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let upgrade_authority: Derive<Info<'b>, "upgrade"> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let spill: Mut<Info<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let buffer: Mut<Info<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let program_data: Mut<Info<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let own_address: Mut<Info<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let rent: Sysvar<'b, Rent> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let clock: Sysvar<'b, Clock> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bpf_loader: Info<'b> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let system: Info<'b> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(UpgradeContract {
                    payer,
                    bridge,
                    vaa,
                    upgrade_authority,
                    spill,
                    buffer,
                    program_data,
                    own_address,
                    rent,
                    clock,
                    bpf_loader,
                    system,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for UpgradeContract<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: UpgradeContract<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Mut<Bridge<'b, { AccountState::Initialized }>> as Peel>::deps());
                deps.append(&mut <ClaimableVAA<'b, GovernancePayloadUpgrade> as Peel>::deps());
                deps.append(&mut <Derive<Info<'b>, "upgrade"> as Peel>::deps());
                deps.append(&mut <Mut<Info<'b>> as Peel>::deps());
                deps.append(&mut <Mut<Info<'b>> as Peel>::deps());
                deps.append(&mut <Mut<Info<'b>> as Peel>::deps());
                deps.append(&mut <Mut<Info<'b>> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Rent> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Clock> as Peel>::deps());
                deps.append(&mut <Info<'b> as Peel>::deps());
                deps.append(&mut <Info<'b> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for UpgradeContract<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.vaa, program_id)?;
                Peel::persist(&self.upgrade_authority, program_id)?;
                Peel::persist(&self.spill, program_id)?;
                Peel::persist(&self.buffer, program_id)?;
                Peel::persist(&self.program_data, program_id)?;
                Peel::persist(&self.own_address, program_id)?;
                Peel::persist(&self.rent, program_id)?;
                Peel::persist(&self.clock, program_id)?;
                Peel::persist(&self.bpf_loader, program_id)?;
                Peel::persist(&self.system, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for UpgradeContract<'b> {}
        pub struct UpgradeContractData {}
        impl borsh::de::BorshDeserialize for UpgradeContractData {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {})
            }
        }
        impl borsh::ser::BorshSerialize for UpgradeContractData {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for UpgradeContractData {
            #[inline]
            fn default() -> UpgradeContractData {
                UpgradeContractData {}
            }
        }
        pub fn upgrade_contract(
            ctx: &ExecutionContext,
            accs: &mut UpgradeContract,
            _data: UpgradeContractData,
        ) -> Result<()> {
            verify_governance(&accs.vaa)?;
            accs.vaa.verify(ctx.program_id)?;
            accs.vaa.claim(ctx, accs.payer.key)?;
            let upgrade_ix = solana_program::bpf_loader_upgradeable::upgrade(
                ctx.program_id,
                &accs.vaa.message.new_contract,
                accs.upgrade_authority.key,
                accs.spill.key,
            );
            let seeds = accs
                .upgrade_authority
                .self_bumped_seeds(None, ctx.program_id);
            let seeds: Vec<&[u8]> = seeds.iter().map(|item| item.as_slice()).collect();
            let seeds = seeds.as_slice();
            invoke_signed(&upgrade_ix, ctx.accounts, &[seeds])?;
            Ok(())
        }
        pub struct UpgradeGuardianSet<'b> {
            /// Payer for account creation (vaa-claim)
            pub payer: Mut<Signer<Info<'b>>>,
            /// Bridge config
            pub bridge: Mut<Bridge<'b, { AccountState::Initialized }>>,
            /// GuardianSet change VAA
            pub vaa: ClaimableVAA<'b, GovernancePayloadGuardianSetChange>,
            /// Old guardian set
            pub guardian_set_old: Mut<GuardianSet<'b, { AccountState::Initialized }>>,
            /// New guardian set
            pub guardian_set_new: Mut<GuardianSet<'b, { AccountState::Uninitialized }>>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for UpgradeGuardianSet<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bridge: Mut<Bridge<'b, { AccountState::Initialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let vaa: ClaimableVAA<'b, GovernancePayloadGuardianSetChange> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let guardian_set_old: Mut<GuardianSet<'b, { AccountState::Initialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let guardian_set_new: Mut<GuardianSet<'b, { AccountState::Uninitialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(UpgradeGuardianSet {
                    payer,
                    bridge,
                    vaa,
                    guardian_set_old,
                    guardian_set_new,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for UpgradeGuardianSet<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: UpgradeGuardianSet<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Mut<Bridge<'b, { AccountState::Initialized }>> as Peel>::deps());
                deps.append(
                    &mut <ClaimableVAA<'b, GovernancePayloadGuardianSetChange> as Peel>::deps(),
                );
                deps.append(
                    &mut <Mut<GuardianSet<'b, { AccountState::Initialized }>> as Peel>::deps(),
                );
                deps.append(
                    &mut <Mut<GuardianSet<'b, { AccountState::Uninitialized }>> as Peel>::deps(),
                );
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for UpgradeGuardianSet<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.vaa, program_id)?;
                Peel::persist(&self.guardian_set_old, program_id)?;
                Peel::persist(&self.guardian_set_new, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for UpgradeGuardianSet<'b> {}
        pub struct UpgradeGuardianSetData {}
        impl borsh::de::BorshDeserialize for UpgradeGuardianSetData {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {})
            }
        }
        impl borsh::ser::BorshSerialize for UpgradeGuardianSetData {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for UpgradeGuardianSetData {
            #[inline]
            fn default() -> UpgradeGuardianSetData {
                UpgradeGuardianSetData {}
            }
        }
        pub fn upgrade_guardian_set(
            ctx: &ExecutionContext,
            accs: &mut UpgradeGuardianSet,
            _data: UpgradeGuardianSetData,
        ) -> Result<()> {
            if accs.guardian_set_old.index != accs.vaa.new_guardian_set_index - 1 {
                return Err(InvalidGuardianSetUpgrade.into());
            }
            if accs.bridge.guardian_set_index != accs.vaa.new_guardian_set_index - 1 {
                return Err(InvalidGuardianSetUpgrade.into());
            }
            verify_governance(&accs.vaa)?;
            accs.vaa.verify(ctx.program_id)?;
            accs.guardian_set_old.verify_derivation(
                ctx.program_id,
                &GuardianSetDerivationData {
                    index: accs.vaa.new_guardian_set_index - 1,
                },
            )?;
            accs.guardian_set_new.verify_derivation(
                ctx.program_id,
                &GuardianSetDerivationData {
                    index: accs.vaa.new_guardian_set_index,
                },
            )?;
            accs.vaa.claim(ctx, accs.payer.key)?;
            accs.guardian_set_old.expiration_time =
                accs.vaa.meta().vaa_time + accs.bridge.config.guardian_set_expiration_time;
            accs.guardian_set_new.index = accs.vaa.new_guardian_set_index;
            accs.guardian_set_new.creation_time = accs.vaa.meta().vaa_time;
            accs.guardian_set_new.keys = accs.vaa.new_guardian_set.clone();
            accs.guardian_set_new.create(
                &GuardianSetDerivationData {
                    index: accs.guardian_set_new.index,
                },
                ctx,
                accs.payer.key,
                Exempt,
            )?;
            accs.bridge.guardian_set_index = accs.vaa.new_guardian_set_index;
            Ok(())
        }
        pub struct SetFees<'b> {
            /// Payer for account creation (vaa-claim)
            pub payer: Mut<Signer<Info<'b>>>,
            /// Bridge config
            pub bridge: Mut<Bridge<'b, { AccountState::Initialized }>>,
            /// Governance VAA
            pub vaa: ClaimableVAA<'b, GovernancePayloadSetMessageFee>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for SetFees<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bridge: Mut<Bridge<'b, { AccountState::Initialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let vaa: ClaimableVAA<'b, GovernancePayloadSetMessageFee> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(SetFees { payer, bridge, vaa })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for SetFees<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: SetFees<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Mut<Bridge<'b, { AccountState::Initialized }>> as Peel>::deps());
                deps.append(
                    &mut <ClaimableVAA<'b, GovernancePayloadSetMessageFee> as Peel>::deps(),
                );
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for SetFees<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.vaa, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for SetFees<'b> {}
        pub struct SetFeesData {}
        impl borsh::de::BorshDeserialize for SetFeesData {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {})
            }
        }
        impl borsh::ser::BorshSerialize for SetFeesData {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SetFeesData {
            #[inline]
            fn default() -> SetFeesData {
                SetFeesData {}
            }
        }
        pub fn set_fees(
            ctx: &ExecutionContext,
            accs: &mut SetFees,
            _data: SetFeesData,
        ) -> Result<()> {
            verify_governance(&accs.vaa)?;
            accs.vaa.verify(ctx.program_id)?;
            accs.vaa.claim(ctx, accs.payer.key)?;
            accs.bridge.config.fee = accs.vaa.fee.as_u64();
            Ok(())
        }
        pub struct TransferFees<'b> {
            /// Payer for account creation (vaa-claim)
            pub payer: Mut<Signer<Info<'b>>>,
            /// Bridge config
            pub bridge: Bridge<'b, { AccountState::Initialized }>,
            /// Governance VAA
            pub vaa: ClaimableVAA<'b, GovernancePayloadTransferFees>,
            /// Account collecting tx fees
            pub fee_collector: Mut<Derive<Info<'b>, "fee_collector">>,
            /// Fee recipient
            pub recipient: Mut<Info<'b>>,
            /// Rent calculator to check transfer sizes.
            pub rent: Sysvar<'b, Rent>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for TransferFees<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bridge: Bridge<'b, { AccountState::Initialized }> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let vaa: ClaimableVAA<'b, GovernancePayloadTransferFees> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let fee_collector: Mut<Derive<Info<'b>, "fee_collector">> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let recipient: Mut<Info<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let rent: Sysvar<'b, Rent> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(TransferFees {
                    payer,
                    bridge,
                    vaa,
                    fee_collector,
                    recipient,
                    rent,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for TransferFees<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: TransferFees<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Bridge<'b, { AccountState::Initialized }> as Peel>::deps());
                deps.append(&mut <ClaimableVAA<'b, GovernancePayloadTransferFees> as Peel>::deps());
                deps.append(&mut <Mut<Derive<Info<'b>, "fee_collector">> as Peel>::deps());
                deps.append(&mut <Mut<Info<'b>> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Rent> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for TransferFees<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.vaa, program_id)?;
                Peel::persist(&self.fee_collector, program_id)?;
                Peel::persist(&self.recipient, program_id)?;
                Peel::persist(&self.rent, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for TransferFees<'b> {}
        pub struct TransferFeesData {}
        impl borsh::de::BorshDeserialize for TransferFeesData {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {})
            }
        }
        impl borsh::ser::BorshSerialize for TransferFeesData {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for TransferFeesData {
            #[inline]
            fn default() -> TransferFeesData {
                TransferFeesData {}
            }
        }
        pub fn transfer_fees(
            ctx: &ExecutionContext,
            accs: &mut TransferFees,
            _data: TransferFeesData,
        ) -> Result<()> {
            if accs.vaa.to != accs.recipient.key.to_bytes() {
                return Err(InvalidFeeRecipient.into());
            }
            verify_governance(&accs.vaa)?;
            accs.vaa.verify(ctx.program_id)?;
            if accs
                .fee_collector
                .lamports()
                .saturating_sub(accs.vaa.amount.as_u64())
                < accs.rent.minimum_balance(accs.fee_collector.data_len())
            {
                return Err(InvalidGovernanceWithdrawal.into());
            }
            accs.vaa.claim(ctx, accs.payer.key)?;
            let transfer_ix = solana_program::system_instruction::transfer(
                accs.fee_collector.key,
                accs.recipient.key,
                accs.vaa.amount.as_u64(),
            );
            let seeds = accs.fee_collector.self_bumped_seeds(None, ctx.program_id);
            let seeds: Vec<&[u8]> = seeds.iter().map(|item| item.as_slice()).collect();
            let seeds = seeds.as_slice();
            invoke_signed(&transfer_ix, ctx.accounts, &[seeds])?;
            Ok(())
        }
    }
    pub mod initialize {
        use crate::{
            accounts::{
                Bridge, BridgeConfig, FeeCollector, GuardianSet, GuardianSetDerivationData,
            },
            error::Error::TooManyGuardians,
            MAX_LEN_GUARDIAN_KEYS,
        };
        use solana_program::sysvar::clock::Clock;
        use solitaire::{CreationLamports::Exempt, *};
        type Payer<'a> = Signer<Info<'a>>;
        pub struct Initialize<'b> {
            /// Bridge config.
            pub bridge: Mut<Bridge<'b, { AccountState::Uninitialized }>>,
            /// Location the new guardian set will be allocated at.
            pub guardian_set: Mut<GuardianSet<'b, { AccountState::Uninitialized }>>,
            /// Location of the fee collector that users will need to pay.
            pub fee_collector: Mut<FeeCollector<'b>>,
            /// Payer for account creation.
            pub payer: Mut<Payer<'b>>,
            /// Clock used for recording the initialization time.
            pub clock: Sysvar<'b, Clock>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for Initialize<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let bridge: Mut<Bridge<'b, { AccountState::Uninitialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let guardian_set: Mut<GuardianSet<'b, { AccountState::Uninitialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let fee_collector: Mut<FeeCollector<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let payer: Mut<Payer<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let clock: Sysvar<'b, Clock> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(Initialize {
                    bridge,
                    guardian_set,
                    fee_collector,
                    payer,
                    clock,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for Initialize<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: Initialize<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(
                    &mut <Mut<Bridge<'b, { AccountState::Uninitialized }>> as Peel>::deps(),
                );
                deps.append(
                    &mut <Mut<GuardianSet<'b, { AccountState::Uninitialized }>> as Peel>::deps(),
                );
                deps.append(&mut <Mut<FeeCollector<'b>> as Peel>::deps());
                deps.append(&mut <Mut<Payer<'b>> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Clock> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for Initialize<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.guardian_set, program_id)?;
                Peel::persist(&self.fee_collector, program_id)?;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.clock, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for Initialize<'b> {}
        pub struct InitializeData {
            /// Period for how long a guardian set is valid after it has been replaced by a new one.  This
            /// guarantees that VAAs issued by that set can still be submitted for a certain period.  In
            /// this period we still trust the old guardian set.
            pub guardian_set_expiration_time: u32,
            /// Amount of lamports that needs to be paid to the protocol to post a message
            pub fee: u64,
            /// Initial Guardian Set
            pub initial_guardians: Vec<[u8; 20]>,
        }
        impl borsh::de::BorshDeserialize for InitializeData
        where
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Vec<[u8; 20]>: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    guardian_set_expiration_time: borsh::BorshDeserialize::deserialize(buf)?,
                    fee: borsh::BorshDeserialize::deserialize(buf)?,
                    initial_guardians: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for InitializeData
        where
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            Vec<[u8; 20]>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.guardian_set_expiration_time, writer)?;
                borsh::BorshSerialize::serialize(&self.fee, writer)?;
                borsh::BorshSerialize::serialize(&self.initial_guardians, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for InitializeData {
            #[inline]
            fn default() -> InitializeData {
                InitializeData {
                    guardian_set_expiration_time: ::core::default::Default::default(),
                    fee: ::core::default::Default::default(),
                    initial_guardians: ::core::default::Default::default(),
                }
            }
        }
        pub fn initialize(
            ctx: &ExecutionContext,
            accs: &mut Initialize,
            data: InitializeData,
        ) -> Result<()> {
            let index = 0;
            if data.initial_guardians.len() > MAX_LEN_GUARDIAN_KEYS {
                return Err(TooManyGuardians.into());
            }
            accs.guardian_set.index = index;
            accs.guardian_set.creation_time = accs.clock.unix_timestamp as u32;
            accs.guardian_set.keys.extend(&data.initial_guardians);
            accs.guardian_set.create(
                &GuardianSetDerivationData { index },
                ctx,
                accs.payer.key,
                Exempt,
            )?;
            accs.bridge.create(ctx, accs.payer.key, Exempt)?;
            accs.bridge.guardian_set_index = index;
            accs.bridge.config = BridgeConfig {
                guardian_set_expiration_time: data.guardian_set_expiration_time,
                fee: data.fee,
            };
            accs.fee_collector.create(
                ctx,
                accs.payer.key,
                Exempt,
                0,
                &solana_program::system_program::id(),
            )?;
            accs.bridge.last_lamports = accs.fee_collector.lamports();
            Ok(())
        }
    }
    pub mod post_message {
        use crate::{
            accounts::{Bridge, FeeCollector, PostedMessage, Sequence, SequenceDerivationData},
            error::Error::{InsufficientFees, MathOverflow},
            types::ConsistencyLevel,
            CHAIN_ID_SOLANA,
        };
        use solana_program::{msg, sysvar::clock::Clock};
        use solitaire::{processors::seeded::Seeded, trace, CreationLamports::Exempt, *};
        pub type UninitializedMessage<'b> = PostedMessage<'b, { AccountState::Uninitialized }>;
        impl<'a> From<&PostMessage<'a>> for SequenceDerivationData<'a> {
            fn from(accs: &PostMessage<'a>) -> Self {
                SequenceDerivationData {
                    emitter_key: accs.emitter.key,
                }
            }
        }
        pub struct PostMessage<'b> {
            /// Bridge config needed for fee calculation.
            pub bridge: Mut<Bridge<'b, { AccountState::Initialized }>>,
            /// Account to store the posted message
            pub message: Signer<Mut<UninitializedMessage<'b>>>,
            /// Emitter of the VAA
            pub emitter: Signer<MaybeMut<Info<'b>>>,
            /// Tracker for the emitter sequence
            pub sequence: Mut<Sequence<'b>>,
            /// Payer for account creation
            pub payer: Mut<Signer<Info<'b>>>,
            /// Account to collect tx fee
            pub fee_collector: Mut<FeeCollector<'b>>,
            pub clock: Sysvar<'b, Clock>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for PostMessage<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let bridge: Mut<Bridge<'b, { AccountState::Initialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let message: Signer<Mut<UninitializedMessage<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let emitter: Signer<MaybeMut<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let sequence: Mut<Sequence<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let fee_collector: Mut<FeeCollector<'b>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let clock: Sysvar<'b, Clock> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(PostMessage {
                    bridge,
                    message,
                    emitter,
                    sequence,
                    payer,
                    fee_collector,
                    clock,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for PostMessage<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: PostMessage<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Bridge<'b, { AccountState::Initialized }>> as Peel>::deps());
                deps.append(&mut <Signer<Mut<UninitializedMessage<'b>>> as Peel>::deps());
                deps.append(&mut <Signer<MaybeMut<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Mut<Sequence<'b>> as Peel>::deps());
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Mut<FeeCollector<'b>> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Clock> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for PostMessage<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.bridge, program_id)?;
                Peel::persist(&self.message, program_id)?;
                Peel::persist(&self.emitter, program_id)?;
                Peel::persist(&self.sequence, program_id)?;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.fee_collector, program_id)?;
                Peel::persist(&self.clock, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for PostMessage<'b> {}
        pub struct PostMessageData {
            /// Unique nonce for this message
            pub nonce: u32,
            /// Message payload
            pub payload: Vec<u8>,
            /// Commitment Level required for an attestation to be produced
            pub consistency_level: ConsistencyLevel,
        }
        impl borsh::de::BorshDeserialize for PostMessageData
        where
            u32: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
            ConsistencyLevel: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    nonce: borsh::BorshDeserialize::deserialize(buf)?,
                    payload: borsh::BorshDeserialize::deserialize(buf)?,
                    consistency_level: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for PostMessageData
        where
            u32: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
            ConsistencyLevel: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.nonce, writer)?;
                borsh::BorshSerialize::serialize(&self.payload, writer)?;
                borsh::BorshSerialize::serialize(&self.consistency_level, writer)?;
                Ok(())
            }
        }
        pub fn post_message(
            ctx: &ExecutionContext,
            accs: &mut PostMessage,
            data: PostMessageData,
        ) -> Result<()> {
            accs.sequence
                .verify_derivation(ctx.program_id, &(&*accs).into())?;
            let fee = accs.bridge.config.fee;
            if accs
                .fee_collector
                .lamports()
                .checked_sub(accs.bridge.last_lamports)
                .ok_or(MathOverflow)?
                < fee
            {
                return Err(InsufficientFees.into());
            }
            accs.bridge.last_lamports = accs.fee_collector.lamports();
            if !accs.sequence.is_initialized() {
                accs.sequence
                    .create(&(&*accs).into(), ctx, accs.payer.key, Exempt)?;
            }
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Sequence: "],
                    &match (&accs.sequence.sequence,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            });
            accs.message.submission_time = accs.clock.unix_timestamp as u32;
            accs.message.emitter_chain = CHAIN_ID_SOLANA;
            accs.message.emitter_address = accs.emitter.key.to_bytes();
            accs.message.nonce = data.nonce;
            accs.message.payload = data.payload;
            accs.message.sequence = accs.sequence.sequence;
            accs.message.consistency_level = match data.consistency_level {
                ConsistencyLevel::Confirmed => 1,
                ConsistencyLevel::Finalized => 32,
            };
            let size = accs.message.size();
            let ix = solana_program::system_instruction::create_account(
                accs.payer.key,
                accs.message.info().key,
                Exempt.amount(size),
                size as u64,
                ctx.program_id,
            );
            solana_program::program::invoke(&ix, ctx.accounts)?;
            accs.sequence.sequence += 1;
            Ok(())
        }
    }
    pub mod post_vaa {
        use solitaire::*;
        use borsh::{BorshDeserialize, BorshSerialize};
        use solana_program::{self, sysvar::clock::Clock};
        use crate::{
            accounts::{
                Bridge, GuardianSet, GuardianSetDerivationData, PostedVAA, PostedVAADerivationData,
                SignatureSet,
            },
            error::Error::{
                GuardianSetMismatch, PostVAAConsensusFailed, PostVAAGuardianSetExpired,
            },
        };
        use byteorder::{BigEndian, WriteBytesExt};
        use serde::{Deserialize, Serialize};
        use sha3::Digest;
        use solana_program::program_error::ProgramError;
        use solitaire::{processors::seeded::Seeded, CreationLamports::Exempt};
        use std::io::{Cursor, Write};
        impl From<&PostVAAData> for GuardianSetDerivationData {
            fn from(data: &PostVAAData) -> Self {
                GuardianSetDerivationData {
                    index: data.guardian_set_index,
                }
            }
        }
        pub struct PostVAA<'b> {
            /// Information about the current guardian set.
            pub guardian_set: GuardianSet<'b, { AccountState::Initialized }>,
            /// Bridge Info
            pub bridge_info: Bridge<'b, { AccountState::Initialized }>,
            /// Signature Info
            pub signature_set: SignatureSet<'b, { AccountState::Initialized }>,
            /// Message the VAA is associated with.
            pub message: Mut<PostedVAA<'b, { AccountState::MaybeInitialized }>>,
            /// Account used to pay for auxillary instructions.
            pub payer: Mut<Signer<Info<'b>>>,
            /// Clock used for timestamping.
            pub clock: Sysvar<'b, Clock>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for PostVAA<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let guardian_set: GuardianSet<'b, { AccountState::Initialized }> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let bridge_info: Bridge<'b, { AccountState::Initialized }> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let signature_set: SignatureSet<'b, { AccountState::Initialized }> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let message: Mut<PostedVAA<'b, { AccountState::MaybeInitialized }>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let clock: Sysvar<'b, Clock> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(PostVAA {
                    guardian_set,
                    bridge_info,
                    signature_set,
                    message,
                    payer,
                    clock,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for PostVAA<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: PostVAA<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <GuardianSet<'b, { AccountState::Initialized }> as Peel>::deps());
                deps.append(&mut <Bridge<'b, { AccountState::Initialized }> as Peel>::deps());
                deps.append(&mut <SignatureSet<'b, { AccountState::Initialized }> as Peel>::deps());
                deps.append(
                    &mut <Mut<PostedVAA<'b, { AccountState::MaybeInitialized }>> as Peel>::deps(),
                );
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <Sysvar<'b, Clock> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for PostVAA<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.guardian_set, program_id)?;
                Peel::persist(&self.bridge_info, program_id)?;
                Peel::persist(&self.signature_set, program_id)?;
                Peel::persist(&self.message, program_id)?;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.clock, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for PostVAA<'b> {}
        pub struct Signature {
            pub index: u8,
            pub r: [u8; 32],
            pub s: [u8; 32],
            pub v: u8,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for Signature {
            #[inline]
            fn default() -> Signature {
                Signature {
                    index: ::core::default::Default::default(),
                    r: ::core::default::Default::default(),
                    s: ::core::default::Default::default(),
                    v: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for Signature
        where
            u8: borsh::ser::BorshSerialize,
            [u8; 32]: borsh::ser::BorshSerialize,
            [u8; 32]: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.index, writer)?;
                borsh::BorshSerialize::serialize(&self.r, writer)?;
                borsh::BorshSerialize::serialize(&self.s, writer)?;
                borsh::BorshSerialize::serialize(&self.v, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Signature
        where
            u8: borsh::BorshDeserialize,
            [u8; 32]: borsh::BorshDeserialize,
            [u8; 32]: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    index: borsh::BorshDeserialize::deserialize(buf)?,
                    r: borsh::BorshDeserialize::deserialize(buf)?,
                    s: borsh::BorshDeserialize::deserialize(buf)?,
                    v: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        pub type ForeignAddress = [u8; 32];
        pub struct PostVAAData {
            pub version: u8,
            pub guardian_set_index: u32,
            pub timestamp: u32,
            pub nonce: u32,
            pub emitter_chain: u16,
            pub emitter_address: ForeignAddress,
            pub sequence: u64,
            pub consistency_level: u8,
            pub payload: Vec<u8>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for PostVAAData {
            #[inline]
            fn default() -> PostVAAData {
                PostVAAData {
                    version: ::core::default::Default::default(),
                    guardian_set_index: ::core::default::Default::default(),
                    timestamp: ::core::default::Default::default(),
                    nonce: ::core::default::Default::default(),
                    emitter_chain: ::core::default::Default::default(),
                    emitter_address: ::core::default::Default::default(),
                    sequence: ::core::default::Default::default(),
                    consistency_level: ::core::default::Default::default(),
                    payload: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for PostVAAData
        where
            u8: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            ForeignAddress: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.version, writer)?;
                borsh::BorshSerialize::serialize(&self.guardian_set_index, writer)?;
                borsh::BorshSerialize::serialize(&self.timestamp, writer)?;
                borsh::BorshSerialize::serialize(&self.nonce, writer)?;
                borsh::BorshSerialize::serialize(&self.emitter_chain, writer)?;
                borsh::BorshSerialize::serialize(&self.emitter_address, writer)?;
                borsh::BorshSerialize::serialize(&self.sequence, writer)?;
                borsh::BorshSerialize::serialize(&self.consistency_level, writer)?;
                borsh::BorshSerialize::serialize(&self.payload, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for PostVAAData
        where
            u8: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            ForeignAddress: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    version: borsh::BorshDeserialize::deserialize(buf)?,
                    guardian_set_index: borsh::BorshDeserialize::deserialize(buf)?,
                    timestamp: borsh::BorshDeserialize::deserialize(buf)?,
                    nonce: borsh::BorshDeserialize::deserialize(buf)?,
                    emitter_chain: borsh::BorshDeserialize::deserialize(buf)?,
                    emitter_address: borsh::BorshDeserialize::deserialize(buf)?,
                    sequence: borsh::BorshDeserialize::deserialize(buf)?,
                    consistency_level: borsh::BorshDeserialize::deserialize(buf)?,
                    payload: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for PostVAAData {
            #[inline]
            fn clone(&self) -> PostVAAData {
                match *self {
                    PostVAAData {
                        version: ref __self_0_0,
                        guardian_set_index: ref __self_0_1,
                        timestamp: ref __self_0_2,
                        nonce: ref __self_0_3,
                        emitter_chain: ref __self_0_4,
                        emitter_address: ref __self_0_5,
                        sequence: ref __self_0_6,
                        consistency_level: ref __self_0_7,
                        payload: ref __self_0_8,
                    } => PostVAAData {
                        version: ::core::clone::Clone::clone(&(*__self_0_0)),
                        guardian_set_index: ::core::clone::Clone::clone(&(*__self_0_1)),
                        timestamp: ::core::clone::Clone::clone(&(*__self_0_2)),
                        nonce: ::core::clone::Clone::clone(&(*__self_0_3)),
                        emitter_chain: ::core::clone::Clone::clone(&(*__self_0_4)),
                        emitter_address: ::core::clone::Clone::clone(&(*__self_0_5)),
                        sequence: ::core::clone::Clone::clone(&(*__self_0_6)),
                        consistency_level: ::core::clone::Clone::clone(&(*__self_0_7)),
                        payload: ::core::clone::Clone::clone(&(*__self_0_8)),
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PostVAAData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "PostVAAData",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "version",
                        &self.version,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guardian_set_index",
                        &self.guardian_set_index,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timestamp",
                        &self.timestamp,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "nonce",
                        &self.nonce,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "emitter_chain",
                        &self.emitter_chain,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "emitter_address",
                        &self.emitter_address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sequence",
                        &self.sequence,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "consistency_level",
                        &self.consistency_level,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "payload",
                        &self.payload,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PostVAAData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "version" => _serde::__private::Ok(__Field::__field0),
                                "guardian_set_index" => _serde::__private::Ok(__Field::__field1),
                                "timestamp" => _serde::__private::Ok(__Field::__field2),
                                "nonce" => _serde::__private::Ok(__Field::__field3),
                                "emitter_chain" => _serde::__private::Ok(__Field::__field4),
                                "emitter_address" => _serde::__private::Ok(__Field::__field5),
                                "sequence" => _serde::__private::Ok(__Field::__field6),
                                "consistency_level" => _serde::__private::Ok(__Field::__field7),
                                "payload" => _serde::__private::Ok(__Field::__field8),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"version" => _serde::__private::Ok(__Field::__field0),
                                b"guardian_set_index" => _serde::__private::Ok(__Field::__field1),
                                b"timestamp" => _serde::__private::Ok(__Field::__field2),
                                b"nonce" => _serde::__private::Ok(__Field::__field3),
                                b"emitter_chain" => _serde::__private::Ok(__Field::__field4),
                                b"emitter_address" => _serde::__private::Ok(__Field::__field5),
                                b"sequence" => _serde::__private::Ok(__Field::__field6),
                                b"consistency_level" => _serde::__private::Ok(__Field::__field7),
                                b"payload" => _serde::__private::Ok(__Field::__field8),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PostVAAData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PostVAAData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct PostVAAData",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct PostVAAData with 9 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<u16>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                ForeignAddress,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<u64>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 =
                                match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                7usize,
                                                &"struct PostVAAData with 9 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<Vec<u8>>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct PostVAAData with 9 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(PostVAAData {
                                version: __field0,
                                guardian_set_index: __field1,
                                timestamp: __field2,
                                nonce: __field3,
                                emitter_chain: __field4,
                                emitter_address: __field5,
                                sequence: __field6,
                                consistency_level: __field7,
                                payload: __field8,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u8> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<u16> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<ForeignAddress> =
                                _serde::__private::None;
                            let mut __field6: _serde::__private::Option<u64> =
                                _serde::__private::None;
                            let mut __field7: _serde::__private::Option<u8> =
                                _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Vec<u8>> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "version",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u8>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "guardian_set_index",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timestamp",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "nonce",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "emitter_chain",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u16>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "emitter_address",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<ForeignAddress>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sequence",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u64>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "consistency_level",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u8>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "payload",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Vec<u8>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("version") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guardian_set_index")
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("timestamp") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("nonce") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("emitter_chain") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("emitter_address") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("sequence") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("consistency_level")
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("payload") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(PostVAAData {
                                version: __field0,
                                guardian_set_index: __field1,
                                timestamp: __field2,
                                nonce: __field3,
                                emitter_chain: __field4,
                                emitter_address: __field5,
                                sequence: __field6,
                                consistency_level: __field7,
                                payload: __field8,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "version",
                        "guardian_set_index",
                        "timestamp",
                        "nonce",
                        "emitter_chain",
                        "emitter_address",
                        "sequence",
                        "consistency_level",
                        "payload",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PostVAAData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PostVAAData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub fn post_vaa(
            ctx: &ExecutionContext,
            accs: &mut PostVAA,
            vaa: PostVAAData,
        ) -> Result<()> {
            let msg_derivation = PostedVAADerivationData {
                payload_hash: accs.signature_set.hash.to_vec(),
            };
            accs.message
                .verify_derivation(ctx.program_id, &msg_derivation)?;
            accs.guardian_set
                .verify_derivation(ctx.program_id, &(&vaa).into())?;
            if accs.message.is_initialized() {
                return Ok(());
            }
            check_active(&accs.guardian_set, &accs.clock)?;
            check_valid_sigs(&accs.guardian_set, &accs.signature_set)?;
            check_integrity(&vaa, &accs.signature_set)?;
            let signature_count: usize =
                accs.signature_set.signatures.iter().filter(|v| **v).count();
            let required_consensus_count = {
                let len = accs.guardian_set.keys.len();
                let len = (len * 10) / 3;
                let len = len * 2;
                len / 10 + 1
            };
            if signature_count < required_consensus_count {
                return Err(PostVAAConsensusFailed.into());
            }
            accs.message.nonce = vaa.nonce;
            accs.message.emitter_chain = vaa.emitter_chain;
            accs.message.emitter_address = vaa.emitter_address;
            accs.message.sequence = vaa.sequence;
            accs.message.payload = vaa.payload;
            accs.message.consistency_level = vaa.consistency_level;
            accs.message.vaa_version = vaa.version;
            accs.message.vaa_time = vaa.timestamp;
            accs.message.vaa_signature_account = *accs.signature_set.info().key;
            accs.message
                .create(&msg_derivation, ctx, accs.payer.key, Exempt)?;
            Ok(())
        }
        /// A guardian set must not have expired.
        #[inline(always)]
        fn check_active<'r>(
            guardian_set: &GuardianSet<'r, { AccountState::Initialized }>,
            clock: &Sysvar<'r, Clock>,
        ) -> Result<()> {
            if guardian_set.index == 0 && guardian_set.creation_time == 1628099186 {
                return Err(PostVAAGuardianSetExpired.into());
            }
            if guardian_set.expiration_time != 0
                && (guardian_set.expiration_time as i64) < clock.unix_timestamp
            {
                return Err(PostVAAGuardianSetExpired.into());
            }
            Ok(())
        }
        /// The signatures in this instruction must be from the right guardian set.
        #[inline(always)]
        fn check_valid_sigs<'r>(
            guardian_set: &GuardianSet<'r, { AccountState::Initialized }>,
            signatures: &SignatureSet<'r, { AccountState::Initialized }>,
        ) -> Result<()> {
            if signatures.guardian_set_index != guardian_set.index {
                return Err(GuardianSetMismatch.into());
            }
            Ok(())
        }
        #[inline(always)]
        fn check_integrity<'r>(
            vaa: &PostVAAData,
            signatures: &SignatureSet<'r, { AccountState::Initialized }>,
        ) -> Result<()> {
            let body = {
                let mut v = Cursor::new(Vec::new());
                v.write_u32::<BigEndian>(vaa.timestamp)?;
                v.write_u32::<BigEndian>(vaa.nonce)?;
                v.write_u16::<BigEndian>(vaa.emitter_chain)?;
                v.write(&vaa.emitter_address)?;
                v.write_u64::<BigEndian>(vaa.sequence)?;
                v.write_u8(vaa.consistency_level)?;
                v.write(&vaa.payload)?;
                v.into_inner()
            };
            let body_hash: [u8; 32] = {
                let mut h = sha3::Keccak256::default();
                h.write(body.as_slice())
                    .map_err(|_| ProgramError::InvalidArgument)?;
                h.finalize().into()
            };
            if signatures.hash != body_hash {
                return Err(ProgramError::InvalidAccountData.into());
            }
            Ok(())
        }
    }
    pub mod verify_signature {
        use solitaire::*;
        use crate::{
            GuardianSet, GuardianSetDerivationData, SignatureSet,
            error::Error::{
                GuardianSetMismatch, InstructionAtWrongIndex, InvalidHash, InvalidSecpInstruction,
            },
            MAX_LEN_GUARDIAN_KEYS,
        };
        use byteorder::ByteOrder;
        use solana_program::program_error::ProgramError;
        use solitaire::{processors::seeded::Seeded, CreationLamports::Exempt};
        pub struct VerifySignatures<'b> {
            /// Payer for account creation
            pub payer: Mut<Signer<Info<'b>>>,
            /// Guardian set of the signatures
            pub guardian_set: GuardianSet<'b, { AccountState::Initialized }>,
            /// Signature Account
            pub signature_set: Mut<Signer<SignatureSet<'b, { AccountState::MaybeInitialized }>>>,
            /// Instruction reflection account (special sysvar)
            pub instruction_acc: Info<'b>,
        }
        /// Macro generated implementation of FromAccounts by Solitaire.
        impl<'a, 'b: 'a, 'c> solitaire::FromAccounts<'a, 'b, 'c> for VerifySignatures<'b> {
            fn from<DataType>(
                pid: &'a solana_program::pubkey::Pubkey,
                iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
                data: &'a DataType,
            ) -> solitaire::Result<Self> {
                use solana_program::account_info::next_account_info;
                use solitaire::trace;
                let payer: Mut<Signer<Info<'b>>> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let guardian_set: GuardianSet<'b, { AccountState::Initialized }> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let signature_set: Mut<
                    Signer<SignatureSet<'b, { AccountState::MaybeInitialized }>>,
                > = solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                let instruction_acc: Info<'b> =
                    solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
                Ok(VerifySignatures {
                    payer,
                    guardian_set,
                    signature_set,
                    instruction_acc,
                })
            }
        }
        impl<'a, 'b: 'a, 'c> solitaire::Peel<'a, 'b, 'c> for VerifySignatures<'b> {
            fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
            where
                Self: Sized,
            {
                let v: VerifySignatures<'b> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
                Ok(v)
            }
            fn deps() -> Vec<solana_program::pubkey::Pubkey> {
                let mut deps = Vec::new();
                deps.append(&mut <Mut<Signer<Info<'b>>> as Peel>::deps());
                deps.append(&mut <GuardianSet<'b, { AccountState::Initialized }> as Peel>::deps());
                deps.append(&mut <Mut<
                    Signer<SignatureSet<'b, { AccountState::MaybeInitialized }>>,
                > as Peel>::deps());
                deps.append(&mut <Info<'b> as Peel>::deps());
                deps
            }
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                solitaire::Persist::persist(self, program_id)
            }
        }
        /// Macro generated implementation of Persist by Solitaire.
        impl<'b> solitaire::Persist for VerifySignatures<'b> {
            fn persist(
                &self,
                program_id: &solana_program::pubkey::Pubkey,
            ) -> solitaire::Result<()> {
                use solitaire::trace;
                Peel::persist(&self.payer, program_id)?;
                Peel::persist(&self.guardian_set, program_id)?;
                Peel::persist(&self.signature_set, program_id)?;
                Peel::persist(&self.instruction_acc, program_id)?;
                Ok(())
            }
        }
        impl<'b> InstructionContext<'b> for VerifySignatures<'b> {}
        impl From<&VerifySignatures<'_>> for GuardianSetDerivationData {
            fn from(data: &VerifySignatures<'_>) -> Self {
                GuardianSetDerivationData {
                    index: data.guardian_set.index,
                }
            }
        }
        pub struct VerifySignaturesData {
            /// instruction indices of signers (-1 for missing)
            pub signers: [i8; MAX_LEN_GUARDIAN_KEYS],
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for VerifySignaturesData {
            #[inline]
            fn default() -> VerifySignaturesData {
                VerifySignaturesData {
                    signers: ::core::default::Default::default(),
                }
            }
        }
        impl borsh::ser::BorshSerialize for VerifySignaturesData
        where
            [i8; MAX_LEN_GUARDIAN_KEYS]: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.signers, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for VerifySignaturesData
        where
            [i8; MAX_LEN_GUARDIAN_KEYS]: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    signers: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        /// SigInfo contains metadata about signers in a VerifySignature ix
        struct SigInfo {
            /// index of the signer in the guardianset
            signer_index: u8,
            /// index of the signature in the secp instruction
            sig_index: u8,
        }
        struct SecpInstructionPart<'a> {
            address: &'a [u8],
            msg_offset: u16,
            msg_size: u16,
        }
        pub fn verify_signatures(
            ctx: &ExecutionContext,
            accs: &mut VerifySignatures,
            data: VerifySignaturesData,
        ) -> Result<()> {
            accs.guardian_set
                .verify_derivation(ctx.program_id, &(&*accs).into())?;
            let sig_infos: Vec<SigInfo> = data
                .signers
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    if *p == -1 {
                        return None;
                    }
                    return Some(SigInfo {
                        sig_index: *p as u8,
                        signer_index: i as u8,
                    });
                })
                .collect();
            let current_instruction = solana_program::sysvar::instructions::load_current_index(
                &accs.instruction_acc.try_borrow_mut_data()?,
            );
            if current_instruction == 0 {
                return Err(InstructionAtWrongIndex.into());
            }
            let secp_ix_index = (current_instruction - 1) as u8;
            let secp_ix = solana_program::sysvar::instructions::load_instruction_at(
                secp_ix_index as usize,
                &accs.instruction_acc.try_borrow_mut_data()?,
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
            if secp_ix.program_id != solana_program::secp256k1_program::id() {
                return Err(InvalidSecpInstruction.into());
            }
            let secp_data_len = secp_ix.data.len();
            if secp_data_len < 2 {
                return Err(InvalidSecpInstruction.into());
            }
            let sig_len = secp_ix.data[0];
            let mut index = 1;
            let mut secp_ixs: Vec<SecpInstructionPart> = Vec::with_capacity(sig_len as usize);
            for i in 0..sig_len {
                let _sig_offset = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]) as usize;
                index += 2;
                let sig_ix = secp_ix.data[index];
                index += 1;
                let address_offset =
                    byteorder::LE::read_u16(&secp_ix.data[index..index + 2]) as usize;
                index += 2;
                let address_ix = secp_ix.data[index];
                index += 1;
                let msg_offset = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]);
                index += 2;
                let msg_size = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]);
                index += 2;
                let msg_ix = secp_ix.data[index];
                index += 1;
                if address_ix != secp_ix_index || msg_ix != secp_ix_index || sig_ix != secp_ix_index
                {
                    return Err(InvalidSecpInstruction.into());
                }
                let address: &[u8] = &secp_ix.data[address_offset..address_offset + 20];
                if i > 0 {
                    if msg_offset != secp_ixs[0].msg_offset || msg_size != secp_ixs[0].msg_size {
                        return Err(InvalidSecpInstruction.into());
                    }
                }
                secp_ixs.push(SecpInstructionPart {
                    address,
                    msg_offset,
                    msg_size,
                });
            }
            if sig_infos.len() != secp_ixs.len() {
                return Err(ProgramError::InvalidArgument.into());
            }
            if secp_ixs[0].msg_size != 32 {
                return Err(ProgramError::InvalidArgument.into());
            }
            let message = &secp_ix.data[secp_ixs[0].msg_offset as usize
                ..(secp_ixs[0].msg_offset + secp_ixs[0].msg_size) as usize];
            let mut msg_hash: [u8; 32] = [0u8; 32];
            msg_hash.copy_from_slice(message);
            if !accs.signature_set.is_initialized() {
                accs.signature_set.signatures =
                    ::alloc::vec::from_elem(false, accs.guardian_set.keys.len());
                accs.signature_set.guardian_set_index = accs.guardian_set.index;
                accs.signature_set.hash = msg_hash;
                let size = accs.signature_set.size();
                let ix = solana_program::system_instruction::create_account(
                    accs.payer.key,
                    accs.signature_set.info().key,
                    Exempt.amount(size),
                    size as u64,
                    ctx.program_id,
                );
                solana_program::program::invoke(&ix, ctx.accounts)?;
            } else {
                if accs.signature_set.guardian_set_index != accs.guardian_set.index {
                    return Err(GuardianSetMismatch.into());
                }
                if accs.signature_set.hash != msg_hash {
                    return Err(InvalidHash.into());
                }
            }
            for s in sig_infos {
                if s.signer_index > accs.guardian_set.num_guardians() {
                    return Err(ProgramError::InvalidArgument.into());
                }
                if s.sig_index + 1 > sig_len {
                    return Err(ProgramError::InvalidArgument.into());
                }
                let key = accs.guardian_set.keys[s.signer_index as usize];
                if key != secp_ixs[s.sig_index as usize].address {
                    return Err(ProgramError::InvalidArgument.into());
                }
                accs.signature_set.signatures[s.signer_index as usize] = true;
            }
            Ok(())
        }
    }
    pub use governance::*;
    pub use initialize::*;
    pub use post_message::*;
    pub use post_vaa::*;
    pub use verify_signature::*;
}
pub use api::{
    initialize, post_message, post_vaa, set_fees, transfer_fees, upgrade_contract,
    upgrade_guardian_set, verify_signatures, Initialize, InitializeData, PostMessage,
    PostMessageData, PostVAA, PostVAAData, SetFees, SetFeesData, Signature, TransferFees,
    TransferFeesData, UninitializedMessage, UpgradeContract, UpgradeContractData,
    UpgradeGuardianSet, UpgradeGuardianSetData, VerifySignatures, VerifySignaturesData,
};
pub mod error {
    //! Define application level errors that can be returned by the various instruction handlers that
    //! make up the wormhole bridge.
    use crate::trace;
    use solitaire::SolitaireError;
    pub enum Error {
        GuardianSetMismatch,
        InstructionAtWrongIndex,
        InsufficientFees,
        InvalidFeeRecipient,
        InvalidGovernanceAction,
        InvalidGovernanceChain,
        InvalidGovernanceKey,
        InvalidGovernanceModule,
        InvalidGovernanceWithdrawal,
        InvalidGuardianSetUpgrade,
        InvalidHash,
        InvalidSecpInstruction,
        MathOverflow,
        PostVAAConsensusFailed,
        PostVAAGuardianSetExpired,
        TooManyGuardians,
        VAAAlreadyExecuted,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Error::GuardianSetMismatch,) => {
                    ::core::fmt::Formatter::write_str(f, "GuardianSetMismatch")
                }
                (&Error::InstructionAtWrongIndex,) => {
                    ::core::fmt::Formatter::write_str(f, "InstructionAtWrongIndex")
                }
                (&Error::InsufficientFees,) => {
                    ::core::fmt::Formatter::write_str(f, "InsufficientFees")
                }
                (&Error::InvalidFeeRecipient,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidFeeRecipient")
                }
                (&Error::InvalidGovernanceAction,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGovernanceAction")
                }
                (&Error::InvalidGovernanceChain,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGovernanceChain")
                }
                (&Error::InvalidGovernanceKey,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGovernanceKey")
                }
                (&Error::InvalidGovernanceModule,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGovernanceModule")
                }
                (&Error::InvalidGovernanceWithdrawal,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGovernanceWithdrawal")
                }
                (&Error::InvalidGuardianSetUpgrade,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGuardianSetUpgrade")
                }
                (&Error::InvalidHash,) => ::core::fmt::Formatter::write_str(f, "InvalidHash"),
                (&Error::InvalidSecpInstruction,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidSecpInstruction")
                }
                (&Error::MathOverflow,) => ::core::fmt::Formatter::write_str(f, "MathOverflow"),
                (&Error::PostVAAConsensusFailed,) => {
                    ::core::fmt::Formatter::write_str(f, "PostVAAConsensusFailed")
                }
                (&Error::PostVAAGuardianSetExpired,) => {
                    ::core::fmt::Formatter::write_str(f, "PostVAAGuardianSetExpired")
                }
                (&Error::TooManyGuardians,) => {
                    ::core::fmt::Formatter::write_str(f, "TooManyGuardians")
                }
                (&Error::VAAAlreadyExecuted,) => {
                    ::core::fmt::Formatter::write_str(f, "VAAAlreadyExecuted")
                }
            }
        }
    }
    /// Errors thrown by the program will bubble up to the solitaire wrapper, which needs a way to
    /// translate these errors into something Solitaire can log and handle.
    impl From<Error> for SolitaireError {
        fn from(e: Error) -> SolitaireError {
            SolitaireError::Custom(e as u64)
        }
    }
}
pub mod types {
    use crate::{
        api::ForeignAddress,
        vaa::{
            DeserializeGovernancePayload, DeserializePayload, SerializeGovernancePayload,
            SerializePayload,
        },
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use byteorder::{BigEndian, ReadBytesExt};
    use primitive_types::U256;
    use serde::{Deserialize, Serialize};
    use solana_program::{program_error::ProgramError::InvalidAccountData, pubkey::Pubkey};
    use solitaire::SolitaireError;
    use std::{
        self,
        io::{Cursor, Read, Write},
    };
    /// Type representing an Ethereum style public key for Guardians.
    pub type GuardianPublicKey = [u8; 20];
    #[repr(u8)]
    pub enum ConsistencyLevel {
        Confirmed,
        Finalized,
    }
    impl borsh::ser::BorshSerialize for ConsistencyLevel {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> core::result::Result<(), borsh::maybestd::io::Error> {
            match self {
                ConsistencyLevel::Confirmed => {
                    let variant_idx: u8 = 0u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                ConsistencyLevel::Finalized => {
                    let variant_idx: u8 = 1u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ConsistencyLevel {
        fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
            let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
            let return_value = match variant_idx {
                0u8 => ConsistencyLevel::Confirmed,
                1u8 => ConsistencyLevel::Finalized,
                _ => {
                    let msg = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unexpected variant index: "],
                            &match (&variant_idx,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Debug::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    return Err(borsh::maybestd::io::Error::new(
                        borsh::maybestd::io::ErrorKind::InvalidInput,
                        msg,
                    ));
                }
            };
            Ok(return_value)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ConsistencyLevel {
        #[inline]
        fn clone(&self) -> ConsistencyLevel {
            match (&*self,) {
                (&ConsistencyLevel::Confirmed,) => ConsistencyLevel::Confirmed,
                (&ConsistencyLevel::Finalized,) => ConsistencyLevel::Finalized,
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ConsistencyLevel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ConsistencyLevel::Confirmed => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ConsistencyLevel",
                        0u32,
                        "Confirmed",
                    ),
                    ConsistencyLevel::Finalized => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ConsistencyLevel",
                        1u32,
                        "Finalized",
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ConsistencyLevel {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Confirmed" => _serde::__private::Ok(__Field::__field0),
                            "Finalized" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Confirmed" => _serde::__private::Ok(__Field::__field0),
                            b"Finalized" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ConsistencyLevel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ConsistencyLevel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ConsistencyLevel",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ConsistencyLevel::Confirmed)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ConsistencyLevel::Finalized)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Confirmed", "Finalized"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ConsistencyLevel",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ConsistencyLevel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct GovernancePayloadUpgrade {
        pub new_contract: Pubkey,
    }
    impl SerializePayload for GovernancePayloadUpgrade {
        fn serialize<W: Write>(&self, v: &mut W) -> std::result::Result<(), SolitaireError> {
            v.write(&self.new_contract.to_bytes())?;
            Ok(())
        }
    }
    impl DeserializePayload for GovernancePayloadUpgrade
    where
        Self: DeserializeGovernancePayload,
    {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, SolitaireError> {
            let mut c = Cursor::new(buf);
            Self::check_governance_header(&mut c)?;
            let mut addr = [0u8; 32];
            c.read_exact(&mut addr)?;
            if c.position() != c.into_inner().len() as u64 {
                return Err(InvalidAccountData.into());
            }
            Ok(GovernancePayloadUpgrade {
                new_contract: Pubkey::new(&addr[..]),
            })
        }
    }
    impl SerializeGovernancePayload for GovernancePayloadUpgrade {
        const MODULE: &'static str = "Core";
        const ACTION: u8 = 1;
    }
    impl DeserializeGovernancePayload for GovernancePayloadUpgrade {}
    pub struct GovernancePayloadGuardianSetChange {
        pub new_guardian_set_index: u32,
        pub new_guardian_set: Vec<[u8; 20]>,
    }
    impl SerializePayload for GovernancePayloadGuardianSetChange {
        fn serialize<W: Write>(&self, v: &mut W) -> std::result::Result<(), SolitaireError> {
            use byteorder::WriteBytesExt;
            v.write_u32::<BigEndian>(self.new_guardian_set_index)?;
            v.write_u8(self.new_guardian_set.len() as u8)?;
            for key in self.new_guardian_set.iter() {
                v.write(key)?;
            }
            Ok(())
        }
    }
    impl DeserializePayload for GovernancePayloadGuardianSetChange
    where
        Self: DeserializeGovernancePayload,
    {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, SolitaireError> {
            let mut c = Cursor::new(buf);
            Self::check_governance_header(&mut c)?;
            let new_index = c.read_u32::<BigEndian>()?;
            let keys_len = c.read_u8()?;
            let mut keys = Vec::with_capacity(keys_len as usize);
            for _ in 0..keys_len {
                let mut key: [u8; 20] = [0; 20];
                c.read(&mut key)?;
                keys.push(key);
            }
            if c.position() != c.into_inner().len() as u64 {
                return Err(InvalidAccountData.into());
            }
            Ok(GovernancePayloadGuardianSetChange {
                new_guardian_set_index: new_index,
                new_guardian_set: keys,
            })
        }
    }
    impl SerializeGovernancePayload for GovernancePayloadGuardianSetChange {
        const MODULE: &'static str = "Core";
        const ACTION: u8 = 2;
    }
    impl DeserializeGovernancePayload for GovernancePayloadGuardianSetChange {}
    pub struct GovernancePayloadSetMessageFee {
        pub fee: U256,
    }
    impl SerializePayload for GovernancePayloadSetMessageFee {
        fn serialize<W: Write>(&self, v: &mut W) -> std::result::Result<(), SolitaireError> {
            let mut fee_data = [0u8; 32];
            self.fee.to_big_endian(&mut fee_data);
            v.write(&fee_data[..])?;
            Ok(())
        }
    }
    impl DeserializePayload for GovernancePayloadSetMessageFee
    where
        Self: DeserializeGovernancePayload,
    {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, SolitaireError> {
            let mut c = Cursor::new(buf);
            Self::check_governance_header(&mut c)?;
            let mut fee_data: [u8; 32] = [0; 32];
            c.read_exact(&mut fee_data)?;
            let fee = U256::from_big_endian(&fee_data);
            if c.position() != c.into_inner().len() as u64 {
                return Err(InvalidAccountData.into());
            }
            Ok(GovernancePayloadSetMessageFee { fee })
        }
    }
    impl SerializeGovernancePayload for GovernancePayloadSetMessageFee {
        const MODULE: &'static str = "Core";
        const ACTION: u8 = 3;
    }
    impl DeserializeGovernancePayload for GovernancePayloadSetMessageFee {}
    pub struct GovernancePayloadTransferFees {
        pub amount: U256,
        pub to: ForeignAddress,
    }
    impl SerializePayload for GovernancePayloadTransferFees {
        fn serialize<W: Write>(&self, v: &mut W) -> std::result::Result<(), SolitaireError> {
            let mut amount_data = [0u8; 32];
            self.amount.to_big_endian(&mut amount_data);
            v.write(&amount_data)?;
            v.write(&self.to)?;
            Ok(())
        }
    }
    impl DeserializePayload for GovernancePayloadTransferFees
    where
        Self: DeserializeGovernancePayload,
    {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, SolitaireError> {
            let mut c = Cursor::new(buf);
            Self::check_governance_header(&mut c)?;
            let mut amount_data: [u8; 32] = [0; 32];
            c.read_exact(&mut amount_data)?;
            let amount = U256::from_big_endian(&amount_data);
            let mut to = ForeignAddress::default();
            c.read_exact(&mut to)?;
            if c.position() != c.into_inner().len() as u64 {
                return Err(InvalidAccountData.into());
            }
            Ok(GovernancePayloadTransferFees { amount, to })
        }
    }
    impl SerializeGovernancePayload for GovernancePayloadTransferFees {
        const MODULE: &'static str = "Core";
        const ACTION: u8 = 4;
    }
    impl DeserializeGovernancePayload for GovernancePayloadTransferFees {}
}
pub mod vaa {
    use crate::{
        api::{post_vaa::PostVAAData, ForeignAddress},
        error::Error::{
            InvalidGovernanceAction, InvalidGovernanceChain, InvalidGovernanceModule,
            VAAAlreadyExecuted,
        },
        Claim, ClaimDerivationData, PostedVAAData, Result, CHAIN_ID_SOLANA,
    };
    use byteorder::{BigEndian, ReadBytesExt};
    use serde::{Deserialize, Serialize};
    use solana_program::pubkey::Pubkey;
    use solitaire::{
        processors::seeded::Seeded, trace, Context, CreationLamports::Exempt, Data,
        ExecutionContext, Peel, SolitaireError, *,
    };
    use std::{
        io::{Cursor, Read, Write},
        ops::Deref,
    };
    pub trait SerializePayload: Sized {
        fn serialize<W: Write>(&self, writer: &mut W) -> std::result::Result<(), SolitaireError>;
        fn try_to_vec(&self) -> std::result::Result<Vec<u8>, SolitaireError> {
            let mut result = Vec::with_capacity(256);
            self.serialize(&mut result)?;
            Ok(result)
        }
    }
    pub trait DeserializePayload: Sized {
        fn deserialize(buf: &mut &[u8]) -> std::result::Result<Self, SolitaireError>;
    }
    pub trait SerializeGovernancePayload: SerializePayload {
        const MODULE: &'static str;
        const ACTION: u8;
        fn try_to_vec(&self) -> std::result::Result<Vec<u8>, SolitaireError> {
            let mut result = Vec::with_capacity(256);
            self.write_governance_header(&mut result)?;
            self.serialize(&mut result)?;
            Ok(result)
        }
        fn write_governance_header<W: Write>(
            &self,
            c: &mut W,
        ) -> std::result::Result<(), SolitaireError> {
            use byteorder::WriteBytesExt;
            let module = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &match (&Self::MODULE,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '\u{0}',
                            align: ::core::fmt::rt::v1::Alignment::Right,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(32usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
                res
            };
            let module = module.as_bytes();
            c.write(&module)?;
            c.write_u8(Self::ACTION)?;
            c.write_u16::<BigEndian>(CHAIN_ID_SOLANA)?;
            Ok(())
        }
    }
    pub trait DeserializeGovernancePayload:
        DeserializePayload + SerializeGovernancePayload
    {
        fn check_governance_header(
            c: &mut Cursor<&mut &[u8]>,
        ) -> std::result::Result<(), SolitaireError> {
            let mut module = [0u8; 32];
            c.read_exact(&mut module)?;
            if module != {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &match (&Self::MODULE,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '\u{0}',
                            align: ::core::fmt::rt::v1::Alignment::Right,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(32usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
                res
            }
            .as_bytes()
            {
                return Err(InvalidGovernanceModule.into());
            }
            let action = c.read_u8()?;
            if action != Self::ACTION {
                return Err(InvalidGovernanceAction.into());
            }
            let chain = c.read_u16::<BigEndian>()?;
            if chain != CHAIN_ID_SOLANA && chain != 0 {
                return Err(InvalidGovernanceChain.into());
            }
            Ok(())
        }
    }
    pub struct PayloadMessage<'b, T: DeserializePayload>(
        Data<'b, PostedVAAData, { AccountState::Initialized }>,
        T,
    );
    impl<'a, 'b: 'a, 'c, T: DeserializePayload> Peel<'a, 'b, 'c> for PayloadMessage<'b, T> {
        fn peel<I>(ctx: &'c mut Context<'a, 'b, 'c, I>) -> Result<Self>
        where
            Self: Sized,
        {
            let data: Data<'b, PostedVAAData, { AccountState::Initialized }> = Data::peel(ctx)?;
            let payload = DeserializePayload::deserialize(&mut &data.payload[..])?;
            Ok(PayloadMessage(data, payload))
        }
        fn deps() -> Vec<Pubkey> {
            Data::<'b, PostedVAAData, { AccountState::Initialized }>::deps()
        }
        fn persist(&self, program_id: &Pubkey) -> Result<()> {
            Data::persist(&self.0, program_id)
        }
    }
    impl<'b, T: DeserializePayload> Deref for PayloadMessage<'b, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.1
        }
    }
    impl<'b, T: DeserializePayload> PayloadMessage<'b, T> {
        pub fn meta(&self) -> &PostedVAAData {
            &self.0
        }
    }
    pub struct ClaimableVAA<'b, T: DeserializePayload> {
        pub message: PayloadMessage<'b, T>,
        pub claim: Mut<Claim<'b, { AccountState::Uninitialized }>>,
    }
    /// Macro generated implementation of FromAccounts by Solitaire.
    impl<'a, 'b: 'a, 'c, T: DeserializePayload> solitaire::FromAccounts<'a, 'b, 'c>
        for ClaimableVAA<'b, T>
    {
        fn from<DataType>(
            pid: &'a solana_program::pubkey::Pubkey,
            iter: &'c mut std::slice::Iter<'a, solana_program::account_info::AccountInfo<'b>>,
            data: &'a DataType,
        ) -> solitaire::Result<Self> {
            use solana_program::account_info::next_account_info;
            use solitaire::trace;
            let message: PayloadMessage<'b, T> =
                solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
            let claim: Mut<Claim<'b, { AccountState::Uninitialized }>> =
                solitaire::Peel::peel(&mut solitaire::Context::new(pid, iter, data))?;
            Ok(ClaimableVAA { message, claim })
        }
    }
    impl<'a, 'b: 'a, 'c, T: DeserializePayload> solitaire::Peel<'a, 'b, 'c> for ClaimableVAA<'b, T> {
        fn peel<I>(ctx: &'c mut solitaire::Context<'a, 'b, 'c, I>) -> solitaire::Result<Self>
        where
            Self: Sized,
        {
            let v: ClaimableVAA<'b, T> = FromAccounts::from(ctx.this, ctx.iter, ctx.data)?;
            Ok(v)
        }
        fn deps() -> Vec<solana_program::pubkey::Pubkey> {
            let mut deps = Vec::new();
            deps.append(&mut <PayloadMessage<'b, T> as Peel>::deps());
            deps.append(&mut <Mut<Claim<'b, { AccountState::Uninitialized }>> as Peel>::deps());
            deps
        }
        fn persist(&self, program_id: &solana_program::pubkey::Pubkey) -> solitaire::Result<()> {
            solitaire::Persist::persist(self, program_id)
        }
    }
    /// Macro generated implementation of Persist by Solitaire.
    impl<'b, T: DeserializePayload> solitaire::Persist for ClaimableVAA<'b, T> {
        fn persist(&self, program_id: &solana_program::pubkey::Pubkey) -> solitaire::Result<()> {
            use solitaire::trace;
            Peel::persist(&self.message, program_id)?;
            Peel::persist(&self.claim, program_id)?;
            Ok(())
        }
    }
    impl<'b, T: DeserializePayload> Deref for ClaimableVAA<'b, T> {
        type Target = PayloadMessage<'b, T>;
        fn deref(&self) -> &Self::Target {
            &self.message
        }
    }
    impl<'b, T: DeserializePayload> ClaimableVAA<'b, T> {
        pub fn verify(&self, program_id: &Pubkey) -> Result<()> {
            self.claim.verify_derivation(
                program_id,
                &ClaimDerivationData {
                    emitter_address: self.message.meta().emitter_address,
                    emitter_chain: self.message.meta().emitter_chain,
                    sequence: self.message.meta().sequence,
                },
            )?;
            Ok(())
        }
    }
    impl<'b, T: DeserializePayload> ClaimableVAA<'b, T> {
        pub fn is_claimed(&self) -> bool {
            self.claim.claimed
        }
        pub fn claim(&mut self, ctx: &ExecutionContext, payer: &Pubkey) -> Result<()> {
            if self.is_claimed() {
                return Err(VAAAlreadyExecuted.into());
            }
            self.claim.create(
                &ClaimDerivationData {
                    emitter_address: self.message.meta().emitter_address,
                    emitter_chain: self.message.meta().emitter_chain,
                    sequence: self.message.meta().sequence,
                },
                ctx,
                payer,
                Exempt,
            )?;
            self.claim.claimed = true;
            Ok(())
        }
    }
    pub struct SignatureItem {
        pub signature: Vec<u8>,
        pub key: [u8; 20],
        pub index: u8,
    }
    pub struct VAASignature {
        pub signature: Vec<u8>,
        pub guardian_index: u8,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for VAASignature {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "VAASignature",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "signature",
                    &self.signature,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "guardian_index",
                    &self.guardian_index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VAASignature {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "signature" => _serde::__private::Ok(__Field::__field0),
                            "guardian_index" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"signature" => _serde::__private::Ok(__Field::__field0),
                            b"guardian_index" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<VAASignature>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VAASignature;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct VAASignature")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<u8>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct VAASignature with 2 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct VAASignature with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(VAASignature {
                            signature: __field0,
                            guardian_index: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<u8>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u8> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "signature",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<u8>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "guardian_index",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("signature") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("guardian_index") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(VAASignature {
                            signature: __field0,
                            guardian_index: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["signature", "guardian_index"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "VAASignature",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<VAASignature>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for VAASignature {
        #[inline]
        fn default() -> VAASignature {
            VAASignature {
                signature: ::core::default::Default::default(),
                guardian_index: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for VAASignature {
        #[inline]
        fn clone(&self) -> VAASignature {
            match *self {
                VAASignature {
                    signature: ref __self_0_0,
                    guardian_index: ref __self_0_1,
                } => VAASignature {
                    signature: ::core::clone::Clone::clone(&(*__self_0_0)),
                    guardian_index: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    pub struct VAA {
        pub version: u8,
        pub guardian_set_index: u32,
        pub signatures: Vec<VAASignature>,
        pub timestamp: u32,
        pub nonce: u32,
        pub emitter_chain: u16,
        pub emitter_address: ForeignAddress,
        pub sequence: u64,
        pub consistency_level: u8,
        pub payload: Vec<u8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for VAA {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "VAA",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "guardian_set_index",
                    &self.guardian_set_index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "signatures",
                    &self.signatures,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timestamp",
                    &self.timestamp,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "nonce",
                    &self.nonce,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "emitter_chain",
                    &self.emitter_chain,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "emitter_address",
                    &self.emitter_address,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "sequence",
                    &self.sequence,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "consistency_level",
                    &self.consistency_level,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "payload",
                    &self.payload,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VAA {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "version" => _serde::__private::Ok(__Field::__field0),
                            "guardian_set_index" => _serde::__private::Ok(__Field::__field1),
                            "signatures" => _serde::__private::Ok(__Field::__field2),
                            "timestamp" => _serde::__private::Ok(__Field::__field3),
                            "nonce" => _serde::__private::Ok(__Field::__field4),
                            "emitter_chain" => _serde::__private::Ok(__Field::__field5),
                            "emitter_address" => _serde::__private::Ok(__Field::__field6),
                            "sequence" => _serde::__private::Ok(__Field::__field7),
                            "consistency_level" => _serde::__private::Ok(__Field::__field8),
                            "payload" => _serde::__private::Ok(__Field::__field9),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"version" => _serde::__private::Ok(__Field::__field0),
                            b"guardian_set_index" => _serde::__private::Ok(__Field::__field1),
                            b"signatures" => _serde::__private::Ok(__Field::__field2),
                            b"timestamp" => _serde::__private::Ok(__Field::__field3),
                            b"nonce" => _serde::__private::Ok(__Field::__field4),
                            b"emitter_chain" => _serde::__private::Ok(__Field::__field5),
                            b"emitter_address" => _serde::__private::Ok(__Field::__field6),
                            b"sequence" => _serde::__private::Ok(__Field::__field7),
                            b"consistency_level" => _serde::__private::Ok(__Field::__field8),
                            b"payload" => _serde::__private::Ok(__Field::__field9),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<VAA>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VAA;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct VAA")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Vec<VAASignature>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct VAA with 10 elements",
                                ));
                            }
                        };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<u16>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            ForeignAddress,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct VAA with 10 elements",
                                ));
                            }
                        };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct VAA with 10 elements",
                                        ),
                                    );
                                }
                            };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<Vec<u8>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct VAA with 10 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(VAA {
                            version: __field0,
                            guardian_set_index: __field1,
                            signatures: __field2,
                            timestamp: __field3,
                            nonce: __field4,
                            emitter_chain: __field5,
                            emitter_address: __field6,
                            sequence: __field7,
                            consistency_level: __field8,
                            payload: __field9,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<VAASignature>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<u16> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<ForeignAddress> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<Vec<u8>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "guardian_set_index",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "signatures",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<VAASignature>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timestamp",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "nonce",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "emitter_chain",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u16>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "emitter_address",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ForeignAddress>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sequence",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "consistency_level",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<u8>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("version") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("guardian_set_index") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("signatures") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("timestamp") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("nonce") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("emitter_chain") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("emitter_address") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("sequence") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("consistency_level") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("payload") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(VAA {
                            version: __field0,
                            guardian_set_index: __field1,
                            signatures: __field2,
                            timestamp: __field3,
                            nonce: __field4,
                            emitter_chain: __field5,
                            emitter_address: __field6,
                            sequence: __field7,
                            consistency_level: __field8,
                            payload: __field9,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "version",
                    "guardian_set_index",
                    "signatures",
                    "timestamp",
                    "nonce",
                    "emitter_chain",
                    "emitter_address",
                    "sequence",
                    "consistency_level",
                    "payload",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "VAA",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<VAA>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for VAA {
        #[inline]
        fn default() -> VAA {
            VAA {
                version: ::core::default::Default::default(),
                guardian_set_index: ::core::default::Default::default(),
                signatures: ::core::default::Default::default(),
                timestamp: ::core::default::Default::default(),
                nonce: ::core::default::Default::default(),
                emitter_chain: ::core::default::Default::default(),
                emitter_address: ::core::default::Default::default(),
                sequence: ::core::default::Default::default(),
                consistency_level: ::core::default::Default::default(),
                payload: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for VAA {
        #[inline]
        fn clone(&self) -> VAA {
            match *self {
                VAA {
                    version: ref __self_0_0,
                    guardian_set_index: ref __self_0_1,
                    signatures: ref __self_0_2,
                    timestamp: ref __self_0_3,
                    nonce: ref __self_0_4,
                    emitter_chain: ref __self_0_5,
                    emitter_address: ref __self_0_6,
                    sequence: ref __self_0_7,
                    consistency_level: ref __self_0_8,
                    payload: ref __self_0_9,
                } => VAA {
                    version: ::core::clone::Clone::clone(&(*__self_0_0)),
                    guardian_set_index: ::core::clone::Clone::clone(&(*__self_0_1)),
                    signatures: ::core::clone::Clone::clone(&(*__self_0_2)),
                    timestamp: ::core::clone::Clone::clone(&(*__self_0_3)),
                    nonce: ::core::clone::Clone::clone(&(*__self_0_4)),
                    emitter_chain: ::core::clone::Clone::clone(&(*__self_0_5)),
                    emitter_address: ::core::clone::Clone::clone(&(*__self_0_6)),
                    sequence: ::core::clone::Clone::clone(&(*__self_0_7)),
                    consistency_level: ::core::clone::Clone::clone(&(*__self_0_8)),
                    payload: ::core::clone::Clone::clone(&(*__self_0_9)),
                },
            }
        }
    }
    impl VAA {
        pub const HEADER_LEN: usize = 6;
        pub const SIGNATURE_LEN: usize = 66;
        pub fn deserialize(data: &[u8]) -> std::result::Result<VAA, std::io::Error> {
            let mut rdr = Cursor::new(data);
            let mut v = VAA::default();
            v.version = rdr.read_u8()?;
            v.guardian_set_index = rdr.read_u32::<BigEndian>()?;
            let len_sig = rdr.read_u8()?;
            let mut sigs: Vec<VAASignature> = Vec::with_capacity(len_sig as usize);
            for _i in 0..len_sig {
                let mut sig = VAASignature::default();
                sig.guardian_index = rdr.read_u8()?;
                let mut signature_data = [0u8; 65];
                rdr.read_exact(&mut signature_data)?;
                sig.signature = signature_data.to_vec();
                sigs.push(sig);
            }
            v.signatures = sigs;
            v.timestamp = rdr.read_u32::<BigEndian>()?;
            v.nonce = rdr.read_u32::<BigEndian>()?;
            v.emitter_chain = rdr.read_u16::<BigEndian>()?;
            let mut emitter_address = [0u8; 32];
            rdr.read_exact(&mut emitter_address)?;
            v.emitter_address = emitter_address;
            v.sequence = rdr.read_u64::<BigEndian>()?;
            v.consistency_level = rdr.read_u8()?;
            rdr.read_to_end(&mut v.payload)?;
            Ok(v)
        }
    }
    impl From<VAA> for PostVAAData {
        fn from(vaa: VAA) -> Self {
            PostVAAData {
                version: vaa.version,
                guardian_set_index: vaa.guardian_set_index,
                timestamp: vaa.timestamp,
                nonce: vaa.nonce,
                emitter_chain: vaa.emitter_chain,
                emitter_address: vaa.emitter_address,
                sequence: vaa.sequence,
                consistency_level: vaa.consistency_level,
                payload: vaa.payload,
            }
        }
    }
}
pub use vaa::{
    DeserializeGovernancePayload, DeserializePayload, PayloadMessage, SerializeGovernancePayload,
    SerializePayload,
};
pub mod instruction {
    use super::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
        pubkey::Pubkey,
    };
    use solitaire::{trace, ExecutionContext, FromAccounts, Persist, Result, SolitaireError};
    #[allow(non_snake_case)]
    pub mod Initialize {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: InitializeData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            initialize(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod PostMessage {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: PostMessageData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            post_message(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod PostVAA {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: PostVAAData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            post_vaa(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod SetFees {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: SetFeesData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            set_fees(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod TransferFees {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: TransferFeesData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            transfer_fees(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod UpgradeContract {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: UpgradeContractData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            upgrade_contract(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod UpgradeGuardianSet {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: UpgradeGuardianSetData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            upgrade_guardian_set(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    #[allow(non_snake_case)]
    pub mod VerifySignatures {
        use super::*;
        #[inline(never)]
        pub fn execute<'a, 'b: 'a, 'c>(
            p: &Pubkey,
            a: &'c [AccountInfo<'b>],
            d: &[u8],
        ) -> Result<()> {
            let ix_data: VerifySignaturesData = BorshDeserialize::try_from_slice(d)
                .map_err(|e| SolitaireError::InstructionDeserializeFailed(e))?;
            let mut accounts = FromAccounts::from(p, &mut a.iter(), &())?;
            verify_signatures(
                &ExecutionContext {
                    program_id: p,
                    accounts: a,
                },
                &mut accounts,
                ix_data,
            )?;
            Persist::persist(&accounts, p)?;
            Ok(())
        }
    }
    /// Generated:
    /// This Instruction contains a 1-1 mapping for each enum variant to function call. The
    /// function calls can be found below in the `api` module.
    #[repr(u8)]
    pub enum Instruction {
        Initialize,
        PostMessage,
        PostVAA,
        SetFees,
        TransferFees,
        UpgradeContract,
        UpgradeGuardianSet,
        VerifySignatures,
    }
    impl borsh::ser::BorshSerialize for Instruction {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> core::result::Result<(), borsh::maybestd::io::Error> {
            match self {
                Instruction::Initialize => {
                    let variant_idx: u8 = 0u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::PostMessage => {
                    let variant_idx: u8 = 1u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::PostVAA => {
                    let variant_idx: u8 = 2u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::SetFees => {
                    let variant_idx: u8 = 3u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::TransferFees => {
                    let variant_idx: u8 = 4u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::UpgradeContract => {
                    let variant_idx: u8 = 5u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::UpgradeGuardianSet => {
                    let variant_idx: u8 = 6u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
                Instruction::VerifySignatures => {
                    let variant_idx: u8 = 7u8;
                    writer.write_all(&variant_idx.to_le_bytes())?;
                }
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Instruction {
        fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
            let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
            let return_value = match variant_idx {
                0u8 => Instruction::Initialize,
                1u8 => Instruction::PostMessage,
                2u8 => Instruction::PostVAA,
                3u8 => Instruction::SetFees,
                4u8 => Instruction::TransferFees,
                5u8 => Instruction::UpgradeContract,
                6u8 => Instruction::UpgradeGuardianSet,
                7u8 => Instruction::VerifySignatures,
                _ => {
                    let msg = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unexpected variant index: "],
                            &match (&variant_idx,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Debug::fmt,
                                )],
                            },
                        ));
                        res
                    };
                    return Err(borsh::maybestd::io::Error::new(
                        borsh::maybestd::io::ErrorKind::InvalidInput,
                        msg,
                    ));
                }
            };
            Ok(return_value)
        }
    }
    /// This entrypoint is generated from the enum above, it deserializes incoming bytes
    /// and automatically dispatches to the correct method.
    pub fn dispatch<'a, 'b: 'a, 'c>(p: &Pubkey, a: &'c [AccountInfo<'b>], d: &[u8]) -> Result<()> {
        match d[0] {
            n if n == Instruction::Initialize as u8 => Initialize::execute(p, a, &d[1..]),
            n if n == Instruction::PostMessage as u8 => PostMessage::execute(p, a, &d[1..]),
            n if n == Instruction::PostVAA as u8 => PostVAA::execute(p, a, &d[1..]),
            n if n == Instruction::SetFees as u8 => SetFees::execute(p, a, &d[1..]),
            n if n == Instruction::TransferFees as u8 => TransferFees::execute(p, a, &d[1..]),
            n if n == Instruction::UpgradeContract as u8 => UpgradeContract::execute(p, a, &d[1..]),
            n if n == Instruction::UpgradeGuardianSet as u8 => {
                UpgradeGuardianSet::execute(p, a, &d[1..])
            }
            n if n == Instruction::VerifySignatures as u8 => {
                VerifySignatures::execute(p, a, &d[1..])
            }
            other => Err(SolitaireError::UnknownInstruction(other)),
        }
    }
    pub fn solitaire(p: &Pubkey, a: &[AccountInfo], d: &[u8]) -> ProgramResult {
        if let Err(err) = dispatch(p, a, d) {
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Error: "],
                    &match (&err,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                ));
                res
            });
            return Err(err.into());
        }
        Ok(())
    }
}
pub use instruction::solitaire;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match solitaire(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
