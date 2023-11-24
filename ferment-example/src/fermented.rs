#[allow(
    dead_code,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables
)]
pub mod types {
    use crate::RootStruct;
    pub mod example {
        pub mod address {
            use crate::chain::common::chain_type::ChainType;
            use crate::example::address::address_with_script_pubkey;
            use crate::example::address::get_chain_hashes_by_map;
            use crate::example::address::get_chain_type_string;
            use crate::fermented::generics::std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI;
            use crate::fermented::generics::Vec_u8_FFI;
            use crate::fermented::types::chain::common::chain_type::ChainType_FFI;
            use crate::fermented::types::nested::HashID_FFI;
            use crate::nested::HashID;
            use std::collections::BTreeMap;
            #[doc = "FFI-representation of the get_chain_hashes_by_map"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_get_chain_hashes_by_map(
                map : * mut std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI,
            ) -> *mut std::os::raw::c_char {
                let obj = get_chain_hashes_by_map(ferment_interfaces::FFIConversion::ffi_from(map));
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the address_with_script_pubkey"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_with_script_pubkey(
                script: *mut Vec_u8_FFI,
            ) -> *mut std::os::raw::c_char {
                let obj = address_with_script_pubkey({
                    let vec = &*script;
                    {
                        let vec = vec;
                        ferment_interfaces::from_simple_vec(vec.values, vec.count)
                    }
                });
                ferment_interfaces::FFIConversion::ffi_to_opt(obj)
            }
            #[doc = "FFI-representation of the get_chain_type_string"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_get_chain_type_string(
                chain_type: *mut ChainType_FFI,
            ) -> *mut std::os::raw::c_char {
                let obj =
                    get_chain_type_string(ferment_interfaces::FFIConversion::ffi_from(chain_type));
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
    }
    #[doc = "FFI-representation of the # [doc = \"FFI-representation of the RootStruct\"]"]
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct RootStruct_FFI {
        pub name: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<RootStruct> for RootStruct_FFI {
        unsafe fn ffi_from_const(ffi: *const RootStruct_FFI) -> RootStruct {
            let ffi_ref = &*ffi;
            RootStruct {
                name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
            }
        }
        unsafe fn ffi_to_const(obj: RootStruct) -> *const RootStruct_FFI {
            ferment_interfaces::boxed(RootStruct_FFI {
                name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
            })
        }
        unsafe fn destroy(ffi: *mut RootStruct_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for RootStruct_FFI {
        fn drop(&mut self) {
            unsafe {
                let ffi_ref = self;
                <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                    ffi_ref.name,
                );
            }
        }
    }
    pub mod nested {
        use crate::fermented::generics::std_collections_Map_keys_String_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_crate_nested_HashID_values_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI;
        use crate::fermented::generics::std_collections_Map_keys_u32_values_u32_FFI;
        use crate::fermented::generics::Vec_Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::Vec_Vec_u32_FFI;
        use crate::fermented::generics::Vec_bool_FFI;
        use crate::fermented::generics::Vec_crate_nested_HashID_FFI;
        use crate::fermented::generics::Vec_u32_FFI;
        use crate::fermented::generics::Vec_u8_FFI;
        use crate::nested::find_hash_by_u32;
        use crate::nested::AddInsightCallback;
        use crate::nested::ArrayOfArraysOfHashes;
        use crate::nested::BinaryData;
        use crate::nested::DataContractNotPresentError;
        use crate::nested::Hash160;
        use crate::nested::HashID;
        use crate::nested::Identifier;
        use crate::nested::IdentifierBytes32;
        use crate::nested::KeyID;
        use crate::nested::MapOfHashes;
        use crate::nested::MapOfVecHashes;
        use crate::nested::ProtocolError;
        use crate::nested::ShouldProcessDiffWithRangeCallback;
        use crate::nested::SimpleData;
        use crate::nested::TestEnum;
        use crate::nested::TestStruct;
        use crate::nested::UnnamedPair;
        use crate::nested::UsedKeyMatrix;
        use crate::nested::UsedStruct;
        use ferment_interfaces;
        use std::collections::BTreeMap;
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the MapOfVecHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct MapOfVecHashes_FFI(
            *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI,
        );
        impl ferment_interfaces::FFIConversion<MapOfVecHashes> for MapOfVecHashes_FFI {
            unsafe fn ffi_from_const(ffi: *const MapOfVecHashes_FFI) -> MapOfVecHashes {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: MapOfVecHashes) -> *const MapOfVecHashes_FFI {
                ferment_interfaces::boxed(MapOfVecHashes_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut MapOfVecHashes_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for MapOfVecHashes_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the UsedStruct\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UsedStruct_FFI(*mut HashID_FFI);
        impl ferment_interfaces::FFIConversion<UsedStruct> for UsedStruct_FFI {
            unsafe fn ffi_from_const(ffi: *const UsedStruct_FFI) -> UsedStruct {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: UsedStruct) -> *const UsedStruct_FFI {
                ferment_interfaces::boxed(UsedStruct_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut UsedStruct_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UsedStruct_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <HashID_FFI as ferment_interfaces::FFIConversion<HashID>>::destroy(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the AddInsightCallback_FFI"]
        #[allow(non_camel_case_types)]
        pub type AddInsightCallback_FFI = unsafe extern "C" fn(
            block_hash: *mut HashID_FFI,
            context: ferment_interfaces::OpaqueContextFFI,
        );
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the HashID\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct HashID_FFI(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<HashID> for HashID_FFI {
            unsafe fn ffi_from_const(ffi: *const HashID_FFI) -> HashID {
                let ffi_ref = &*ffi;
                *ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: HashID) -> *const HashID_FFI {
                ferment_interfaces::boxed(HashID_FFI(ferment_interfaces::boxed(obj)))
            }
            unsafe fn destroy(ffi: *mut HashID_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for HashID_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the Hash160\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct Hash160_FFI(*mut [u8; 20]);
        impl ferment_interfaces::FFIConversion<Hash160> for Hash160_FFI {
            unsafe fn ffi_from_const(ffi: *const Hash160_FFI) -> Hash160 {
                let ffi_ref = &*ffi;
                *ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: Hash160) -> *const Hash160_FFI {
                ferment_interfaces::boxed(Hash160_FFI(ferment_interfaces::boxed(obj)))
            }
            unsafe fn destroy(ffi: *mut Hash160_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for Hash160_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the IdentifierBytes32\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct IdentifierBytes32_FFI(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<IdentifierBytes32> for IdentifierBytes32_FFI {
            unsafe fn ffi_from_const(ffi: *const IdentifierBytes32_FFI) -> IdentifierBytes32 {
                let ffi_ref = &*ffi;
                IdentifierBytes32(*ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: IdentifierBytes32) -> *const IdentifierBytes32_FFI {
                ferment_interfaces::boxed(IdentifierBytes32_FFI(ferment_interfaces::boxed(obj.0)))
            }
            unsafe fn destroy(ffi: *mut IdentifierBytes32_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for IdentifierBytes32_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the KeyID\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct KeyID_FFI(u32);
        impl ferment_interfaces::FFIConversion<KeyID> for KeyID_FFI {
            unsafe fn ffi_from_const(ffi: *const KeyID_FFI) -> KeyID {
                let ffi_ref = &*ffi;
                ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: KeyID) -> *const KeyID_FFI {
                ferment_interfaces::boxed(KeyID_FFI(obj))
            }
            unsafe fn destroy(ffi: *mut KeyID_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for KeyID_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    {};
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the BinaryData\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct BinaryData_FFI(*mut Vec_u8_FFI);
        impl ferment_interfaces::FFIConversion<BinaryData> for BinaryData_FFI {
            unsafe fn ffi_from_const(ffi: *const BinaryData_FFI) -> BinaryData {
                let ffi_ref = &*ffi;
                BinaryData({
                    let vec = &*ffi_ref.0;
                    {
                        let vec = vec;
                        ferment_interfaces::from_simple_vec(vec.values, vec.count)
                    }
                })
            }
            unsafe fn ffi_to_const(obj: BinaryData) -> *const BinaryData_FFI {
                ferment_interfaces::boxed(BinaryData_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj.0),
                ))
            }
            unsafe fn destroy(ffi: *mut BinaryData_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for BinaryData_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the Identifier\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct Identifier_FFI(*mut IdentifierBytes32_FFI);
        impl ferment_interfaces::FFIConversion<Identifier> for Identifier_FFI {
            unsafe fn ffi_from_const(ffi: *const Identifier_FFI) -> Identifier {
                let ffi_ref = &*ffi;
                Identifier(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: Identifier) -> *const Identifier_FFI {
                ferment_interfaces::boxed(Identifier_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj.0),
                ))
            }
            unsafe fn destroy(ffi: *mut Identifier_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for Identifier_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <IdentifierBytes32_FFI as ferment_interfaces::FFIConversion<
                        IdentifierBytes32,
                    >>::destroy(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the ShouldProcessDiffWithRangeCallback_FFI"]
        #[allow(non_camel_case_types)]
        pub type ShouldProcessDiffWithRangeCallback_FFI =
            unsafe extern "C" fn(
                base_block_hash: *mut HashID_FFI,
                block_hash: *mut HashID_FFI,
                context: ferment_interfaces::OpaqueContextFFI,
            ) -> *mut ProtocolError_FFI;
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the UnnamedPair\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UnnamedPair_FFI(*mut [u8; 32], u32);
        impl ferment_interfaces::FFIConversion<UnnamedPair> for UnnamedPair_FFI {
            unsafe fn ffi_from_const(ffi: *const UnnamedPair_FFI) -> UnnamedPair {
                let ffi_ref = &*ffi;
                UnnamedPair(*ffi_ref.0, ffi_ref.1)
            }
            unsafe fn ffi_to_const(obj: UnnamedPair) -> *const UnnamedPair_FFI {
                ferment_interfaces::boxed(UnnamedPair_FFI(ferment_interfaces::boxed(obj.0), obj.1))
            }
            unsafe fn destroy(ffi: *mut UnnamedPair_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UnnamedPair_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                    {};
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the SimpleData\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct SimpleData_FFI(*mut Vec_u32_FFI);
        impl ferment_interfaces::FFIConversion<SimpleData> for SimpleData_FFI {
            unsafe fn ffi_from_const(ffi: *const SimpleData_FFI) -> SimpleData {
                let ffi_ref = &*ffi;
                SimpleData({
                    let vec = &*ffi_ref.0;
                    {
                        let vec = vec;
                        ferment_interfaces::from_simple_vec(vec.values, vec.count)
                    }
                })
            }
            unsafe fn ffi_to_const(obj: SimpleData) -> *const SimpleData_FFI {
                ferment_interfaces::boxed(SimpleData_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj.0),
                ))
            }
            unsafe fn destroy(ffi: *mut SimpleData_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for SimpleData_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the ArrayOfArraysOfHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct ArrayOfArraysOfHashes_FFI(*mut Vec_Vec_crate_nested_HashID_FFI);
        impl ferment_interfaces::FFIConversion<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
            unsafe fn ffi_from_const(
                ffi: *const ArrayOfArraysOfHashes_FFI,
            ) -> ArrayOfArraysOfHashes {
                let ffi_ref = &*ffi;
                {
                    let vec = &*ffi_ref.0;
                    let count = vec.count;
                    let values = vec.values;
                    (0..count)
                        .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                        .collect()
                }
            }
            unsafe fn ffi_to_const(obj: ArrayOfArraysOfHashes) -> *const ArrayOfArraysOfHashes_FFI {
                ferment_interfaces::boxed(ArrayOfArraysOfHashes_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut ArrayOfArraysOfHashes_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ArrayOfArraysOfHashes_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the UsedKeyMatrix\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UsedKeyMatrix_FFI(*mut Vec_bool_FFI);
        impl ferment_interfaces::FFIConversion<UsedKeyMatrix> for UsedKeyMatrix_FFI {
            unsafe fn ffi_from_const(ffi: *const UsedKeyMatrix_FFI) -> UsedKeyMatrix {
                let ffi_ref = &*ffi;
                {
                    let vec = &*ffi_ref.0;
                    {
                        let vec = vec;
                        ferment_interfaces::from_simple_vec(vec.values, vec.count)
                    }
                }
            }
            unsafe fn ffi_to_const(obj: UsedKeyMatrix) -> *const UsedKeyMatrix_FFI {
                ferment_interfaces::boxed(UsedKeyMatrix_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut UsedKeyMatrix_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UsedKeyMatrix_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the TestStruct\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct TestStruct_FFI { pub vec_u8 : * mut Vec_u8_FFI , pub vec_u32 : * mut Vec_u32_FFI , pub vec_vec_u32 : * mut Vec_Vec_u32_FFI , pub map_key_simple_value_simple : * mut std_collections_Map_keys_u32_values_u32_FFI , pub map_key_simple_value_complex : * mut std_collections_Map_keys_u32_values_crate_nested_HashID_FFI , pub map_key_simple_value_vec_simple : * mut std_collections_Map_keys_u32_values_Vec_u32_FFI , pub map_key_simple_value_vec_complex : * mut std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI , pub map_key_simple_value_map_key_simple_value_simple : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI , pub map_key_simple_value_map_key_simple_value_complex : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI , pub map_key_simple_value_map_key_simple_value_vec_simple : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI , pub map_key_simple_value_map_key_simple_value_vec_complex : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI , pub map_key_complex_value_simple : * mut std_collections_Map_keys_crate_nested_HashID_values_u32_FFI , pub map_key_complex_value_complex : * mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI , pub map_key_complex_value_vec_simple : * mut std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI , pub map_key_complex_value_vec_complex : * mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI , pub map_key_complex_value_map_key_simple_value_vec_simple : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI , pub map_key_complex_value_map_key_simple_value_vec_complex : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI , pub map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI , }
        impl ferment_interfaces::FFIConversion<TestStruct> for TestStruct_FFI {
            unsafe fn ffi_from_const(ffi: *const TestStruct_FFI) -> TestStruct {
                let ffi_ref = &*ffi;
                TestStruct { vec_u8 : { let vec = & * ffi_ref . vec_u8 ; { let vec = vec ; ferment_interfaces :: from_simple_vec (vec . values , vec . count) } } , vec_u32 : { let vec = & * ffi_ref . vec_u32 ; { let vec = vec ; ferment_interfaces :: from_simple_vec (vec . values , vec . count) } } , vec_vec_u32 : { let vec = & * ffi_ref . vec_vec_u32 ; let count = vec . count ; let values = vec . values ; (0 .. count) . map (| i | ferment_interfaces :: FFIConversion :: ffi_from_const (* values . add (i))) . collect () } , map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_simple) , map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_complex) , map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_vec_simple) , map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_vec_complex) , map_key_simple_value_map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_simple) , map_key_simple_value_map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_complex) , map_key_simple_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_vec_simple) , map_key_simple_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_vec_complex) , map_key_complex_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_simple) , map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_complex) , map_key_complex_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_vec_simple) , map_key_complex_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_vec_complex) , map_key_complex_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_vec_simple) , map_key_complex_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_vec_complex) , map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) , }
            }
            unsafe fn ffi_to_const(obj: TestStruct) -> *const TestStruct_FFI {
                ferment_interfaces :: boxed (TestStruct_FFI { vec_u8 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_u8) , vec_u32 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_u32) , vec_vec_u32 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_vec_u32) , map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_simple) , map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_complex) , map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_vec_simple) , map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_vec_complex) , map_key_simple_value_map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_simple) , map_key_simple_value_map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_complex) , map_key_simple_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_vec_simple) , map_key_simple_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_vec_complex) , map_key_complex_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_simple) , map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_complex) , map_key_complex_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_vec_simple) , map_key_complex_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_vec_complex) , map_key_complex_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_vec_simple) , map_key_complex_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_vec_complex) , map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) , })
            }
            unsafe fn destroy(ffi: *mut TestStruct_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for TestStruct_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.vec_u8);
                    ferment_interfaces::unbox_any(ffi_ref.vec_u32);
                    ferment_interfaces::unbox_any(ffi_ref.vec_vec_u32);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_complex);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_vec_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_vec_complex);
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_complex,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_vec_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_vec_complex,
                    );
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_complex);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_vec_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_vec_complex);
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_complex_value_map_key_simple_value_vec_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_complex_value_map_key_simple_value_vec_complex,
                    );
                    ferment_interfaces :: unbox_any (ffi_ref . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) ;
                }
            }
        }
        #[doc = "FFI-representation of the TestEnum"]
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum TestEnum_FFI {
            Variant1(*mut std::os::raw::c_char),
            Variant2,
            Variant3(*mut HashID_FFI, u32),
            Variant4(*mut HashID_FFI, u32, *mut std::os::raw::c_char),
            Variant5(
                *mut std_collections_Map_keys_String_values_crate_nested_HashID_FFI,
                u32,
                *mut std::os::raw::c_char,
            ),
        }
        impl ferment_interfaces::FFIConversion<TestEnum> for TestEnum_FFI {
            unsafe fn ffi_from_const(ffi: *const TestEnum_FFI) -> TestEnum {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    TestEnum_FFI::Variant1(o_0) => {
                        TestEnum::Variant1(ferment_interfaces::FFIConversion::ffi_from(*o_0))
                    }
                    TestEnum_FFI::Variant2 => TestEnum::Variant2,
                    TestEnum_FFI::Variant3(o_0, o_1) => {
                        TestEnum::Variant3(ferment_interfaces::FFIConversion::ffi_from(*o_0), *o_1)
                    }
                    TestEnum_FFI::Variant4(o_0, o_1, o_2) => TestEnum::Variant4(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        *o_1,
                        ferment_interfaces::FFIConversion::ffi_from(*o_2),
                    ),
                    TestEnum_FFI::Variant5(o_0, o_1, o_2) => TestEnum::Variant5(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        *o_1,
                        ferment_interfaces::FFIConversion::ffi_from(*o_2),
                    ),
                }
            }
            unsafe fn ffi_to_const(obj: TestEnum) -> *const TestEnum_FFI {
                ferment_interfaces::boxed(match obj {
                    TestEnum::Variant1(o_0) => {
                        TestEnum_FFI::Variant1(ferment_interfaces::FFIConversion::ffi_to(o_0))
                    }
                    TestEnum::Variant2 => TestEnum_FFI::Variant2,
                    TestEnum::Variant3(o_0, o_1) => {
                        TestEnum_FFI::Variant3(ferment_interfaces::FFIConversion::ffi_to(o_0), o_1)
                    }
                    TestEnum::Variant4(o_0, o_1, o_2) => TestEnum_FFI::Variant4(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                        o_1,
                        ferment_interfaces::FFIConversion::ffi_to(o_2),
                    ),
                    TestEnum::Variant5(o_0, o_1, o_2) => TestEnum_FFI::Variant5(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                        o_1,
                        ferment_interfaces::FFIConversion::ffi_to(o_2),
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut TestEnum_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for TestEnum_FFI {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        TestEnum_FFI::Variant1(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        TestEnum_FFI::Variant2 => {}
                        TestEnum_FFI::Variant3(o_0, o_1) => {
                            <HashID_FFI as ferment_interfaces::FFIConversion<HashID>>::destroy(
                                o_0.to_owned(),
                            );
                            {};
                        }
                        TestEnum_FFI::Variant4(o_0, o_1, o_2) => {
                            <HashID_FFI as ferment_interfaces::FFIConversion<HashID>>::destroy(
                                o_0.to_owned(),
                            );
                            {};
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_2 . to_owned ()) ;
                        }
                        TestEnum_FFI::Variant5(o_0, o_1, o_2) => {
                            ferment_interfaces::unbox_any(o_0.to_owned());
                            {};
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_2 . to_owned ()) ;
                        }
                    }
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the MapOfHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct MapOfHashes_FFI(
            *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI,
        );
        impl ferment_interfaces::FFIConversion<MapOfHashes> for MapOfHashes_FFI {
            unsafe fn ffi_from_const(ffi: *const MapOfHashes_FFI) -> MapOfHashes {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: MapOfHashes) -> *const MapOfHashes_FFI {
                ferment_interfaces::boxed(MapOfHashes_FFI(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut MapOfHashes_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for MapOfHashes_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the DataContractNotPresentError\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct DataContractNotPresentError_FFI {
            pub data_contract_id: *mut Identifier_FFI,
        }
        impl ferment_interfaces::FFIConversion<DataContractNotPresentError>
            for DataContractNotPresentError_FFI
        {
            unsafe fn ffi_from_const(
                ffi: *const DataContractNotPresentError_FFI,
            ) -> DataContractNotPresentError {
                let ffi_ref = &*ffi;
                DataContractNotPresentError {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_from(
                        ffi_ref.data_contract_id,
                    ),
                }
            }
            unsafe fn ffi_to_const(
                obj: DataContractNotPresentError,
            ) -> *const DataContractNotPresentError_FFI {
                ferment_interfaces::boxed(DataContractNotPresentError_FFI {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_to(
                        obj.data_contract_id,
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut DataContractNotPresentError_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for DataContractNotPresentError_FFI {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <Identifier_FFI as ferment_interfaces::FFIConversion<Identifier>>::destroy(
                        ffi_ref.data_contract_id,
                    );
                }
            }
        }
        #[doc = "FFI-representation of the ProtocolError"]
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum ProtocolError_FFI {
            IdentifierError(*mut std::os::raw::c_char),
            StringDecodeError(*mut std::os::raw::c_char),
            StringDecodeError2(*mut std::os::raw::c_char, u32),
            EmptyPublicKeyDataError,
            MaxEncodedBytesReachedError {
                max_size_kbytes: usize,
                size_hit: usize,
            },
            EncodingError(*mut std::os::raw::c_char),
            EncodingError2(*mut std::os::raw::c_char),
            DataContractNotPresentError(*mut DataContractNotPresentError_FFI),
        }
        impl ferment_interfaces::FFIConversion<ProtocolError> for ProtocolError_FFI {
            unsafe fn ffi_from_const(ffi: *const ProtocolError_FFI) -> ProtocolError {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    ProtocolError_FFI::IdentifierError(o_0) => ProtocolError::IdentifierError(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                    ),
                    ProtocolError_FFI::StringDecodeError(o_0) => ProtocolError::StringDecodeError(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                    ),
                    ProtocolError_FFI::StringDecodeError2(o_0, o_1) => {
                        ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            *o_1,
                        )
                    }
                    ProtocolError_FFI::EmptyPublicKeyDataError => {
                        ProtocolError::EmptyPublicKeyDataError
                    }
                    ProtocolError_FFI::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes: *max_size_kbytes,
                        size_hit: *size_hit,
                    },
                    ProtocolError_FFI::EncodingError(o_0) => ProtocolError::EncodingError(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                    ),
                    ProtocolError_FFI::EncodingError2(o_0) => ProtocolError::EncodingError2(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                    ),
                    ProtocolError_FFI::DataContractNotPresentError(o_0) => {
                        ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                }
            }
            unsafe fn ffi_to_const(obj: ProtocolError) -> *const ProtocolError_FFI {
                ferment_interfaces::boxed(match obj {
                    ProtocolError::IdentifierError(o_0) => ProtocolError_FFI::IdentifierError(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                    ),
                    ProtocolError::StringDecodeError(o_0) => ProtocolError_FFI::StringDecodeError(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                    ),
                    ProtocolError::StringDecodeError2(o_0, o_1) => {
                        ProtocolError_FFI::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                            o_1,
                        )
                    }
                    ProtocolError::EmptyPublicKeyDataError => {
                        ProtocolError_FFI::EmptyPublicKeyDataError
                    }
                    ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => ProtocolError_FFI::MaxEncodedBytesReachedError {
                        max_size_kbytes: max_size_kbytes,
                        size_hit: size_hit,
                    },
                    ProtocolError::EncodingError(o_0) => ProtocolError_FFI::EncodingError(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                    ),
                    ProtocolError::EncodingError2(o_0) => ProtocolError_FFI::EncodingError2(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                    ),
                    ProtocolError::DataContractNotPresentError(o_0) => {
                        ProtocolError_FFI::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                        )
                    }
                })
            }
            unsafe fn destroy(ffi: *mut ProtocolError_FFI) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ProtocolError_FFI {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        ProtocolError_FFI::IdentifierError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError_FFI::StringDecodeError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError_FFI::StringDecodeError2(o_0, o_1) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                            {};
                        }
                        ProtocolError_FFI::EmptyPublicKeyDataError => {}
                        ProtocolError_FFI::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => {
                            {}
                            {}
                        }
                        ProtocolError_FFI::EncodingError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError_FFI::EncodingError2(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError_FFI::DataContractNotPresentError(o_0) => {
                            < DataContractNotPresentError_FFI as ferment_interfaces :: FFIConversion < DataContractNotPresentError >> :: destroy (o_0 . to_owned ()) ;
                        }
                    }
                }
            }
        }
        #[doc = "FFI-representation of the find_hash_by_u32"]
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ffi_find_hash_by_u32(
            key: u32,
            map: *mut std_collections_Map_keys_u32_values_crate_nested_HashID_FFI,
        ) -> *mut HashID_FFI {
            let obj = find_hash_by_u32(key, ferment_interfaces::FFIConversion::ffi_from(map));
            ferment_interfaces::FFIConversion::ffi_to_opt(obj)
        }
    }
    pub mod chain {
        pub mod common {
            pub mod chain_type {
                use crate::chain::common::chain_type::ChainType;
                use crate::chain::common::chain_type::DevnetType;
                use crate::chain::common::chain_type::IHaveChainSettings;
                use crate::fermented::types::nested::HashID_FFI;
                use crate::nested::HashID;
                #[doc = "FFI-representation of the DevnetType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum DevnetType_FFI {
                    JackDaniels = 0,
                    Devnet333 = 1,
                    Chacha = 2,
                    Mojito = 3,
                    WhiteRussian = 4,
                }
                impl ferment_interfaces::FFIConversion<DevnetType> for DevnetType_FFI {
                    unsafe fn ffi_from_const(ffi: *const DevnetType_FFI) -> DevnetType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            DevnetType_FFI::JackDaniels => DevnetType::JackDaniels,
                            DevnetType_FFI::Devnet333 => DevnetType::Devnet333,
                            DevnetType_FFI::Chacha => DevnetType::Chacha,
                            DevnetType_FFI::Mojito => DevnetType::Mojito,
                            DevnetType_FFI::WhiteRussian => DevnetType::WhiteRussian,
                        }
                    }
                    unsafe fn ffi_to_const(obj: DevnetType) -> *const DevnetType_FFI {
                        ferment_interfaces::boxed(match obj {
                            DevnetType::JackDaniels => DevnetType_FFI::JackDaniels,
                            DevnetType::Devnet333 => DevnetType_FFI::Devnet333,
                            DevnetType::Chacha => DevnetType_FFI::Chacha,
                            DevnetType::Mojito => DevnetType_FFI::Mojito,
                            DevnetType::WhiteRussian => DevnetType_FFI::WhiteRussian,
                        })
                    }
                    unsafe fn destroy(ffi: *mut DevnetType_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for DevnetType_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                DevnetType_FFI::JackDaniels => {}
                                DevnetType_FFI::Devnet333 => {}
                                DevnetType_FFI::Chacha => {}
                                DevnetType_FFI::Mojito => {}
                                DevnetType_FFI::WhiteRussian => {}
                            }
                        }
                    }
                }
                #[allow(non_snake_case)]
                static DevnetType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn DevnetType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj.name();
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_genesis_hash(
                        obj: *const (),
                    ) -> *mut HashID_FFI {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj.genesis_hash();
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj.genesis_height();
                        obj
                    }
                    unsafe extern "C" fn DevnetType_has_genesis_hash(
                        obj: *const (),
                        hash: *mut HashID_FFI,
                    ) -> bool {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj
                            .has_genesis_hash(ferment_interfaces::FFIConversion::ffi_from(hash));
                        obj
                    }
                    unsafe extern "C" fn DevnetType_get_hash_by_hash(
                        obj: *const (),
                        hash: *mut HashID_FFI,
                    ) -> *mut HashID_FFI {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj
                            .get_hash_by_hash(ferment_interfaces::FFIConversion::ffi_from(hash));
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: u16,
                    ) -> bool {
                        let cast_obj = &(*(obj as *const DevnetType));
                        let obj = cast_obj.should_process_llmq_of_type(llmq_type);
                        obj
                    }
                    IHaveChainSettings_VTable {
                        name: DevnetType_name,
                        genesis_hash: DevnetType_genesis_hash,
                        genesis_height: DevnetType_genesis_height,
                        has_genesis_hash: DevnetType_has_genesis_hash,
                        get_hash_by_hash: DevnetType_get_hash_by_hash,
                        should_process_llmq_of_type: DevnetType_should_process_llmq_of_type,
                    }
                };
                #[doc = "FFI-representation of the ChainType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum ChainType_FFI {
                    MainNet,
                    TestNet,
                    DevNet(*mut DevnetType_FFI),
                }
                impl ferment_interfaces::FFIConversion<ChainType> for ChainType_FFI {
                    unsafe fn ffi_from_const(ffi: *const ChainType_FFI) -> ChainType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ChainType_FFI::MainNet => ChainType::MainNet,
                            ChainType_FFI::TestNet => ChainType::TestNet,
                            ChainType_FFI::DevNet(o_0) => {
                                ChainType::DevNet(ferment_interfaces::FFIConversion::ffi_from(*o_0))
                            }
                        }
                    }
                    unsafe fn ffi_to_const(obj: ChainType) -> *const ChainType_FFI {
                        ferment_interfaces::boxed(match obj {
                            ChainType::MainNet => ChainType_FFI::MainNet,
                            ChainType::TestNet => ChainType_FFI::TestNet,
                            ChainType::DevNet(o_0) => ChainType_FFI::DevNet(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ChainType_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ChainType_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ChainType_FFI::MainNet => {}
                                ChainType_FFI::TestNet => {}
                                ChainType_FFI::DevNet(o_0) => {
                                    <DevnetType_FFI as ferment_interfaces::FFIConversion<
                                        DevnetType,
                                    >>::destroy(o_0.to_owned());
                                }
                            }
                        }
                    }
                }
                #[allow(non_snake_case)]
                static ChainType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn ChainType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj.name();
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_genesis_hash(obj: *const ()) -> *mut HashID_FFI {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj.genesis_hash();
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj.genesis_height();
                        obj
                    }
                    unsafe extern "C" fn ChainType_has_genesis_hash(
                        obj: *const (),
                        hash: *mut HashID_FFI,
                    ) -> bool {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj
                            .has_genesis_hash(ferment_interfaces::FFIConversion::ffi_from(hash));
                        obj
                    }
                    unsafe extern "C" fn ChainType_get_hash_by_hash(
                        obj: *const (),
                        hash: *mut HashID_FFI,
                    ) -> *mut HashID_FFI {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj
                            .get_hash_by_hash(ferment_interfaces::FFIConversion::ffi_from(hash));
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: u16,
                    ) -> bool {
                        let cast_obj = &(*(obj as *const ChainType));
                        let obj = cast_obj.should_process_llmq_of_type(llmq_type);
                        obj
                    }
                    IHaveChainSettings_VTable {
                        name: ChainType_name,
                        genesis_hash: ChainType_genesis_hash,
                        genesis_height: ChainType_genesis_height,
                        has_genesis_hash: ChainType_has_genesis_hash,
                        get_hash_by_hash: ChainType_get_hash_by_hash,
                        should_process_llmq_of_type: ChainType_should_process_llmq_of_type,
                    }
                };
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_VTable {
                    pub name: unsafe extern "C" fn(obj: *const ()) -> *mut std::os::raw::c_char,
                    pub genesis_hash: unsafe extern "C" fn(obj: *const ()) -> *mut HashID_FFI,
                    pub genesis_height: unsafe extern "C" fn(obj: *const ()) -> u32,
                    pub has_genesis_hash:
                        unsafe extern "C" fn(obj: *const (), hash: *mut HashID_FFI) -> bool,
                    pub get_hash_by_hash: unsafe extern "C" fn(
                        obj: *const (),
                        hash: *mut HashID_FFI,
                    )
                        -> *mut HashID_FFI,
                    pub should_process_llmq_of_type:
                        unsafe extern "C" fn(obj: *const (), llmq_type: u16) -> bool,
                }
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_TraitObject {
                    pub object: *const (),
                    pub vtable: *const IHaveChainSettings_VTable,
                }
            }
        }
    }
}
#[allow(
    dead_code,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables
)]
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut *mut std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: complex_vec_iterator :: < crate :: nested :: HashID , crate :: fermented :: types :: nested :: HashID_FFI > (obj . keys () . cloned ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > , std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_Vec_u32_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut Vec_u32_FFI,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Vec<u32>>>
        for std_collections_Map_keys_u32_values_Vec_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_u32_FFI,
        ) -> std::collections::BTreeMap<u32, Vec<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Vec<u32>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_u32_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::complex_vec_iterator::<Vec<u32>, Vec_u32_FFI>(
                    obj.values().cloned(),
                ),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Vec_u32_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI
    {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std_collections_Map_keys_u32_values_crate_nested_HashID_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_simple_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_simple_vec (obj . keys () . cloned () . collect ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > , std_collections_Map_keys_u32_values_crate_nested_HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_crate_nested_HashID_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::types::nested::HashID_FFI,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, crate::nested::HashID>>
        for std_collections_Map_keys_u32_values_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_crate_nested_HashID_FFI,
        ) -> std::collections::BTreeMap<u32, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_u32_values_crate_nested_HashID_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_crate_nested_HashID_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_u32_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, u32>>
        for std_collections_Map_keys_u32_values_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_u32_FFI,
        ) -> std::collections::BTreeMap<u32, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, u32>,
        ) -> *const std_collections_Map_keys_u32_values_u32_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::to_simple_vec(obj.values().cloned().collect()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_u32_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_u32_FFI {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<crate::nested::HashID, u32>>
        for std_collections_Map_keys_crate_nested_HashID_values_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_u32_FFI,
        ) -> std::collections::BTreeMap<crate::nested::HashID, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, u32>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_u32_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.keys().cloned()),
                values: ferment_interfaces::to_simple_vec(
                    obj.values().cloned().collect::<Vec<_>>(),
                ),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_u32_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut *mut Vec_u32_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, Vec<u32>>,
        > for std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI,
        ) -> std::collections::BTreeMap<crate::nested::HashID, Vec<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, Vec<u32>>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.keys().cloned()),
                values: ferment_interfaces::complex_vec_iterator::<Vec<u32>, Vec_u32_FFI>(
                    obj.values().cloned(),
                ),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_nested_HashID_FFI {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::nested::HashID_FFI,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::nested::HashID>> for Vec_crate_nested_HashID_FFI {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_nested_HashID_FFI,
        ) -> Vec<crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::nested::HashID>,
        ) -> *const Vec_crate_nested_HashID_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_nested_HashID_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_nested_HashID_FFI {
        type Value = crate::nested::HashID;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            {
                let count = self.count;
                let values = self.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            }
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::complex_vec_iterator::<
                    Self::Value,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut *mut crate::fermented::types::nested::HashID_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>,
        > for std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI,
        ) -> std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.keys().cloned()),
                values: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std_collections_Map_keys_u32_values_Vec_u32_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, Vec<u32>>>,
        > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI,
        ) -> std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, Vec<u32>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }        unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < u32 > > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI{
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::complex_vec_iterator::<
                    std::collections::BTreeMap<u32, Vec<u32>>,
                    std_collections_Map_keys_u32_values_Vec_u32_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut Vec_crate_nested_HashID_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, Vec<crate::nested::HashID>>,
        > for std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI,
        ) -> std::collections::BTreeMap<u32, Vec<crate::nested::HashID>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Vec<crate::nested::HashID>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::complex_vec_iterator::<
                    Vec<crate::nested::HashID>,
                    Vec_crate_nested_HashID_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_Vec_crate_nested_HashID_FFI {
        pub count: usize,
        pub values: *mut *mut Vec_crate_nested_HashID_FFI,
    }
    impl ferment_interfaces::FFIConversion<Vec<Vec<crate::nested::HashID>>>
        for Vec_Vec_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_Vec_crate_nested_HashID_FFI,
        ) -> Vec<Vec<crate::nested::HashID>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(
            obj: Vec<Vec<crate::nested::HashID>>,
        ) -> *const Vec_Vec_crate_nested_HashID_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Vec_crate_nested_HashID_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Vec_crate_nested_HashID_FFI {
        type Value = Vec<crate::nested::HashID>;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            {
                let count = self.count;
                let values = self.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            }
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::complex_vec_iterator::<
                    Self::Value,
                    Vec_crate_nested_HashID_FFI,
                >(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Vec_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::chain::common::chain_type::ChainType_FFI,
        pub values: *mut *mut crate::fermented::types::nested::HashID_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID > > for std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID > { let ffi_ref = & * ffi ; ferment_interfaces :: from_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID >) -> * const std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: complex_vec_iterator :: < crate :: chain :: common :: chain_type :: ChainType , crate :: fermented :: types :: chain :: common :: chain_type :: ChainType_FFI > (obj . keys () . cloned ()) , values : ferment_interfaces :: complex_vec_iterator :: < crate :: nested :: HashID , crate :: fermented :: types :: nested :: HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_u8_FFI {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<u8>> for Vec_u8_FFI {
        unsafe fn ffi_from_const(ffi: *const Vec_u8_FFI) -> Vec<u8> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u8_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u8_FFI {
        type Value = u8;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            ferment_interfaces::from_simple_vec(self.values as *const Self::Value, self.count)
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u8_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std_collections_Map_keys_u32_values_u32_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>,
        > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI,
        ) -> std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>,
        ) -> *const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()),
                values: ferment_interfaces::complex_vec_iterator::<
                    std::collections::BTreeMap<u32, u32>,
                    std_collections_Map_keys_u32_values_u32_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI
    {
        pub count: usize,
        pub keys: *mut u32,
        pub values:
            *mut *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_simple_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_simple_vec (obj . keys () . cloned () . collect ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > , std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_bool_FFI {
        pub count: usize,
        pub values: *mut bool,
    }
    impl ferment_interfaces::FFIConversion<Vec<bool>> for Vec_bool_FFI {
        unsafe fn ffi_from_const(ffi: *const Vec_bool_FFI) -> Vec<bool> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_bool_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_bool_FFI {
        type Value = bool;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            ferment_interfaces::from_simple_vec(self.values as *const Self::Value, self.count)
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_bool_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut *mut std_collections_Map_keys_u32_values_Vec_u32_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: complex_vec_iterator :: < crate :: nested :: HashID , crate :: fermented :: types :: nested :: HashID_FFI > (obj . keys () . cloned ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < u32 , Vec < u32 > > , std_collections_Map_keys_u32_values_Vec_u32_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: nested :: HashID_FFI , pub values : * mut * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: complex_vec_iterator :: < crate :: nested :: HashID , crate :: fermented :: types :: nested :: HashID_FFI > (obj . keys () . cloned ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > , std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID_FFI,
        pub values: *mut *mut Vec_crate_nested_HashID_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>,
        > for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI,
        ) -> std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.keys().cloned()),
                values: ferment_interfaces::complex_vec_iterator::<
                    Vec<crate::nested::HashID>,
                    Vec_crate_nested_HashID_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI
    {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: from_simple_complex_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_simple_vec (obj . keys () . cloned () . collect ()) , values : ferment_interfaces :: complex_vec_iterator :: < std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > , std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI > (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI) { ferment_interfaces :: unbox_any (ffi) ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_u32_FFI {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Vec<u32>> for Vec_u32_FFI {
        unsafe fn ffi_from_const(ffi: *const Vec_u32_FFI) -> Vec<u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(obj: Vec<u32>) -> *const Vec_u32_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u32_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u32_FFI {
        type Value = u32;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            ferment_interfaces::from_simple_vec(self.values as *const Self::Value, self.count)
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_Vec_u32_FFI {
        pub count: usize,
        pub values: *mut *mut Vec_u32_FFI,
    }
    impl ferment_interfaces::FFIConversion<Vec<Vec<u32>>> for Vec_Vec_u32_FFI {
        unsafe fn ffi_from_const(ffi: *const Vec_Vec_u32_FFI) -> Vec<Vec<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(obj: Vec<Vec<u32>>) -> *const Vec_Vec_u32_FFI {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Vec_u32_FFI) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Vec_u32_FFI {
        type Value = Vec<u32>;
        unsafe fn decode(&self) -> Vec<Self::Value> {
            {
                let count = self.count;
                let values = self.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            }
        }
        unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::complex_vec_iterator::<Self::Value, Vec_u32_FFI>(
                    obj.into_iter(),
                ),
            })
        }
    }
    impl Drop for Vec_Vec_u32_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_String_values_crate_nested_HashID_FFI {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut crate::fermented::types::nested::HashID_FFI,
    }
    impl
        ferment_interfaces::FFIConversion<std::collections::BTreeMap<String, crate::nested::HashID>>
        for std_collections_Map_keys_String_values_crate_nested_HashID_FFI
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_crate_nested_HashID_FFI,
        ) -> std::collections::BTreeMap<String, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<String, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_String_values_crate_nested_HashID_FFI {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::complex_vec_iterator::<String, std::os::raw::c_char>(
                    obj.keys().cloned(),
                ),
                values: ferment_interfaces::complex_vec_iterator::<
                    crate::nested::HashID,
                    crate::fermented::types::nested::HashID_FFI,
                >(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_String_values_crate_nested_HashID_FFI,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_crate_nested_HashID_FFI {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
}
