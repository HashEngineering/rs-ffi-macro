/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class ContractBounds_FFI_SingleContractDocumentType_Body {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected ContractBounds_FFI_SingleContractDocumentType_Body(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(ContractBounds_FFI_SingleContractDocumentType_Body obj) {
    return (obj == null) ? 0 : obj.swigCPtr;
  }

  @SuppressWarnings("deprecation")
  protected void finalize() {
    delete();
  }

  public synchronized void delete() {
    if (swigCPtr != 0) {
      if (swigCMemOwn) {
        swigCMemOwn = false;
        exampleJNI.delete_ContractBounds_FFI_SingleContractDocumentType_Body(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public void set_0(Identifier value) {
    exampleJNI.ContractBounds_FFI_SingleContractDocumentType_Body__0_set(swigCPtr, this, Identifier.getCPtr(value), value);
  }

  public Identifier get_0() {
    long cPtr = exampleJNI.ContractBounds_FFI_SingleContractDocumentType_Body__0_get(swigCPtr, this);
    return (cPtr == 0) ? null : new Identifier(cPtr, false);
  }

  public void set_1(String value) {
    exampleJNI.ContractBounds_FFI_SingleContractDocumentType_Body__1_set(swigCPtr, this, value);
  }

  public String get_1() {
    return exampleJNI.ContractBounds_FFI_SingleContractDocumentType_Body__1_get(swigCPtr, this);
  }

  public ContractBounds_FFI_SingleContractDocumentType_Body() {
    this(exampleJNI.new_ContractBounds_FFI_SingleContractDocumentType_Body(), true);
  }

}
