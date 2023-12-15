/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class example {
  public static RootStruct rootStructCtor(String name) {
    long cPtr = exampleJNI.rootStructCtor(name);
    return (cPtr == 0) ? null : new RootStruct(cPtr, false);
  }

  public static void rootStructDestroy(RootStruct ffi) {
    exampleJNI.rootStructDestroy(RootStruct.getCPtr(ffi), ffi);
  }

  public static SWIGTYPE_p_DevnetType devnetTypeJackDanielsCtor() {
    long cPtr = exampleJNI.devnetTypeJackDanielsCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_DevnetType(cPtr, false);
  }

  public static SWIGTYPE_p_DevnetType devnetTypeDevnet333Ctor() {
    long cPtr = exampleJNI.devnetTypeDevnet333Ctor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_DevnetType(cPtr, false);
  }

  public static SWIGTYPE_p_DevnetType devnetTypeChachaCtor() {
    long cPtr = exampleJNI.devnetTypeChachaCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_DevnetType(cPtr, false);
  }

  public static SWIGTYPE_p_DevnetType devnetTypeMojitoCtor() {
    long cPtr = exampleJNI.devnetTypeMojitoCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_DevnetType(cPtr, false);
  }

  public static SWIGTYPE_p_DevnetType devnetTypeWhiteRussianCtor() {
    long cPtr = exampleJNI.devnetTypeWhiteRussianCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_DevnetType(cPtr, false);
  }

  public static void devnetTypeDestroy(SWIGTYPE_p_DevnetType ffi) {
    exampleJNI.devnetTypeDestroy(SWIGTYPE_p_DevnetType.getCPtr(ffi));
  }

  public static IHaveChainSettings_TraitObject devnetTypeAsIHaveChainSettingsTraitObject(SWIGTYPE_p_DevnetType obj) {
    return new IHaveChainSettings_TraitObject(exampleJNI.devnetTypeAsIHaveChainSettingsTraitObject(SWIGTYPE_p_DevnetType.getCPtr(obj)), true);
  }

  public static void devnetTypeAsIHaveChainSettingsTraitObjectDestroy(IHaveChainSettings_TraitObject obj) {
    exampleJNI.devnetTypeAsIHaveChainSettingsTraitObjectDestroy(IHaveChainSettings_TraitObject.getCPtr(obj), obj);
  }

  public static ChainType ffiGetChainSettings() {
    long cPtr = exampleJNI.ffiGetChainSettings();
    return (cPtr == 0) ? null : new ChainType(cPtr, false);
  }

  public static ChainType chainTypeMainNetCtor() {
    long cPtr = exampleJNI.chainTypeMainNetCtor();
    return (cPtr == 0) ? null : new ChainType(cPtr, false);
  }

  public static ChainType chainTypeTestNetCtor() {
    long cPtr = exampleJNI.chainTypeTestNetCtor();
    return (cPtr == 0) ? null : new ChainType(cPtr, false);
  }

  public static ChainType chainTypeDevNetCtor(SWIGTYPE_p_DevnetType o_o_0) {
    long cPtr = exampleJNI.chainTypeDevNetCtor(SWIGTYPE_p_DevnetType.getCPtr(o_o_0));
    return (cPtr == 0) ? null : new ChainType(cPtr, false);
  }

  public static void chainTypeDestroy(ChainType ffi) {
    exampleJNI.chainTypeDestroy(ChainType.getCPtr(ffi), ffi);
  }

  public static IHaveChainSettings_TraitObject chainTypeAsIHaveChainSettingsTraitObject(ChainType obj) {
    return new IHaveChainSettings_TraitObject(exampleJNI.chainTypeAsIHaveChainSettingsTraitObject(ChainType.getCPtr(obj), obj), true);
  }

  public static void chainTypeAsIHaveChainSettingsTraitObjectDestroy(IHaveChainSettings_TraitObject obj) {
    exampleJNI.chainTypeAsIHaveChainSettingsTraitObjectDestroy(IHaveChainSettings_TraitObject.getCPtr(obj), obj);
  }

  public static ContractBounds contractBoundsSingleContractCtor(Identifier o_o_0) {
    long cPtr = exampleJNI.contractBoundsSingleContractCtor(Identifier.getCPtr(o_o_0), o_o_0);
    return (cPtr == 0) ? null : new ContractBounds(cPtr, false);
  }

  public static ContractBounds contractBoundsSingleContractDocumentTypeCtor(Identifier o_o_0, String o_o_1) {
    long cPtr = exampleJNI.contractBoundsSingleContractDocumentTypeCtor(Identifier.getCPtr(o_o_0), o_o_0, o_o_1);
    return (cPtr == 0) ? null : new ContractBounds(cPtr, false);
  }

  public static void contractBoundsDestroy(ContractBounds ffi) {
    exampleJNI.contractBoundsDestroy(ContractBounds.getCPtr(ffi), ffi);
  }

  public static UsedKeyMatrix usedKeyMatrixCtor(Vec_bool o_0) {
    long cPtr = exampleJNI.usedKeyMatrixCtor(Vec_bool.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new UsedKeyMatrix(cPtr, false);
  }

  public static void usedKeyMatrixDestroy(UsedKeyMatrix ffi) {
    exampleJNI.usedKeyMatrixDestroy(UsedKeyMatrix.getCPtr(ffi), ffi);
  }

  public static MyIdentityFactory myIdentityFactoryCtor(long first) {
    long cPtr = exampleJNI.myIdentityFactoryCtor(first);
    return (cPtr == 0) ? null : new MyIdentityFactory(cPtr, false);
  }

  public static void myIdentityFactoryDestroy(MyIdentityFactory ffi) {
    exampleJNI.myIdentityFactoryDestroy(MyIdentityFactory.getCPtr(ffi), ffi);
  }

  public static IdentityFactory_TraitObject myIdentityFactoryAsIdentityFactoryTraitObject(MyIdentityFactory obj) {
    return new IdentityFactory_TraitObject(exampleJNI.myIdentityFactoryAsIdentityFactoryTraitObject(MyIdentityFactory.getCPtr(obj), obj), true);
  }

  public static void myIdentityFactoryAsIdentityFactoryTraitObjectDestroy(IdentityFactory_TraitObject obj) {
    exampleJNI.myIdentityFactoryAsIdentityFactoryTraitObjectDestroy(IdentityFactory_TraitObject.getCPtr(obj), obj);
  }

  public static HashID hashIDCtor(byte[] o_0) {
    long cPtr = exampleJNI.hashIDCtor(o_0);
    return (cPtr == 0) ? null : new HashID(cPtr, false);
  }

  public static void hashIDDestroy(HashID ffi) {
    exampleJNI.hashIDDestroy(HashID.getCPtr(ffi), ffi);
  }

  public static IdentityPublicKey identityPublicKeyV0Ctor(IdentityPublicKeyV0 o_o_0) {
    long cPtr = exampleJNI.identityPublicKeyV0Ctor__SWIG_0(IdentityPublicKeyV0.getCPtr(o_o_0), o_o_0);
    return (cPtr == 0) ? null : new IdentityPublicKey(cPtr, false);
  }

  public static void identityPublicKeyDestroy(IdentityPublicKey ffi) {
    exampleJNI.identityPublicKeyDestroy(IdentityPublicKey.getCPtr(ffi), ffi);
  }

  public static ArrayOfArraysOfHashes arrayOfArraysOfHashesCtor(Vec_Vec_crate_nested_HashID o_0) {
    long cPtr = exampleJNI.arrayOfArraysOfHashesCtor(Vec_Vec_crate_nested_HashID.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new ArrayOfArraysOfHashes(cPtr, false);
  }

  public static void arrayOfArraysOfHashesDestroy(ArrayOfArraysOfHashes ffi) {
    exampleJNI.arrayOfArraysOfHashesDestroy(ArrayOfArraysOfHashes.getCPtr(ffi), ffi);
  }

  public static IdentifierBytes32 identifierBytes32Ctor(byte[] o_0) {
    long cPtr = exampleJNI.identifierBytes32Ctor(o_0);
    return (cPtr == 0) ? null : new IdentifierBytes32(cPtr, false);
  }

  public static void identifierBytes32Destroy(IdentifierBytes32 ffi) {
    exampleJNI.identifierBytes32Destroy(IdentifierBytes32.getCPtr(ffi), ffi);
  }

  public static Hash160 hash160Ctor(SWIGTYPE_p_a_20__unsigned_char o_0) {
    long cPtr = exampleJNI.hash160Ctor(SWIGTYPE_p_a_20__unsigned_char.getCPtr(o_0));
    return (cPtr == 0) ? null : new Hash160(cPtr, false);
  }

  public static void hash160Destroy(Hash160 ffi) {
    exampleJNI.hash160Destroy(Hash160.getCPtr(ffi), ffi);
  }

  public static Identity identityV0Ctor(IdentityV0 o_o_0) {
    long cPtr = exampleJNI.identityV0Ctor__SWIG_0(IdentityV0.getCPtr(o_o_0), o_o_0);
    return (cPtr == 0) ? null : new Identity(cPtr, false);
  }

  public static void identityDestroy(Identity ffi) {
    exampleJNI.identityDestroy(Identity.getCPtr(ffi), ffi);
  }

  public static KeyID keyIDCtor(long o_0) {
    long cPtr = exampleJNI.keyIDCtor(o_0);
    return (cPtr == 0) ? null : new KeyID(cPtr, false);
  }

  public static void keyIDDestroy(KeyID ffi) {
    exampleJNI.keyIDDestroy(KeyID.getCPtr(ffi), ffi);
  }

  public static TimestampMillis timestampMillisCtor(java.math.BigInteger o_0) {
    long cPtr = exampleJNI.timestampMillisCtor(o_0);
    return (cPtr == 0) ? null : new TimestampMillis(cPtr, false);
  }

  public static void timestampMillisDestroy(TimestampMillis ffi) {
    exampleJNI.timestampMillisDestroy(TimestampMillis.getCPtr(ffi), ffi);
  }

  public static Identity ffiGetAnIdentity() {
    long cPtr = exampleJNI.ffiGetAnIdentity();
    return (cPtr == 0) ? null : new Identity(cPtr, false);
  }

  public static ProtocolError protocolErrorIdentifierErrorCtor(String o_o_0) {
    long cPtr = exampleJNI.protocolErrorIdentifierErrorCtor(o_o_0);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorStringDecodeErrorCtor(String o_o_0) {
    long cPtr = exampleJNI.protocolErrorStringDecodeErrorCtor(o_o_0);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorStringDecodeError2Ctor(String o_o_0, long o_o_1) {
    long cPtr = exampleJNI.protocolErrorStringDecodeError2Ctor(o_o_0, o_o_1);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorEmptyPublicKeyDataErrorCtor() {
    long cPtr = exampleJNI.protocolErrorEmptyPublicKeyDataErrorCtor();
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorMaxEncodedBytesReachedErrorCtor(long max_size_kbytes, long size_hit) {
    long cPtr = exampleJNI.protocolErrorMaxEncodedBytesReachedErrorCtor(max_size_kbytes, size_hit);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorEncodingErrorCtor(String o_o_0) {
    long cPtr = exampleJNI.protocolErrorEncodingErrorCtor(o_o_0);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorEncodingError2Ctor(String o_o_0) {
    long cPtr = exampleJNI.protocolErrorEncodingError2Ctor(o_o_0);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static ProtocolError protocolErrorUnknownVersionMismatchCtor() {
    long cPtr = exampleJNI.protocolErrorUnknownVersionMismatchCtor();
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

  public static void protocolErrorDestroy(ProtocolError ffi) {
    exampleJNI.protocolErrorDestroy(ProtocolError.getCPtr(ffi), ffi);
  }

  public static IdentityPublicKeyV0 identityPublicKeyV0Ctor(KeyID id, SWIGTYPE_p_Purpose purpose, SWIGTYPE_p_SecurityLevel security_level, ContractBounds contract_bounds, SWIGTYPE_p_KeyType key_type, boolean read_only, BinaryData data, TimestampMillis disabled_at) {
    long cPtr = exampleJNI.identityPublicKeyV0Ctor__SWIG_1(KeyID.getCPtr(id), id, SWIGTYPE_p_Purpose.getCPtr(purpose), SWIGTYPE_p_SecurityLevel.getCPtr(security_level), ContractBounds.getCPtr(contract_bounds), contract_bounds, SWIGTYPE_p_KeyType.getCPtr(key_type), read_only, BinaryData.getCPtr(data), data, TimestampMillis.getCPtr(disabled_at), disabled_at);
    return (cPtr == 0) ? null : new IdentityPublicKeyV0(cPtr, false);
  }

  public static void identityPublicKeyV0Destroy(IdentityPublicKeyV0 ffi) {
    exampleJNI.identityPublicKeyV0Destroy(IdentityPublicKeyV0.getCPtr(ffi), ffi);
  }

  public static Revision revisionCtor(java.math.BigInteger o_0) {
    long cPtr = exampleJNI.revisionCtor(o_0);
    return (cPtr == 0) ? null : new Revision(cPtr, false);
  }

  public static void revisionDestroy(Revision ffi) {
    exampleJNI.revisionDestroy(Revision.getCPtr(ffi), ffi);
  }

  public static SWIGTYPE_p_SecurityLevel securityLevelMASTERCtor() {
    long cPtr = exampleJNI.securityLevelMASTERCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_SecurityLevel(cPtr, false);
  }

  public static SWIGTYPE_p_SecurityLevel securityLevelCRITICALCtor() {
    long cPtr = exampleJNI.securityLevelCRITICALCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_SecurityLevel(cPtr, false);
  }

  public static SWIGTYPE_p_SecurityLevel securityLevelHIGHCtor() {
    long cPtr = exampleJNI.securityLevelHIGHCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_SecurityLevel(cPtr, false);
  }

  public static SWIGTYPE_p_SecurityLevel securityLevelMEDIUMCtor() {
    long cPtr = exampleJNI.securityLevelMEDIUMCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_SecurityLevel(cPtr, false);
  }

  public static void securityLevelDestroy(SWIGTYPE_p_SecurityLevel ffi) {
    exampleJNI.securityLevelDestroy(SWIGTYPE_p_SecurityLevel.getCPtr(ffi));
  }

  public static TestEnum testEnumVariant1Ctor(String o_o_0) {
    long cPtr = exampleJNI.testEnumVariant1Ctor(o_o_0);
    return (cPtr == 0) ? null : new TestEnum(cPtr, false);
  }

  public static TestEnum testEnumVariant2Ctor() {
    long cPtr = exampleJNI.testEnumVariant2Ctor();
    return (cPtr == 0) ? null : new TestEnum(cPtr, false);
  }

  public static TestEnum testEnumVariant3Ctor(HashID o_o_0, long o_o_1) {
    long cPtr = exampleJNI.testEnumVariant3Ctor(HashID.getCPtr(o_o_0), o_o_0, o_o_1);
    return (cPtr == 0) ? null : new TestEnum(cPtr, false);
  }

  public static TestEnum testEnumVariant4Ctor(HashID o_o_0, long o_o_1, String o_o_2) {
    long cPtr = exampleJNI.testEnumVariant4Ctor(HashID.getCPtr(o_o_0), o_o_0, o_o_1, o_o_2);
    return (cPtr == 0) ? null : new TestEnum(cPtr, false);
  }

  public static TestEnum testEnumVariant5Ctor(std_collections_Map_keys_String_values_crate_nested_HashID o_o_0, long o_o_1, String o_o_2) {
    long cPtr = exampleJNI.testEnumVariant5Ctor(std_collections_Map_keys_String_values_crate_nested_HashID.getCPtr(o_o_0), o_o_0, o_o_1, o_o_2);
    return (cPtr == 0) ? null : new TestEnum(cPtr, false);
  }

  public static void testEnumDestroy(TestEnum ffi) {
    exampleJNI.testEnumDestroy(TestEnum.getCPtr(ffi), ffi);
  }

  public static HashID ffiFindHashByU32(long key, std_collections_Map_keys_u32_values_crate_nested_HashID map) {
    long cPtr = exampleJNI.ffiFindHashByU32(key, std_collections_Map_keys_u32_values_crate_nested_HashID.getCPtr(map), map);
    return (cPtr == 0) ? null : new HashID(cPtr, false);
  }

  public static UnnamedPair unnamedPairCtor(byte[] o_0, long o_1) {
    long cPtr = exampleJNI.unnamedPairCtor(o_0, o_1);
    return (cPtr == 0) ? null : new UnnamedPair(cPtr, false);
  }

  public static void unnamedPairDestroy(UnnamedPair ffi) {
    exampleJNI.unnamedPairDestroy(UnnamedPair.getCPtr(ffi), ffi);
  }

  public static MyIdentityFactory ffiGetIdentityFactory() {
    long cPtr = exampleJNI.ffiGetIdentityFactory();
    return (cPtr == 0) ? null : new MyIdentityFactory(cPtr, false);
  }

  public static MapOfHashes mapOfHashesCtor(std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID o_0) {
    long cPtr = exampleJNI.mapOfHashesCtor(std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new MapOfHashes(cPtr, false);
  }

  public static void mapOfHashesDestroy(MapOfHashes ffi) {
    exampleJNI.mapOfHashesDestroy(MapOfHashes.getCPtr(ffi), ffi);
  }

  public static MapOfVecHashes mapOfVecHashesCtor(std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID o_0) {
    long cPtr = exampleJNI.mapOfVecHashesCtor(std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new MapOfVecHashes(cPtr, false);
  }

  public static void mapOfVecHashesDestroy(MapOfVecHashes ffi) {
    exampleJNI.mapOfVecHashesDestroy(MapOfVecHashes.getCPtr(ffi), ffi);
  }

  public static BinaryData binaryDataCtor(byte[] o_0) {
    long cPtr = exampleJNI.binaryDataCtor(o_0);
    return (cPtr == 0) ? null : new BinaryData(cPtr, false);
  }

  public static void binaryDataDestroy(BinaryData ffi) {
    exampleJNI.binaryDataDestroy(BinaryData.getCPtr(ffi), ffi);
  }

  public static UsedStruct usedStructCtor(HashID o_0) {
    long cPtr = exampleJNI.usedStructCtor(HashID.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new UsedStruct(cPtr, false);
  }

  public static void usedStructDestroy(UsedStruct ffi) {
    exampleJNI.usedStructDestroy(UsedStruct.getCPtr(ffi), ffi);
  }

  public static SWIGTYPE_p_Purpose purposeAUTHENTICATIONCtor() {
    long cPtr = exampleJNI.purposeAUTHENTICATIONCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static SWIGTYPE_p_Purpose purposeENCRYPTIONCtor() {
    long cPtr = exampleJNI.purposeENCRYPTIONCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static SWIGTYPE_p_Purpose purposeDECRYPTIONCtor() {
    long cPtr = exampleJNI.purposeDECRYPTIONCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static SWIGTYPE_p_Purpose purposeWITHDRAWCtor() {
    long cPtr = exampleJNI.purposeWITHDRAWCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static SWIGTYPE_p_Purpose purposeSYSTEMCtor() {
    long cPtr = exampleJNI.purposeSYSTEMCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static SWIGTYPE_p_Purpose purposeVOTINGCtor() {
    long cPtr = exampleJNI.purposeVOTINGCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_Purpose(cPtr, false);
  }

  public static void purposeDestroy(SWIGTYPE_p_Purpose ffi) {
    exampleJNI.purposeDestroy(SWIGTYPE_p_Purpose.getCPtr(ffi));
  }

  public static IdentityV0 identityV0Ctor(Identifier id, std_collections_Map_keys_crate_nested_KeyID_values_crate_nested_IdentityPublicKey public_keys, java.math.BigInteger balance, Revision revision) {
    long cPtr = exampleJNI.identityV0Ctor__SWIG_1(Identifier.getCPtr(id), id, std_collections_Map_keys_crate_nested_KeyID_values_crate_nested_IdentityPublicKey.getCPtr(public_keys), public_keys, balance, Revision.getCPtr(revision), revision);
    return (cPtr == 0) ? null : new IdentityV0(cPtr, false);
  }

  public static void identityV0Destroy(IdentityV0 ffi) {
    exampleJNI.identityV0Destroy(IdentityV0.getCPtr(ffi), ffi);
  }

  public static SWIGTYPE_p_KeyType keyTypeECDSASECP256K1Ctor() {
    long cPtr = exampleJNI.keyTypeECDSASECP256K1Ctor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static SWIGTYPE_p_KeyType keyTypeBLS12381Ctor() {
    long cPtr = exampleJNI.keyTypeBLS12381Ctor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static SWIGTYPE_p_KeyType keyTypeECDSAHASH160Ctor() {
    long cPtr = exampleJNI.keyTypeECDSAHASH160Ctor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static SWIGTYPE_p_KeyType keyTypeBIP13SCRIPTHASHCtor() {
    long cPtr = exampleJNI.keyTypeBIP13SCRIPTHASHCtor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static SWIGTYPE_p_KeyType keyTypeEDDSA25519HASH160Ctor() {
    long cPtr = exampleJNI.keyTypeEDDSA25519HASH160Ctor();
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static void keyTypeDestroy(SWIGTYPE_p_KeyType ffi) {
    exampleJNI.keyTypeDestroy(SWIGTYPE_p_KeyType.getCPtr(ffi));
  }

  public static DataContractNotPresentError dataContractNotPresentErrorCtor(Identifier data_contract_id) {
    long cPtr = exampleJNI.dataContractNotPresentErrorCtor(Identifier.getCPtr(data_contract_id), data_contract_id);
    return (cPtr == 0) ? null : new DataContractNotPresentError(cPtr, false);
  }

  public static void dataContractNotPresentErrorDestroy(DataContractNotPresentError ffi) {
    exampleJNI.dataContractNotPresentErrorDestroy(DataContractNotPresentError.getCPtr(ffi), ffi);
  }

  public static Identifier identifierCtor(IdentifierBytes32 o_0) {
    long cPtr = exampleJNI.identifierCtor(IdentifierBytes32.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new Identifier(cPtr, false);
  }

  public static void identifierDestroy(Identifier ffi) {
    exampleJNI.identifierDestroy(Identifier.getCPtr(ffi), ffi);
  }

  public static SimpleData simpleDataCtor(Vec_u32 o_0) {
    long cPtr = exampleJNI.simpleDataCtor(Vec_u32.getCPtr(o_0), o_0);
    return (cPtr == 0) ? null : new SimpleData(cPtr, false);
  }

  public static void simpleDataDestroy(SimpleData ffi) {
    exampleJNI.simpleDataDestroy(SimpleData.getCPtr(ffi), ffi);
  }

  public static Identity ffiGetIdentity2(Identifier identifier) {
    long cPtr = exampleJNI.ffiGetIdentity2(Identifier.getCPtr(identifier), identifier);
    return (cPtr == 0) ? null : new Identity(cPtr, false);
  }

  public static void testStructDestroy(TestStruct ffi) {
    exampleJNI.testStructDestroy(TestStruct.getCPtr(ffi), ffi);
  }

  public static Result_ok_u32_err_crate_nested_ProtocolError ffiAddressSimpleComplexResult(Vec_u32 script) {
    long cPtr = exampleJNI.ffiAddressSimpleComplexResult(Vec_u32.getCPtr(script), script);
    return (cPtr == 0) ? null : new Result_ok_u32_err_crate_nested_ProtocolError(cPtr, false);
  }

  public static Result_ok_crate_nested_HashID_err_u32 ffiAddressComplexSimpleResult(byte[] script) {
    long cPtr = exampleJNI.ffiAddressComplexSimpleResult(script);
    return (cPtr == 0) ? null : new Result_ok_crate_nested_HashID_err_u32(cPtr, false);
  }

  public static String ffiGetChainTypeString(ChainType chain_type) {
    return exampleJNI.ffiGetChainTypeString(ChainType.getCPtr(chain_type), chain_type);
  }

  public static Result_ok_u32_err_u32 ffiAddressSimpleResult(Vec_u32 script) {
    long cPtr = exampleJNI.ffiAddressSimpleResult(Vec_u32.getCPtr(script), script);
    return (cPtr == 0) ? null : new Result_ok_u32_err_u32(cPtr, false);
  }

  public static String ffiAddressWithScriptPubkey(byte[] script) {
    return exampleJNI.ffiAddressWithScriptPubkey(script);
  }

  public static String ffiGetChainHashesByMap(std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID map) {
    return exampleJNI.ffiGetChainHashesByMap(std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID.getCPtr(map), map);
  }

  public static Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError ffiAddressComplexResult(byte[] script) {
    long cPtr = exampleJNI.ffiAddressComplexResult(script);
    return (cPtr == 0) ? null : new Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError(cPtr, false);
  }

  public static RequestSettings requestSettingsCtor(SWIGTYPE_p_Duration_FFI timeout, long retries) {
    long cPtr = exampleJNI.requestSettingsCtor(SWIGTYPE_p_Duration_FFI.getCPtr(timeout), retries);
    return (cPtr == 0) ? null : new RequestSettings(cPtr, false);
  }

  public static void requestSettingsDestroy(RequestSettings ffi) {
    exampleJNI.requestSettingsDestroy(RequestSettings.getCPtr(ffi), ffi);
  }

  public static Uri uriCtor(String scheme) {
    long cPtr = exampleJNI.uriCtor(scheme);
    return (cPtr == 0) ? null : new Uri(cPtr, false);
  }

  public static void uriDestroy(Uri ffi) {
    exampleJNI.uriDestroy(Uri.getCPtr(ffi), ffi);
  }

  public static AppliedRequestSettings appliedRequestSettingsCtor(SWIGTYPE_p_Duration_FFI timeout, long retries) {
    long cPtr = exampleJNI.appliedRequestSettingsCtor(SWIGTYPE_p_Duration_FFI.getCPtr(timeout), retries);
    return (cPtr == 0) ? null : new AppliedRequestSettings(cPtr, false);
  }

  public static void appliedRequestSettingsDestroy(AppliedRequestSettings ffi) {
    exampleJNI.appliedRequestSettingsDestroy(AppliedRequestSettings.getCPtr(ffi), ffi);
  }

  public static String ffiGetChainTypeStringAsync(SWIGTYPE_p_void runtime, ChainType chain_type) {
    return exampleJNI.ffiGetChainTypeStringAsync(SWIGTYPE_p_void.getCPtr(runtime), ChainType.getCPtr(chain_type), chain_type);
  }

  public static byte[] vecU8Ctor(long count, SWIGTYPE_p_unsigned_char values) {
    return exampleJNI.vecU8Ctor(count, SWIGTYPE_p_unsigned_char.getCPtr(values));
  }

  public static void vecU8Destroy(byte[] vec_u8) {
    exampleJNI.vecU8Destroy(vec_u8);
  }

  public static SWIGTYPE_p_KeyType intToKeyType(int value) {
    long cPtr = exampleJNI.intToKeyType(value);
    return (cPtr == 0) ? null : new SWIGTYPE_p_KeyType(cPtr, false);
  }

  public static SWIGTYPE_p_SecurityLevel intToSecurityLevel(int value) {
    long cPtr = exampleJNI.intToSecurityLevel(value);
    return (cPtr == 0) ? null : new SWIGTYPE_p_SecurityLevel(cPtr, false);
  }

}
