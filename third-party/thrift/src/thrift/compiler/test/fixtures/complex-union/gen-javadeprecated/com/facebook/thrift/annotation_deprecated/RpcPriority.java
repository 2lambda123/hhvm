/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */
package com.facebook.thrift.annotation_deprecated;


import com.facebook.thrift.IntRangeSet;
import java.util.Map;
import java.util.HashMap;

@SuppressWarnings({ "unused" })
public enum RpcPriority implements com.facebook.thrift.TEnum {
  HIGH_IMPORTANT(0),
  HIGH(1),
  IMPORTANT(2),
  NORMAL(3),
  BEST_EFFORT(4);

  private final int value;

  private RpcPriority(int value) {
    this.value = value;
  }

  /**
   * Get the integer value of this enum value, as defined in the Thrift IDL.
   */
  public int getValue() {
    return value;
  }

  /**
   * Find a the enum type by its integer value, as defined in the Thrift IDL.
   * @return null if the value is not found.
   */
  public static RpcPriority findByValue(int value) { 
    switch (value) {
      case 0:
        return HIGH_IMPORTANT;
      case 1:
        return HIGH;
      case 2:
        return IMPORTANT;
      case 3:
        return NORMAL;
      case 4:
        return BEST_EFFORT;
      default:
        return null;
    }
  }
}
