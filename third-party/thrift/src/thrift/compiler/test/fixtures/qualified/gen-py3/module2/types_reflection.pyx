#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#


import folly.iobuf as _fbthrift_iobuf

from thrift.py3.reflection cimport (
    NumberType as __NumberType,
    StructType as __StructType,
    Qualifier as __Qualifier,
)

cimport module0.types as _module0_types
cimport module1.types as _module1_types

cimport module2.types as _module2_types

from thrift.py3.types cimport (
    constant_shared_ptr,
    default_inst,
)


cdef __StructSpec get_reflection__Struct():
    cdef _module2_types.Struct defaults = _module2_types.Struct._fbthrift_create(
        constant_shared_ptr[_module2_types.cStruct](
            default_inst[_module2_types.cStruct]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="Struct",
        kind=__StructType.STRUCT,
        annotations={
        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="first",
            py_name="first",
            type=_module0_types.Struct,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=2,
            name="second",
            py_name="second",
            type=_module1_types.Struct,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    return spec
cdef __StructSpec get_reflection__BigStruct():
    cdef _module2_types.BigStruct defaults = _module2_types.BigStruct._fbthrift_create(
        constant_shared_ptr[_module2_types.cBigStruct](
            default_inst[_module2_types.cBigStruct]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="BigStruct",
        kind=__StructType.STRUCT,
        annotations={
        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="s",
            py_name="s",
            type=_module2_types.Struct,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=2,
            name="id",
            py_name="id",
            type=int,
            kind=__NumberType.I32,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    return spec
