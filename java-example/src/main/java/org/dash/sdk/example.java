/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class example {
  public static Identity ffiGetAnIdentity() {
    long cPtr = exampleJNI.ffiGetAnIdentity();
    return (cPtr == 0) ? null : new Identity(cPtr, false);
  }

  public static HashID ffiFindHashByU32(long key, std_collections_Map_keys_u32_values_crate_nested_HashID_FFI map) {
    long cPtr = exampleJNI.ffiFindHashByU32(key, std_collections_Map_keys_u32_values_crate_nested_HashID_FFI.getCPtr(map), map);
    return (cPtr == 0) ? null : new HashID(cPtr, false);
  }

  public static Identity ffiGetIdentity(Identifier identifier) {
    long cPtr = exampleJNI.ffiGetIdentity(Identifier.getCPtr(identifier), identifier);
    return (cPtr == 0) ? null : new Identity(cPtr, false);
  }

  public static String ffiGetChainHashesByMap(std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI map) {
    return exampleJNI.ffiGetChainHashesByMap(std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID_FFI.getCPtr(map), map);
  }

  public static String ffiAddressWithScriptPubkey(byte[] script) {
    return exampleJNI.ffiAddressWithScriptPubkey(script);
  }

  public static String ffiGetChainTypeString(SWIGTYPE_p_ChainType_FFI chain_type) {
    return exampleJNI.ffiGetChainTypeString(SWIGTYPE_p_ChainType_FFI.getCPtr(chain_type));
  }

}
