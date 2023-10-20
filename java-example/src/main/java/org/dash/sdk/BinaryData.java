/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

public class BinaryData {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected BinaryData(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(BinaryData obj) {
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
        exampleJNI.delete_BinaryData(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public void set_0(Vec_u8_FFI value) {
    exampleJNI.BinaryData__0_set(swigCPtr, this, Vec_u8_FFI.getCPtr(value), value);
  }

  public Vec_u8_FFI get_0() {
    long cPtr = exampleJNI.BinaryData__0_get(swigCPtr, this);
    return (cPtr == 0) ? null : new Vec_u8_FFI(cPtr, false);
  }

  public BinaryData() {
    this(exampleJNI.new_BinaryData(), true);
  }

}
