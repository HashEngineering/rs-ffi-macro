/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public final class ContractBounds_FFI_Tag {
  public final static ContractBounds_FFI_Tag ContractBounds_FFI_SingleContract = new ContractBounds_FFI_Tag("ContractBounds_FFI_SingleContract");
  public final static ContractBounds_FFI_Tag ContractBounds_FFI_SingleContractDocumentType = new ContractBounds_FFI_Tag("ContractBounds_FFI_SingleContractDocumentType");

  public final int swigValue() {
    return swigValue;
  }

  public String toString() {
    return swigName;
  }

  public static ContractBounds_FFI_Tag swigToEnum(int swigValue) {
    if (swigValue < swigValues.length && swigValue >= 0 && swigValues[swigValue].swigValue == swigValue)
      return swigValues[swigValue];
    for (int i = 0; i < swigValues.length; i++)
      if (swigValues[i].swigValue == swigValue)
        return swigValues[i];
    throw new IllegalArgumentException("No enum " + ContractBounds_FFI_Tag.class + " with value " + swigValue);
  }

  private ContractBounds_FFI_Tag(String swigName) {
    this.swigName = swigName;
    this.swigValue = swigNext++;
  }

  private ContractBounds_FFI_Tag(String swigName, int swigValue) {
    this.swigName = swigName;
    this.swigValue = swigValue;
    swigNext = swigValue+1;
  }

  private ContractBounds_FFI_Tag(String swigName, ContractBounds_FFI_Tag swigEnum) {
    this.swigName = swigName;
    this.swigValue = swigEnum.swigValue;
    swigNext = this.swigValue+1;
  }

  private static ContractBounds_FFI_Tag[] swigValues = { ContractBounds_FFI_SingleContract, ContractBounds_FFI_SingleContractDocumentType };
  private static int swigNext = 0;
  private final int swigValue;
  private final String swigName;
}

