/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class exampleJNI {
  public final static native int KeyType_ECDSA_SECP256K1_get();
  public final static native int KeyType_BLS12_381_get();
  public final static native int KeyType_ECDSA_HASH160_get();
  public final static native int KeyType_BIP13_SCRIPT_HASH_get();
  public final static native int KeyType_EDDSA_25519_HASH160_get();
  public final static native int Purpose_AUTHENTICATION_get();
  public final static native int Purpose_ENCRYPTION_get();
  public final static native int Purpose_DECRYPTION_get();
  public final static native int Purpose_WITHDRAW_get();
  public final static native int Purpose_SYSTEM_get();
  public final static native int Purpose_VOTING_get();
  public final static native int SecurityLevel_MASTER_get();
  public final static native int SecurityLevel_CRITICAL_get();
  public final static native int SecurityLevel_HIGH_get();
  public final static native int SecurityLevel_MEDIUM_get();
  public final static native void RootStruct_name_set(long jarg1, RootStruct jarg1_, String jarg2);
  public final static native String RootStruct_name_get(long jarg1, RootStruct jarg1_);
  public final static native void delete_RootStruct(long jarg1);
  public final static native long new_TimestampMillis__SWIG_0();
  public final static native long new_TimestampMillis__SWIG_1(long jarg1);
  public final static native void delete_TimestampMillis(long jarg1);
  public final static native long TimestampMillis_toLong(long jarg1, TimestampMillis jarg1_);
  public final static native boolean TimestampMillis_objectEquals(long jarg1, TimestampMillis jarg1_, long jarg2, TimestampMillis jarg2_);
  public final static native long new_KeyID(int jarg1);
  public final static native void delete_KeyID(long jarg1);
  public final static native int KeyID_toInt(long jarg1, KeyID jarg1_);
  public final static native void IdentifierBytes32__0_set(long jarg1, IdentifierBytes32 jarg1_, byte[] jarg2);
  public final static native byte[] IdentifierBytes32__0_get(long jarg1, IdentifierBytes32 jarg1_);
  public final static native long new_IdentifierBytes32(byte[] jarg1);
  public final static native void delete_IdentifierBytes32(long jarg1);
  public final static native void Identifier__0_set(long jarg1, Identifier jarg1_, long jarg2, IdentifierBytes32 jarg2_);
  public final static native long Identifier__0_get(long jarg1, Identifier jarg1_);
  public final static native long new_Identifier(byte[] jarg1);
  public final static native void delete_Identifier(long jarg1);
  public final static native void ContractBounds_SingleContract_Body_id_set(long jarg1, ContractBounds_SingleContract_Body jarg1_, long jarg2, Identifier jarg2_);
  public final static native long ContractBounds_SingleContract_Body_id_get(long jarg1, ContractBounds_SingleContract_Body jarg1_);
  public final static native void delete_ContractBounds_SingleContract_Body(long jarg1);
  public final static native void ContractBounds_SingleContractDocumentType_Body_id_set(long jarg1, ContractBounds_SingleContractDocumentType_Body jarg1_, long jarg2, Identifier jarg2_);
  public final static native long ContractBounds_SingleContractDocumentType_Body_id_get(long jarg1, ContractBounds_SingleContractDocumentType_Body jarg1_);
  public final static native void ContractBounds_SingleContractDocumentType_Body_document_type_name_set(long jarg1, ContractBounds_SingleContractDocumentType_Body jarg1_, String jarg2);
  public final static native String ContractBounds_SingleContractDocumentType_Body_document_type_name_get(long jarg1, ContractBounds_SingleContractDocumentType_Body jarg1_);
  public final static native void delete_ContractBounds_SingleContractDocumentType_Body(long jarg1);
  public final static native void ContractBounds_tag_set(long jarg1, ContractBounds jarg1_, int jarg2);
  public final static native int ContractBounds_tag_get(long jarg1, ContractBounds jarg1_);
  public final static native void ContractBounds_single_contract_set(long jarg1, ContractBounds jarg1_, long jarg2, ContractBounds_SingleContract_Body jarg2_);
  public final static native long ContractBounds_single_contract_get(long jarg1, ContractBounds jarg1_);
  public final static native void ContractBounds_single_contract_document_type_set(long jarg1, ContractBounds jarg1_, long jarg2, ContractBounds_SingleContractDocumentType_Body jarg2_);
  public final static native long ContractBounds_single_contract_document_type_get(long jarg1, ContractBounds jarg1_);
  public final static native long new_ContractBounds__SWIG_0(long jarg1, Identifier jarg1_);
  public final static native long new_ContractBounds__SWIG_1(long jarg1, Identifier jarg1_, String jarg2);
  public final static native void delete_ContractBounds(long jarg1);
  public final static native void BinaryData__0_set(long jarg1, BinaryData jarg1_, byte[] jarg2);
  public final static native byte[] BinaryData__0_get(long jarg1, BinaryData jarg1_);
  public final static native long new_BinaryData(byte[] jarg1);
  public final static native void delete_BinaryData(long jarg1);
  public final static native void IdentityPublicKeyV0_id_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, KeyID jarg2_);
  public final static native long IdentityPublicKeyV0_id_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_contract_bounds_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, ContractBounds jarg2_);
  public final static native long IdentityPublicKeyV0_contract_bounds_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_read_only_set(long jarg1, IdentityPublicKeyV0 jarg1_, boolean jarg2);
  public final static native boolean IdentityPublicKeyV0_read_only_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_data_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, BinaryData jarg2_);
  public final static native long IdentityPublicKeyV0_data_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_disabled_at_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, TimestampMillis jarg2_);
  public final static native long IdentityPublicKeyV0_disabled_at_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long new_IdentityPublicKeyV0(long jarg1, KeyID jarg1_, int jarg2, int jarg3, long jarg4, ContractBounds jarg4_, int jarg5, boolean jarg6, long jarg7, BinaryData jarg7_, long jarg8, TimestampMillis jarg8_);
  public final static native void delete_IdentityPublicKeyV0(long jarg1);
  public final static native int IdentityPublicKeyV0_getKeyType(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native int IdentityPublicKeyV0_getPurpose(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native int IdentityPublicKeyV0_getSecurityLevel(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKey_tag_set(long jarg1, IdentityPublicKey jarg1_, int jarg2);
  public final static native int IdentityPublicKey_tag_get(long jarg1, IdentityPublicKey jarg1_);
  public final static native void IdentityPublicKey_v0_set(long jarg1, IdentityPublicKey jarg1_, long jarg2, IdentityPublicKeyV0 jarg2_);
  public final static native long IdentityPublicKey_v0_get(long jarg1, IdentityPublicKey jarg1_);
  public final static native void delete_IdentityPublicKey(long jarg1);
  public final static native void std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_count_set(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_, long jarg2);
  public final static native long std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_count_get(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_);
  public final static native void std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_keys_set(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_, long jarg2);
  public final static native long std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_keys_get(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_);
  public final static native void std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_values_set(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_, long jarg2);
  public final static native long std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_values_get(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_);
  public final static native void delete_std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey(long jarg1);
  public final static native long new_Revision__SWIG_0();
  public final static native long new_Revision__SWIG_1(long jarg1);
  public final static native void delete_Revision(long jarg1);
  public final static native long Revision_toLong(long jarg1, Revision jarg1_);
  public final static native void IdentityV0_id_set(long jarg1, IdentityV0 jarg1_, long jarg2, Identifier jarg2_);
  public final static native long IdentityV0_id_get(long jarg1, IdentityV0 jarg1_);
  public final static native void IdentityV0_revision_set(long jarg1, IdentityV0 jarg1_, long jarg2, Revision jarg2_);
  public final static native long IdentityV0_revision_get(long jarg1, IdentityV0 jarg1_);
  public final static native void delete_IdentityV0(long jarg1);
  public final static native int IdentityV0_getPublicKeyCount(long jarg1, IdentityV0 jarg1_);
  public final static native long IdentityV0_getPublicKey(long jarg1, IdentityV0 jarg1_, long jarg2);
  public final static native long IdentityV0_getPublicKeyById(long jarg1, IdentityV0 jarg1_, long jarg2);
  public final static native long IdentityV0_getBalance(long jarg1, IdentityV0 jarg1_);
  public final static native void Identity_tag_set(long jarg1, Identity jarg1_, int jarg2);
  public final static native int Identity_tag_get(long jarg1, Identity jarg1_);
  public final static native void Identity_v0_set(long jarg1, Identity jarg1_, long jarg2, IdentityV0 jarg2_);
  public final static native long Identity_v0_get(long jarg1, Identity jarg1_);
  public final static native long new_Identity();
  public final static native void delete_Identity(long jarg1);
  public final static native void DataContractNotPresentError_data_contract_id_set(long jarg1, DataContractNotPresentError jarg1_, long jarg2, Identifier jarg2_);
  public final static native long DataContractNotPresentError_data_contract_id_get(long jarg1, DataContractNotPresentError jarg1_);
  public final static native void delete_DataContractNotPresentError(long jarg1);
  public final static native void ProtocolError_StringDecodeError2_Body__0_set(long jarg1, ProtocolError_StringDecodeError2_Body jarg1_, String jarg2);
  public final static native String ProtocolError_StringDecodeError2_Body__0_get(long jarg1, ProtocolError_StringDecodeError2_Body jarg1_);
  public final static native void ProtocolError_StringDecodeError2_Body__1_set(long jarg1, ProtocolError_StringDecodeError2_Body jarg1_, long jarg2);
  public final static native long ProtocolError_StringDecodeError2_Body__1_get(long jarg1, ProtocolError_StringDecodeError2_Body jarg1_);
  public final static native void delete_ProtocolError_StringDecodeError2_Body(long jarg1);
  public final static native void ProtocolError_MaxEncodedBytesReachedError_Body_max_size_kbytes_set(long jarg1, ProtocolError_MaxEncodedBytesReachedError_Body jarg1_, long jarg2);
  public final static native long ProtocolError_MaxEncodedBytesReachedError_Body_max_size_kbytes_get(long jarg1, ProtocolError_MaxEncodedBytesReachedError_Body jarg1_);
  public final static native void ProtocolError_MaxEncodedBytesReachedError_Body_size_hit_set(long jarg1, ProtocolError_MaxEncodedBytesReachedError_Body jarg1_, long jarg2);
  public final static native long ProtocolError_MaxEncodedBytesReachedError_Body_size_hit_get(long jarg1, ProtocolError_MaxEncodedBytesReachedError_Body jarg1_);
  public final static native void delete_ProtocolError_MaxEncodedBytesReachedError_Body(long jarg1);
  public final static native void ProtocolError_UnknownVersionMismatch_Body_method_set(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_, String jarg2);
  public final static native String ProtocolError_UnknownVersionMismatch_Body_method_get(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_);
  public final static native void ProtocolError_UnknownVersionMismatch_Body_known_versions_set(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_, long jarg2);
  public final static native long ProtocolError_UnknownVersionMismatch_Body_known_versions_get(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_);
  public final static native void ProtocolError_UnknownVersionMismatch_Body_received_set(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_, long jarg2);
  public final static native long ProtocolError_UnknownVersionMismatch_Body_received_get(long jarg1, ProtocolError_UnknownVersionMismatch_Body jarg1_);
  public final static native void delete_ProtocolError_UnknownVersionMismatch_Body(long jarg1);
  public final static native void ProtocolError_tag_set(long jarg1, ProtocolError jarg1_, int jarg2);
  public final static native int ProtocolError_tag_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_identifier_error_set(long jarg1, ProtocolError jarg1_, String jarg2);
  public final static native String ProtocolError_identifier_error_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_string_decode_error_set(long jarg1, ProtocolError jarg1_, String jarg2);
  public final static native String ProtocolError_string_decode_error_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_string_decode_error2_set(long jarg1, ProtocolError jarg1_, long jarg2, ProtocolError_StringDecodeError2_Body jarg2_);
  public final static native long ProtocolError_string_decode_error2_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_max_encoded_bytes_reached_error_set(long jarg1, ProtocolError jarg1_, long jarg2, ProtocolError_MaxEncodedBytesReachedError_Body jarg2_);
  public final static native long ProtocolError_max_encoded_bytes_reached_error_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_encoding_error_set(long jarg1, ProtocolError jarg1_, String jarg2);
  public final static native String ProtocolError_encoding_error_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_encoding_error2_set(long jarg1, ProtocolError jarg1_, String jarg2);
  public final static native String ProtocolError_encoding_error2_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_data_contract_not_present_error_set(long jarg1, ProtocolError jarg1_, long jarg2, DataContractNotPresentError jarg2_);
  public final static native long ProtocolError_data_contract_not_present_error_get(long jarg1, ProtocolError jarg1_);
  public final static native void ProtocolError_unknown_version_mismatch_set(long jarg1, ProtocolError jarg1_, long jarg2, ProtocolError_UnknownVersionMismatch_Body jarg2_);
  public final static native long ProtocolError_unknown_version_mismatch_get(long jarg1, ProtocolError jarg1_);
  public final static native void delete_ProtocolError(long jarg1);
  public final static native void Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_ok_set(long jarg1, Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError jarg1_, long jarg2, Identity jarg2_);
  public final static native long Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_ok_get(long jarg1, Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError jarg1_);
  public final static native void Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_error_set(long jarg1, Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError jarg1_, long jarg2, ProtocolError jarg2_);
  public final static native long Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_error_get(long jarg1, Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError jarg1_);
  public final static native void delete_Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError(long jarg1);
  public final static native void FeatureVersionBounds_min_version_set(long jarg1, FeatureVersionBounds jarg1_, long jarg2);
  public final static native long FeatureVersionBounds_min_version_get(long jarg1, FeatureVersionBounds jarg1_);
  public final static native void FeatureVersionBounds_max_version_set(long jarg1, FeatureVersionBounds jarg1_, long jarg2);
  public final static native long FeatureVersionBounds_max_version_get(long jarg1, FeatureVersionBounds jarg1_);
  public final static native void FeatureVersionBounds_default_current_version_set(long jarg1, FeatureVersionBounds jarg1_, long jarg2);
  public final static native long FeatureVersionBounds_default_current_version_get(long jarg1, FeatureVersionBounds jarg1_);
  public final static native void delete_FeatureVersionBounds(long jarg1);
  public final static native void PlatformVersion_protocol_version_set(long jarg1, PlatformVersion jarg1_, long jarg2);
  public final static native long PlatformVersion_protocol_version_get(long jarg1, PlatformVersion jarg1_);
  public final static native void PlatformVersion_identity_set(long jarg1, PlatformVersion jarg1_, long jarg2, FeatureVersionBounds jarg2_);
  public final static native long PlatformVersion_identity_get(long jarg1, PlatformVersion jarg1_);
  public final static native void PlatformVersion_proofs_set(long jarg1, PlatformVersion jarg1_, long jarg2, FeatureVersionBounds jarg2_);
  public final static native long PlatformVersion_proofs_get(long jarg1, PlatformVersion jarg1_);
  public final static native void delete_PlatformVersion(long jarg1);
  public final static native void OptionalFeatureVersion__0_set(long jarg1, OptionalFeatureVersion jarg1_, int jarg2);
  public final static native int OptionalFeatureVersion__0_get(long jarg1, OptionalFeatureVersion jarg1_);
  public final static native void delete_OptionalFeatureVersion(long jarg1);
  public final static native void InnerStruct_first_set(long jarg1, InnerStruct jarg1_, java.math.BigInteger jarg2);
  public final static native java.math.BigInteger InnerStruct_first_get(long jarg1, InnerStruct jarg1_);
  public final static native void InnerStruct_second_set(long jarg1, InnerStruct jarg1_, java.math.BigInteger jarg2);
  public final static native java.math.BigInteger InnerStruct_second_get(long jarg1, InnerStruct jarg1_);
  public final static native long new_InnerStruct(int jarg1, int jarg2);
  public final static native void delete_InnerStruct(long jarg1);
  public final static native void OuterStruct_first_set(long jarg1, OuterStruct jarg1_, long jarg2, InnerStruct jarg2_);
  public final static native long OuterStruct_first_get(long jarg1, OuterStruct jarg1_);
  public final static native void OuterStruct_second_set(long jarg1, OuterStruct jarg1_, long jarg2, InnerStruct jarg2_);
  public final static native long OuterStruct_second_get(long jarg1, OuterStruct jarg1_);
  public final static native long new_OuterStruct__SWIG_0(long jarg1, InnerStruct jarg1_, long jarg2, InnerStruct jarg2_);
  public final static native long new_OuterStruct__SWIG_1(int jarg1, int jarg2, int jarg3, int jarg4);
  public final static native void delete_OuterStruct(long jarg1);
  public final static native void VecU8Holder_first_set(long jarg1, VecU8Holder jarg1_, byte[] jarg2);
  public final static native byte[] VecU8Holder_first_get(long jarg1, VecU8Holder jarg1_);
  public final static native long new_VecU8Holder(byte[] jarg1);
  public final static native void delete_VecU8Holder(long jarg1);
  public final static native long rootStructCtor(String jarg1);
  public final static native void rootStructDestroy(long jarg1, RootStruct jarg1_);
  public final static native long timestampMillisCtor(java.math.BigInteger jarg1);
  public final static native void timestampMillisDestroy(long jarg1, TimestampMillis jarg1_);
  public final static native long identityPublicKeyV0Ctor(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void identityPublicKeyDestroy(long jarg1, IdentityPublicKey jarg1_);
  public final static native long createBasicIdentity(byte[] jarg1, long jarg2, PlatformVersion jarg2_);
  public final static native long createBasicIdentityV0(byte[] jarg1);
  public final static native long identityV0Ctor__SWIG_0(long jarg1, IdentityV0 jarg1_);
  public final static native void identityDestroy(long jarg1, Identity jarg1_);
  public final static native long getIdentityContractBounds(long jarg1, Identifier jarg1_, long jarg2, Identifier jarg2_);
  public final static native long getAnIdentity();
  public final static native long getIdentity2(long jarg1, Identifier jarg1_);
  public final static native void identityPublicKeyV0Destroy(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void contractBoundsDestroy(long jarg1, ContractBounds jarg1_);
  public final static native long identityV0Ctor__SWIG_1(long jarg1, Identifier jarg1_, long jarg2, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg2_, java.math.BigInteger jarg3, long jarg4, Revision jarg4_);
  public final static native void identityV0Destroy(long jarg1, IdentityV0 jarg1_);
  public final static native long randomKey(long jarg1, KeyID jarg1_);
  public final static native long randomKeyArgs(long jarg1, KeyID jarg1_, long jarg2, Identifier jarg2_, long jarg3, TimestampMillis jarg3_);
  public final static native long revisionCtor(java.math.BigInteger jarg1);
  public final static native void revisionDestroy(long jarg1, Revision jarg1_);
  public final static native long keyIDCtor(long jarg1);
  public final static native void keyIDDestroy(long jarg1, KeyID jarg1_);
  public final static native long featureVersionCtor(int jarg1);
  public final static native void featureVersionDestroy(long jarg1);
  public final static native long platformVersionCtor(long jarg1, long jarg2, FeatureVersionBounds jarg2_, long jarg3, FeatureVersionBounds jarg3_);
  public final static native void platformVersionDestroy(long jarg1, PlatformVersion jarg1_);
  public final static native long identifierCtor(long jarg1, IdentifierBytes32 jarg1_);
  public final static native void identifierDestroy(long jarg1, Identifier jarg1_);
  public final static native long protocolErrorIdentifierErrorCtor(String jarg1);
  public final static native long protocolErrorStringDecodeErrorCtor(String jarg1);
  public final static native long protocolErrorStringDecodeError2Ctor(String jarg1, long jarg2);
  public final static native long protocolErrorEmptyPublicKeyDataErrorCtor();
  public final static native long protocolErrorMaxEncodedBytesReachedErrorCtor(long jarg1, long jarg2);
  public final static native long protocolErrorEncodingErrorCtor(String jarg1);
  public final static native long protocolErrorEncodingError2Ctor(String jarg1);
  public final static native long protocolErrorDataContractNotPresentErrorCtor(long jarg1, DataContractNotPresentError jarg1_);
  public final static native long protocolErrorUnknownVersionMismatchCtor(String jarg1, long jarg2, long jarg3);
  public final static native void protocolErrorDestroy(long jarg1, ProtocolError jarg1_);
  public final static native long optionalFeatureVersionCtor(int jarg1);
  public final static native void optionalFeatureVersionDestroy(long jarg1, OptionalFeatureVersion jarg1_);
  public final static native long binaryDataCtor(byte[] jarg1);
  public final static native void binaryDataDestroy(long jarg1, BinaryData jarg1_);
  public final static native long identifierBytes32Ctor(byte[] jarg1);
  public final static native void identifierBytes32Destroy(long jarg1, IdentifierBytes32 jarg1_);
  public final static native long dataContractNotPresentErrorCtor(long jarg1, Identifier jarg1_);
  public final static native void dataContractNotPresentErrorDestroy(long jarg1, DataContractNotPresentError jarg1_);
  public final static native long featureVersionBoundsCtor(long jarg1, long jarg2, long jarg3);
  public final static native void featureVersionBoundsDestroy(long jarg1, FeatureVersionBounds jarg1_);
  public final static native long outerStructCtor(long jarg1, InnerStruct jarg1_, long jarg2, InnerStruct jarg2_);
  public final static native void outerStructDestroy(long jarg1, OuterStruct jarg1_);
  public final static native long innerStructCtor(java.math.BigInteger jarg1, java.math.BigInteger jarg2);
  public final static native void innerStructDestroy(long jarg1, InnerStruct jarg1_);
  public final static native long createOuter(long jarg1, InnerStruct jarg1_, long jarg2, InnerStruct jarg2_);
  public final static native long vecU8HolderCtor(byte[] jarg1);
  public final static native void vecU8HolderDestroy(long jarg1, VecU8Holder jarg1_);
  public final static native byte[] vecU8Ctor(long jarg1, long jarg2);
  public final static native void vecU8Destroy(byte[] jarg1);
  public final static native long resultOkCrateIdentityIdentityIdentityErrCrateNestedProtocolErrorCtor(long jarg1, Identity jarg1_, long jarg2, ProtocolError jarg2_);
  public final static native void resultOkCrateIdentityIdentityIdentityErrCrateNestedProtocolErrorDestroy(long jarg1, Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError jarg1_);
  public final static native long vecCrateNestedFeatureVersionCtor(long jarg1, long jarg2);
  public final static native void vecCrateNestedFeatureVersionDestroy(long jarg1);
  public final static native long stdCollectionsMapKeysCrateIdentityIdentityKeyIDValuesCrateIdentityIdentityIdentityPublicKeyCtor(long jarg1, long jarg2, long jarg3);
  public final static native void stdCollectionsMapKeysCrateIdentityIdentityKeyIDValuesCrateIdentityIdentityIdentityPublicKeyDestroy(long jarg1, std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey jarg1_);
  public final static native long MemoryFactory_getInstance();
  public final static native long new_MemoryFactory();
  public final static native void delete_MemoryFactory(long jarg1);
  public final static native long MemoryFactory_size(long jarg1, MemoryFactory jarg1_);
  public final static native long MemoryFactory_alloc__SWIG_0(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native String MemoryFactory_clone(long jarg1, MemoryFactory jarg1_, String jarg2);
  public final static native long MemoryFactory_alloc__SWIG_1(long jarg1, MemoryFactory jarg1_, long jarg2, long jarg3);
  public final static native void MemoryFactory_destroy__SWIG_0(long jarg1, MemoryFactory jarg1_, long jarg2, long jarg3);
  public final static native void MemoryFactory_destroy__SWIG_1(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native void MemoryFactory_destroyItem(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native void memoryFactory_set(long jarg1, MemoryFactory jarg1_);
  public final static native long memoryFactory_get();
  public final static native long identifierClone(long jarg1, Identifier jarg1_);
  public final static native long singleContract(long jarg1, Identifier jarg1_);
  public final static native long singleContractDocumentType(long jarg1, Identifier jarg1_, String jarg2);
}
