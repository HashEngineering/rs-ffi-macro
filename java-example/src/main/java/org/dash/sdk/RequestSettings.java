/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class RequestSettings {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected RequestSettings(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(RequestSettings obj) {
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
        exampleJNI.delete_RequestSettings(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public void setTimeout(SWIGTYPE_p_Duration_FFI value) {
    exampleJNI.RequestSettings_timeout_set(swigCPtr, this, SWIGTYPE_p_Duration_FFI.getCPtr(value));
  }

  public SWIGTYPE_p_Duration_FFI getTimeout() {
    long cPtr = exampleJNI.RequestSettings_timeout_get(swigCPtr, this);
    return (cPtr == 0) ? null : new SWIGTYPE_p_Duration_FFI(cPtr, false);
  }

  public void setRetries(long value) {
    exampleJNI.RequestSettings_retries_set(swigCPtr, this, value);
  }

  public long getRetries() {
    return exampleJNI.RequestSettings_retries_get(swigCPtr, this);
  }

}
