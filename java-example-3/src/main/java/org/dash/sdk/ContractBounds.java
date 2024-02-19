/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class ContractBounds {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected ContractBounds(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(ContractBounds obj) {
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
        exampleJNI.delete_ContractBounds(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public void setTag(ContractBounds_Tag value) {
    exampleJNI.ContractBounds_tag_set(swigCPtr, this, value.swigValue());
  }

  public ContractBounds_Tag getTag() {
    return ContractBounds_Tag.swigToEnum(exampleJNI.ContractBounds_tag_get(swigCPtr, this));
  }

  public void setSingle_contract(ContractBounds_SingleContract_Body value) {
    exampleJNI.ContractBounds_single_contract_set(swigCPtr, this, ContractBounds_SingleContract_Body.getCPtr(value), value);
  }

  public ContractBounds_SingleContract_Body getSingle_contract() {
    long cPtr = exampleJNI.ContractBounds_single_contract_get(swigCPtr, this);
    return (cPtr == 0) ? null : new ContractBounds_SingleContract_Body(cPtr, false);
  }

  public void setSingle_contract_document_type(ContractBounds_SingleContractDocumentType_Body value) {
    exampleJNI.ContractBounds_single_contract_document_type_set(swigCPtr, this, ContractBounds_SingleContractDocumentType_Body.getCPtr(value), value);
  }

  public ContractBounds_SingleContractDocumentType_Body getSingle_contract_document_type() {
    long cPtr = exampleJNI.ContractBounds_single_contract_document_type_get(swigCPtr, this);
    return (cPtr == 0) ? null : new ContractBounds_SingleContractDocumentType_Body(cPtr, false);
  }

  public static ContractBounds singleContract(Identifier id) {
    long cPtr = exampleJNI.ContractBounds_singleContract(Identifier.getCPtr(id), id);
    return (cPtr == 0) ? null : new ContractBounds(cPtr, false);
  }

  public static ContractBounds singleContractDocumentType(Identifier id, String type) {
    long cPtr = exampleJNI.ContractBounds_singleContractDocumentType(Identifier.getCPtr(id), id, type);
    return (cPtr == 0) ? null : new ContractBounds(cPtr, false);
  }

}
