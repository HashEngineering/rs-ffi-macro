/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dash.sdk;

import org.dash.sdk.base.BaseObject;

public class ferment_example_nested_model_snapshot_LLMQSnapshot extends BaseObject {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected ferment_example_nested_model_snapshot_LLMQSnapshot(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(ferment_example_nested_model_snapshot_LLMQSnapshot obj) {
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
        exampleJNI.delete_ferment_example_nested_model_snapshot_LLMQSnapshot(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  protected long getCPointer() {
    return swigCPtr;
  }

  public void setMember_list(byte[] value) {
    exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_member_list_set(swigCPtr, this, value);
  }

  public byte[] getMember_list() {
    return exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_member_list_get(swigCPtr, this);
  }

  public void setSkip_list(Vec_i32 value) {
    exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_skip_list_set(swigCPtr, this, Vec_i32.getCPtr(value), value);
  }

  public Vec_i32 getSkip_list() {
    long cPtr = exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_skip_list_get(swigCPtr, this);
    return (cPtr == 0) ? null : new Vec_i32(cPtr, false);
  }

  public void setSkip_list_mode(SWIGTYPE_p_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode value) {
    exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_skip_list_mode_set(swigCPtr, this, SWIGTYPE_p_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode.getCPtr(value));
  }

  public SWIGTYPE_p_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode getSkip_list_mode() {
    long cPtr = exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_skip_list_mode_get(swigCPtr, this);
    return (cPtr == 0) ? null : new SWIGTYPE_p_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode(cPtr, false);
  }

  public void setOption_vec(byte[] value) {
    exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_option_vec_set(swigCPtr, this, value);
  }

  public byte[] getOption_vec() {
    return exampleJNI.ferment_example_nested_model_snapshot_LLMQSnapshot_option_vec_get(swigCPtr, this);
  }

}
