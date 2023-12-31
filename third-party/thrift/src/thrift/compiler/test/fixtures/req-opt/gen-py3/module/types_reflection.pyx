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


cimport module.types as _module_types

from thrift.py3.types cimport (
    constant_shared_ptr,
    default_inst,
)


cdef __StructSpec get_reflection__Foo():
    cdef _module_types.Foo defaults = _module_types.Foo._fbthrift_create(
        constant_shared_ptr[_module_types.cFoo](
            default_inst[_module_types.cFoo]()
        )
    )
    cdef __StructSpec spec = __StructSpec._fbthrift_create(
        name="Foo",
        kind=__StructType.STRUCT,
        annotations={
        },
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=1,
            name="myInteger",
            py_name="myInteger",
            type=int,
            kind=__NumberType.I32,
            qualifier=__Qualifier.REQUIRED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=2,
            name="myString",
            py_name="myString",
            type=str,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.OPTIONAL,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=3,
            name="myBools",
            py_name="myBools",
            type=_module_types.List__bool,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.UNQUALIFIED,
            default=None,
            annotations={
            },
        ),
    )
    spec.add_field(
        __FieldSpec._fbthrift_create(
            id=4,
            name="myNumbers",
            py_name="myNumbers",
            type=_module_types.List__i32,
            kind=__NumberType.NOT_A_NUMBER,
            qualifier=__Qualifier.REQUIRED,
            default=None,
            annotations={
            },
        ),
    )
    return spec
cdef __ListSpec get_reflection__List__bool():
    return __ListSpec._fbthrift_create(
        value=bool,
        kind=__NumberType.NOT_A_NUMBER,
    )

cdef __ListSpec get_reflection__List__i32():
    return __ListSpec._fbthrift_create(
        value=int,
        kind=__NumberType.I32,
    )

