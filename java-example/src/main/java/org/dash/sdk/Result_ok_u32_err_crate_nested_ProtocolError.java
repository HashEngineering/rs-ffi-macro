/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class Result_ok_u32_err_crate_nested_ProtocolError {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected Result_ok_u32_err_crate_nested_ProtocolError(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(Result_ok_u32_err_crate_nested_ProtocolError obj) {
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
        exampleJNI.delete_Result_ok_u32_err_crate_nested_ProtocolError(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public void setOk(SWIGTYPE_p_unsigned_int value) {
    exampleJNI.Result_ok_u32_err_crate_nested_ProtocolError_ok_set(swigCPtr, this, SWIGTYPE_p_unsigned_int.getCPtr(value));
  }

  public SWIGTYPE_p_unsigned_int getOk() {
    long cPtr = exampleJNI.Result_ok_u32_err_crate_nested_ProtocolError_ok_get(swigCPtr, this);
    return (cPtr == 0) ? null : new SWIGTYPE_p_unsigned_int(cPtr, false);
  }

  public void setError(ProtocolError value) {
    exampleJNI.Result_ok_u32_err_crate_nested_ProtocolError_error_set(swigCPtr, this, ProtocolError.getCPtr(value), value);
  }

  public ProtocolError getError() {
    long cPtr = exampleJNI.Result_ok_u32_err_crate_nested_ProtocolError_error_get(swigCPtr, this);
    return (cPtr == 0) ? null : new ProtocolError(cPtr, false);
  }

}
