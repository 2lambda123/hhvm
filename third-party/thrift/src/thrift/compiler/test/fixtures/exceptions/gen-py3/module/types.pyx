#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#
cimport cython as __cython
from cpython.object cimport PyTypeObject, Py_LT, Py_LE, Py_EQ, Py_NE, Py_GT, Py_GE
from libcpp.memory cimport shared_ptr, make_shared, unique_ptr, make_unique
from libcpp.optional cimport optional as __optional
from libcpp.string cimport string
from libcpp cimport bool as cbool
from libcpp.iterator cimport inserter as cinserter
from cpython cimport bool as pbool
from cython.operator cimport dereference as deref, preincrement as inc, address as ptr_address
import thrift.py3.types
from thrift.py3.types import _IsSet as _fbthrift_IsSet
cimport thrift.py3.types
cimport thrift.py3.exceptions
from thrift.py3.std_libcpp cimport sv_to_str as __sv_to_str, string_view as __cstring_view
from thrift.py3.types cimport (
    cSetOp as __cSetOp,
    richcmp as __richcmp,
    set_op as __set_op,
    setcmp as __setcmp,
    list_index as __list_index,
    list_count as __list_count,
    list_slice as __list_slice,
    list_getitem as __list_getitem,
    set_iter as __set_iter,
    map_iter as __map_iter,
    map_contains as __map_contains,
    map_getitem as __map_getitem,
    reference_shared_ptr as __reference_shared_ptr,
    get_field_name_by_index as __get_field_name_by_index,
    reset_field as __reset_field,
    translate_cpp_enum_to_python,
    SetMetaClass as __SetMetaClass,
    const_pointer_cast,
    constant_shared_ptr,
    NOTSET as __NOTSET,
    EnumData as __EnumData,
    EnumFlagsData as __EnumFlagsData,
    UnionTypeEnumData as __UnionTypeEnumData,
    createEnumDataForUnionType as __createEnumDataForUnionType,
)
cimport thrift.py3.serializer as serializer
import folly.iobuf as _fbthrift_iobuf
from folly.optional cimport cOptional
from folly.memory cimport to_shared_ptr as __to_shared_ptr
from folly.range cimport Range as __cRange

import sys
from collections.abc import Sequence, Set, Mapping, Iterable
import weakref as __weakref
import builtins as _builtins

cimport module.types_reflection as _types_reflection



@__cython.auto_pickle(False)
cdef class Fiery(thrift.py3.exceptions.GeneratedError):
    def __init__(Fiery self, *args, **kwargs):
        self._cpp_obj = make_shared[cFiery]()
        self._fields_setter = _fbthrift_types_fields.__Fiery_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("Fiery", {
          "message": deref(self._cpp_obj).message_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cFiery] cpp_obj):
        __fbthrift_inst = <Fiery>Fiery.__new__(Fiery, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline message_impl(self):

        return (<bytes>deref(self._cpp_obj).message_ref().value()).decode('UTF-8')

    @property
    def message(self):
        return self.message_impl()


    def __hash__(Fiery self):
        return super().__hash__()

    def __repr__(Fiery self):
        return super().__repr__()

    def __str__(Fiery self):
        field = self.message
        if field is None:
            return str(field)
        return field


    def __copy__(Fiery self):
        cdef shared_ptr[cFiery] cpp_obj = make_shared[cFiery](
            deref(self._cpp_obj)
        )
        return Fiery._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cFiery](
            self._cpp_obj,
            (<Fiery>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__Fiery()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cFiery].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.Fiery"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cFiery](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 1

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(Fiery self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cFiery](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(Fiery self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cFiery]()
        with nogil:
            needed = serializer.cdeserialize[cFiery](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.Fiery, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.Fiery, self)
@__cython.auto_pickle(False)
cdef class Serious(thrift.py3.exceptions.GeneratedError):
    def __init__(Serious self, *args, **kwargs):
        self._cpp_obj = make_shared[cSerious]()
        self._fields_setter = _fbthrift_types_fields.__Serious_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("Serious", {
          "sonnet": deref(self._cpp_obj).sonnet_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cSerious] cpp_obj):
        __fbthrift_inst = <Serious>Serious.__new__(Serious, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline sonnet_impl(self):
        if not deref(self._cpp_obj).sonnet_ref().has_value():
            return None

        return (<bytes>deref(self._cpp_obj).sonnet_ref().value_unchecked()).decode('UTF-8')

    @property
    def sonnet(self):
        return self.sonnet_impl()


    def __hash__(Serious self):
        return super().__hash__()

    def __repr__(Serious self):
        return super().__repr__()

    def __str__(Serious self):
        field = self.sonnet
        if field is None:
            return str(field)
        return field


    def __copy__(Serious self):
        cdef shared_ptr[cSerious] cpp_obj = make_shared[cSerious](
            deref(self._cpp_obj)
        )
        return Serious._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cSerious](
            self._cpp_obj,
            (<Serious>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__Serious()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cSerious].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.Serious"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cSerious](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 1

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(Serious self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cSerious](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(Serious self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cSerious]()
        with nogil:
            needed = serializer.cdeserialize[cSerious](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.Serious, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.Serious, self)
@__cython.auto_pickle(False)
cdef class ComplexFieldNames(thrift.py3.exceptions.GeneratedError):
    def __init__(ComplexFieldNames self, *args, **kwargs):
        self._cpp_obj = make_shared[cComplexFieldNames]()
        self._fields_setter = _fbthrift_types_fields.__ComplexFieldNames_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("ComplexFieldNames", {
          "error_message": deref(self._cpp_obj).error_message_ref().has_value(),
          "internal_error_message": deref(self._cpp_obj).internal_error_message_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cComplexFieldNames] cpp_obj):
        __fbthrift_inst = <ComplexFieldNames>ComplexFieldNames.__new__(ComplexFieldNames, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline error_message_impl(self):

        return (<bytes>deref(self._cpp_obj).error_message_ref().value()).decode('UTF-8')

    @property
    def error_message(self):
        return self.error_message_impl()

    cdef inline internal_error_message_impl(self):

        return (<bytes>deref(self._cpp_obj).internal_error_message_ref().value()).decode('UTF-8')

    @property
    def internal_error_message(self):
        return self.internal_error_message_impl()


    def __hash__(ComplexFieldNames self):
        return super().__hash__()

    def __repr__(ComplexFieldNames self):
        return super().__repr__()

    def __str__(ComplexFieldNames self):
        field = self.internal_error_message
        if field is None:
            return str(field)
        return field


    def __copy__(ComplexFieldNames self):
        cdef shared_ptr[cComplexFieldNames] cpp_obj = make_shared[cComplexFieldNames](
            deref(self._cpp_obj)
        )
        return ComplexFieldNames._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cComplexFieldNames](
            self._cpp_obj,
            (<ComplexFieldNames>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__ComplexFieldNames()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cComplexFieldNames].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.ComplexFieldNames"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cComplexFieldNames](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 2

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(ComplexFieldNames self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cComplexFieldNames](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(ComplexFieldNames self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cComplexFieldNames]()
        with nogil:
            needed = serializer.cdeserialize[cComplexFieldNames](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.ComplexFieldNames, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.ComplexFieldNames, self)
@__cython.auto_pickle(False)
cdef class CustomFieldNames(thrift.py3.exceptions.GeneratedError):
    def __init__(CustomFieldNames self, *args, **kwargs):
        self._cpp_obj = make_shared[cCustomFieldNames]()
        self._fields_setter = _fbthrift_types_fields.__CustomFieldNames_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("CustomFieldNames", {
          "error_message": deref(self._cpp_obj).error_message_ref().has_value(),
          "internal_error_message": deref(self._cpp_obj).internal_error_message_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cCustomFieldNames] cpp_obj):
        __fbthrift_inst = <CustomFieldNames>CustomFieldNames.__new__(CustomFieldNames, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline error_message_impl(self):

        return (<bytes>deref(self._cpp_obj).error_message_ref().value()).decode('UTF-8')

    @property
    def error_message(self):
        return self.error_message_impl()

    cdef inline internal_error_message_impl(self):

        return (<bytes>deref(self._cpp_obj).internal_error_message_ref().value()).decode('UTF-8')

    @property
    def internal_error_message(self):
        return self.internal_error_message_impl()


    def __hash__(CustomFieldNames self):
        return super().__hash__()

    def __repr__(CustomFieldNames self):
        return super().__repr__()

    def __str__(CustomFieldNames self):
        field = self.internal_error_message
        if field is None:
            return str(field)
        return field


    def __copy__(CustomFieldNames self):
        cdef shared_ptr[cCustomFieldNames] cpp_obj = make_shared[cCustomFieldNames](
            deref(self._cpp_obj)
        )
        return CustomFieldNames._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cCustomFieldNames](
            self._cpp_obj,
            (<CustomFieldNames>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__CustomFieldNames()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cCustomFieldNames].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.CustomFieldNames"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cCustomFieldNames](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 2

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(CustomFieldNames self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cCustomFieldNames](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(CustomFieldNames self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cCustomFieldNames]()
        with nogil:
            needed = serializer.cdeserialize[cCustomFieldNames](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.CustomFieldNames, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.CustomFieldNames, self)
@__cython.auto_pickle(False)
cdef class ExceptionWithPrimitiveField(thrift.py3.exceptions.GeneratedError):
    def __init__(ExceptionWithPrimitiveField self, *args, **kwargs):
        self._cpp_obj = make_shared[cExceptionWithPrimitiveField]()
        self._fields_setter = _fbthrift_types_fields.__ExceptionWithPrimitiveField_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("ExceptionWithPrimitiveField", {
          "message": deref(self._cpp_obj).message_ref().has_value(),
          "error_code": deref(self._cpp_obj).error_code_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cExceptionWithPrimitiveField] cpp_obj):
        __fbthrift_inst = <ExceptionWithPrimitiveField>ExceptionWithPrimitiveField.__new__(ExceptionWithPrimitiveField, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline message_impl(self):

        return (<bytes>deref(self._cpp_obj).message_ref().value()).decode('UTF-8')

    @property
    def message(self):
        return self.message_impl()

    cdef inline error_code_impl(self):

        return deref(self._cpp_obj).error_code_ref().value()

    @property
    def error_code(self):
        return self.error_code_impl()


    def __hash__(ExceptionWithPrimitiveField self):
        return super().__hash__()

    def __repr__(ExceptionWithPrimitiveField self):
        return super().__repr__()

    def __str__(ExceptionWithPrimitiveField self):
        field = self.message
        if field is None:
            return str(field)
        return field


    def __copy__(ExceptionWithPrimitiveField self):
        cdef shared_ptr[cExceptionWithPrimitiveField] cpp_obj = make_shared[cExceptionWithPrimitiveField](
            deref(self._cpp_obj)
        )
        return ExceptionWithPrimitiveField._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cExceptionWithPrimitiveField](
            self._cpp_obj,
            (<ExceptionWithPrimitiveField>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__ExceptionWithPrimitiveField()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cExceptionWithPrimitiveField].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.ExceptionWithPrimitiveField"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cExceptionWithPrimitiveField](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 2

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(ExceptionWithPrimitiveField self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cExceptionWithPrimitiveField](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(ExceptionWithPrimitiveField self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cExceptionWithPrimitiveField]()
        with nogil:
            needed = serializer.cdeserialize[cExceptionWithPrimitiveField](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.ExceptionWithPrimitiveField, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.ExceptionWithPrimitiveField, self)
@__cython.auto_pickle(False)
cdef class ExceptionWithStructuredAnnotation(thrift.py3.exceptions.GeneratedError):
    def __init__(ExceptionWithStructuredAnnotation self, *args, **kwargs):
        self._cpp_obj = make_shared[cExceptionWithStructuredAnnotation]()
        self._fields_setter = _fbthrift_types_fields.__ExceptionWithStructuredAnnotation_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("ExceptionWithStructuredAnnotation", {
          "message_field": deref(self._cpp_obj).message_field_ref().has_value(),
          "error_code": deref(self._cpp_obj).error_code_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cExceptionWithStructuredAnnotation] cpp_obj):
        __fbthrift_inst = <ExceptionWithStructuredAnnotation>ExceptionWithStructuredAnnotation.__new__(ExceptionWithStructuredAnnotation, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst

    cdef inline message_field_impl(self):

        return (<bytes>deref(self._cpp_obj).message_field_ref().value()).decode('UTF-8')

    @property
    def message_field(self):
        return self.message_field_impl()

    cdef inline error_code_impl(self):

        return deref(self._cpp_obj).error_code_ref().value()

    @property
    def error_code(self):
        return self.error_code_impl()


    def __hash__(ExceptionWithStructuredAnnotation self):
        return super().__hash__()

    def __repr__(ExceptionWithStructuredAnnotation self):
        return super().__repr__()

    def __str__(ExceptionWithStructuredAnnotation self):
        field = self.message_field
        if field is None:
            return str(field)
        return field


    def __copy__(ExceptionWithStructuredAnnotation self):
        cdef shared_ptr[cExceptionWithStructuredAnnotation] cpp_obj = make_shared[cExceptionWithStructuredAnnotation](
            deref(self._cpp_obj)
        )
        return ExceptionWithStructuredAnnotation._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cExceptionWithStructuredAnnotation](
            self._cpp_obj,
            (<ExceptionWithStructuredAnnotation>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__ExceptionWithStructuredAnnotation()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cExceptionWithStructuredAnnotation].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.ExceptionWithStructuredAnnotation"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cExceptionWithStructuredAnnotation](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 2

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(ExceptionWithStructuredAnnotation self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cExceptionWithStructuredAnnotation](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(ExceptionWithStructuredAnnotation self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cExceptionWithStructuredAnnotation]()
        with nogil:
            needed = serializer.cdeserialize[cExceptionWithStructuredAnnotation](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.ExceptionWithStructuredAnnotation, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.ExceptionWithStructuredAnnotation, self)
@__cython.auto_pickle(False)
cdef class Banal(thrift.py3.exceptions.GeneratedError):
    def __init__(Banal self, *args, **kwargs):
        self._cpp_obj = make_shared[cBanal]()
        self._fields_setter = _fbthrift_types_fields.__Banal_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__( *args, **kwargs)

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("Banal", {
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cBanal] cpp_obj):
        __fbthrift_inst = <Banal>Banal.__new__(Banal, (<bytes>deref(cpp_obj).what()).decode('utf-8'))
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        _builtins.Exception.__init__(__fbthrift_inst, *(v for _, v in __fbthrift_inst))
        return __fbthrift_inst


    def __hash__(Banal self):
        return super().__hash__()

    def __repr__(Banal self):
        return super().__repr__()

    def __str__(Banal self):
        return super().__str__()


    def __copy__(Banal self):
        cdef shared_ptr[cBanal] cpp_obj = make_shared[cBanal](
            deref(self._cpp_obj)
        )
        return Banal._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cBanal](
            self._cpp_obj,
            (<Banal>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__Banal()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        ExceptionMetadata[cBanal].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.Banal"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cBanal](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 0

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(Banal self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cBanal](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(Banal self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cBanal]()
        with nogil:
            needed = serializer.cdeserialize[cBanal](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.Banal, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.Banal, self)
