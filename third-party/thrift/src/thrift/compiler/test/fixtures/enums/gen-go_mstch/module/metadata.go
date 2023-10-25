// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]

import (
    thrift "github.com/facebook/fbthrift/thrift/lib/go/thrift"
    metadata "github.com/facebook/fbthrift/thrift/lib/thrift/metadata"
)

// (needed to ensure safety because of naive import list construction)
var _ = thrift.ZERO
var _ = metadata.GoUnusedProtection__

// Primitive Thrift types
var (
    primitiveThriftType_THRIFT_STRING_TYPE = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr())
    primitiveThriftType_THRIFT_BINARY_TYPE = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_BINARY_TYPE.Ptr())
    primitiveThriftType_THRIFT_BOOL_TYPE   = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr())
    primitiveThriftType_THRIFT_BYTE_TYPE   = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_BYTE_TYPE.Ptr())
    primitiveThriftType_THRIFT_I16_TYPE    = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_I16_TYPE.Ptr())
    primitiveThriftType_THRIFT_I32_TYPE    = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_I32_TYPE.Ptr())
    primitiveThriftType_THRIFT_I64_TYPE    = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr())
    primitiveThriftType_THRIFT_DOUBLE_TYPE = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_DOUBLE_TYPE.Ptr())
    primitiveThriftType_THRIFT_FLOAT_TYPE  = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_FLOAT_TYPE.Ptr())
    primitiveThriftType_THRIFT_VOID_TYPE   = metadata.NewThriftType().SetTPrimitive(metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr())
)

var structMetadatas = []*metadata.ThriftStruct{
    metadata.NewThriftStruct().
    SetName("module.SomeStruct").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("reasonable").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.Metasyntactic"),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("fine").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.Metasyntactic"),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("questionable").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.Metasyntactic"),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("tags").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTSet(
        metadata.NewThriftSetType().
            SetValueType(primitiveThriftType_THRIFT_I32_TYPE),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.MyStruct").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("me2_3").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum2"),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("me3_n3").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum3"),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("me1_t1").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum1"),
            ),
    ),
            metadata.NewThriftField().
    SetId(6).
    SetName("me1_t2").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum1"),
            ),
    ),
        },
    ),
}

var exceptionMetadatas = []*metadata.ThriftException{
}

var enumMetadatas = []*metadata.ThriftEnum{
    metadata.NewThriftEnum().
    SetName("module.Metasyntactic").
    SetElements(
        map[int32]string{
            1: "FOO",
            2: "BAR",
            3: "BAZ",
            4: "BAX",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyEnum1").
    SetElements(
        map[int32]string{
            0: "ME1_0",
            1: "ME1_1",
            2: "ME1_2",
            3: "ME1_3",
            5: "ME1_5",
            6: "ME1_6",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyEnum2").
    SetElements(
        map[int32]string{
            0: "ME2_0",
            1: "ME2_1",
            2: "ME2_2",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyEnum3").
    SetElements(
        map[int32]string{
            0: "ME3_0",
            1: "ME3_1",
            -2: "ME3_N2",
            -1: "ME3_N1",
            9: "ME3_9",
            10: "ME3_10",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyEnum4").
    SetElements(
        map[int32]string{
            2147483645: "ME4_A",
            2147483646: "ME4_B",
            2147483647: "ME4_C",
            -2147483648: "ME4_D",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyBitmaskEnum1").
    SetElements(
        map[int32]string{
            1: "ONE",
            2: "TWO",
            4: "FOUR",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.MyBitmaskEnum2").
    SetElements(
        map[int32]string{
            1: "ONE",
            2: "TWO",
            4: "FOUR",
        },
    ),
}

var serviceMetadatas = []*metadata.ThriftService{
}

// GetThriftMetadata returns complete Thrift metadata for current and imported packages.
func GetThriftMetadata() *metadata.ThriftMetadata {
    includedEnumsMetadatas := []map[string]*metadata.ThriftEnum{
        GetEnumsMetadata(),
    }
    includedStructsMetadatas := []map[string]*metadata.ThriftStruct{
        GetStructsMetadata(),
    }
    includedExceptionsMetadatas := []map[string]*metadata.ThriftException{
        GetExceptionsMetadata(),
    }
    includedServicesMetadatas := []map[string]*metadata.ThriftService{
        GetServicesMetadata(),
    }

	allEnums := make(map[string]*metadata.ThriftEnum)
	allStructs := make(map[string]*metadata.ThriftStruct)
	allExceptions := make(map[string]*metadata.ThriftException)
    allServices := make(map[string]*metadata.ThriftService)

    for _, includedEnumsMetadata := range includedEnumsMetadatas {
        for enumName, thriftEnum := range includedEnumsMetadata {
            allEnums[enumName] = thriftEnum
        }
    }
    for _, includedStructsMetadata := range includedStructsMetadatas {
        for structName, thriftStruct := range includedStructsMetadata {
            allStructs[structName] = thriftStruct
        }
    }
    for _, includedExceptionsMetadata := range includedExceptionsMetadatas {
        for exceptionName, thriftException := range includedExceptionsMetadata {
            allExceptions[exceptionName] = thriftException
        }
    }
    for _, includedServicesMetadata := range includedServicesMetadatas {
        for serviceName, thriftService := range includedServicesMetadata {
            allServices[serviceName] = thriftService
        }
    }

    return metadata.NewThriftMetadata().
        SetEnums(allEnums).
        SetStructs(allStructs).
        SetExceptions(allExceptions).
        SetServices(allServices)
}

// GetStructsMetadata returns Thrift metadata for enums in the current package.
func GetEnumsMetadata() map[string]*metadata.ThriftEnum {
    result := make(map[string]*metadata.ThriftEnum)
    for _, enumMetadata := range enumMetadatas {
        result[enumMetadata.GetName()] = enumMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for structs in the current package.
func GetStructsMetadata() map[string]*metadata.ThriftStruct {
    result := make(map[string]*metadata.ThriftStruct)
    for _, structMetadata := range structMetadatas {
        result[structMetadata.GetName()] = structMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for exceptions in the current package.
func GetExceptionsMetadata() map[string]*metadata.ThriftException {
    result := make(map[string]*metadata.ThriftException)
    for _, exceptionMetadata := range exceptionMetadatas {
        result[exceptionMetadata.GetName()] = exceptionMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for services in the current package.
func GetServicesMetadata() map[string]*metadata.ThriftService {
    result := make(map[string]*metadata.ThriftService)
    for _, serviceMetadata := range serviceMetadatas {
        result[serviceMetadata.GetName()] = serviceMetadata
    }
    return result
}
