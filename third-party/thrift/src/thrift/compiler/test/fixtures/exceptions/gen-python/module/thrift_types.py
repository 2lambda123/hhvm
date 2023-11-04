#
# Autogenerated by Thrift
#
# DO NOT EDIT
#  @generated
#

from __future__ import annotations

import folly.iobuf as _fbthrift_iobuf
import thrift.python.types as _fbthrift_python_types
import thrift.python.exceptions as _fbthrift_python_exceptions



class Fiery(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.Fiery"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_Fiery()


    def __str__(self):
        field = self.message
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.Fiery, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.Fiery, self)


class Serious(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "sonnet",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.Serious"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_Serious()


    def __str__(self):
        field = self.sonnet
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.Serious, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.Serious, self)


class ComplexFieldNames(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "error_message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "internal_error_message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.ComplexFieldNames"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_ComplexFieldNames()


    def __str__(self):
        field = self.internal_error_message
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.ComplexFieldNames, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.ComplexFieldNames, self)


class CustomFieldNames(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "error_message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "internal_error_message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.CustomFieldNames"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_CustomFieldNames()


    def __str__(self):
        field = self.internal_error_message
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.CustomFieldNames, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.CustomFieldNames, self)


class ExceptionWithPrimitiveField(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "message",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "error_code",  # name
            _fbthrift_python_types.typeinfo_i32,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.ExceptionWithPrimitiveField"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_ExceptionWithPrimitiveField()


    def __str__(self):
        field = self.message
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.ExceptionWithPrimitiveField, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.ExceptionWithPrimitiveField, self)


class ExceptionWithStructuredAnnotation(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "message_field",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Unqualified, # qualifier
            "error_code",  # name
            _fbthrift_python_types.typeinfo_i32,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.ExceptionWithStructuredAnnotation"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_ExceptionWithStructuredAnnotation()


    def __str__(self):
        field = self.message_field
        if field is None:
            return str(field)
        return field

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.ExceptionWithStructuredAnnotation, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.ExceptionWithStructuredAnnotation, self)


class Banal(metaclass=_fbthrift_python_exceptions.GeneratedErrorMeta):
    _fbthrift_SPEC = (
    )

    @staticmethod
    def __get_thrift_name__() -> str:
        return "module.Banal"

    @staticmethod
    def __get_thrift_uri__():
        return None

    @staticmethod
    def __get_metadata__():
        return _fbthrift_metadata__exception_Banal()

    def _to_python(self):
        return self

    def _to_py3(self):
        import importlib
        py3_types = importlib.import_module("module.types")
        import thrift.py3.converter
        return thrift.py3.converter.to_py3_struct(py3_types.Banal, self)

    def _to_py_deprecated(self):
        import importlib
        py_deprecated_types = importlib.import_module("module.ttypes")
        import thrift.util.converter
        return thrift.util.converter.to_py_struct(py_deprecated_types.Banal, self)

# This unfortunately has to be down here to prevent circular imports
import module.thrift_metadata


_fbthrift_all_enums = [
]

def _fbthrift_metadata__exception_Fiery():
    return module.thrift_metadata.gen_metadata_exception_Fiery()
def _fbthrift_metadata__exception_Serious():
    return module.thrift_metadata.gen_metadata_exception_Serious()
def _fbthrift_metadata__exception_ComplexFieldNames():
    return module.thrift_metadata.gen_metadata_exception_ComplexFieldNames()
def _fbthrift_metadata__exception_CustomFieldNames():
    return module.thrift_metadata.gen_metadata_exception_CustomFieldNames()
def _fbthrift_metadata__exception_ExceptionWithPrimitiveField():
    return module.thrift_metadata.gen_metadata_exception_ExceptionWithPrimitiveField()
def _fbthrift_metadata__exception_ExceptionWithStructuredAnnotation():
    return module.thrift_metadata.gen_metadata_exception_ExceptionWithStructuredAnnotation()
def _fbthrift_metadata__exception_Banal():
    return module.thrift_metadata.gen_metadata_exception_Banal()

_fbthrift_all_structs = [
    Fiery,
    Serious,
    ComplexFieldNames,
    CustomFieldNames,
    ExceptionWithPrimitiveField,
    ExceptionWithStructuredAnnotation,
    Banal,
]
_fbthrift_python_types.fill_specs(*_fbthrift_all_structs)



class _fbthrift_Raiser_doBland_args(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
    )


class _fbthrift_Raiser_doBland_result(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
    )


class _fbthrift_Raiser_doRaise_args(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
    )


class _fbthrift_Raiser_doRaise_result(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "b",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Banal),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "f",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Fiery),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            3,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "s",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Serious),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )


class _fbthrift_Raiser_get200_args(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
    )


class _fbthrift_Raiser_get200_result(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
        (
            0,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "success",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )


class _fbthrift_Raiser_get500_args(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
    )


class _fbthrift_Raiser_get500_result(metaclass=_fbthrift_python_types.StructMeta):
    _fbthrift_SPEC = (
        (
            0,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "success",  # name
            _fbthrift_python_types.typeinfo_string,  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            1,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "f",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Fiery),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            2,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "b",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Banal),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
        (
            3,  # id
            _fbthrift_python_types.FieldQualifier.Optional, # qualifier
            "s",  # name
            lambda: _fbthrift_python_types.StructTypeInfo(Serious),  # typeinfo
            None,  # default value
            None,  # adapter info
        ),
    )



_fbthrift_python_types.fill_specs(
    _fbthrift_Raiser_doBland_args,
    _fbthrift_Raiser_doBland_result,
    _fbthrift_Raiser_doRaise_args,
    _fbthrift_Raiser_doRaise_result,
    _fbthrift_Raiser_get200_args,
    _fbthrift_Raiser_get200_result,
    _fbthrift_Raiser_get500_args,
    _fbthrift_Raiser_get500_result,
    
)
