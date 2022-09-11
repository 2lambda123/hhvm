#
# Autogenerated by Thrift
#
# DO NOT EDIT
#  @generated
#

from __future__ import annotations


import typing as _typing

import enum

import folly.iobuf as _fbthrift_iobuf
import thrift.python.types as _fbthrift_python_types
import thrift.python.exceptions as _fbthrift_python_exceptions

import apache.thrift.type.standard.thrift_types

import facebook.thrift.annotation.cpp.thrift_types

import facebook.thrift.annotation.scope.thrift_types

import facebook.thrift.annotation.thrift.thrift_types


class PatchOp(_fbthrift_python_types.Enum, enum.Enum):
    Assign: PatchOp = ...
    Clear: PatchOp = ...
    PatchPrior: PatchOp = ...
    EnsureUnion: PatchOp = ...
    EnsureStruct: PatchOp = ...
    PatchAfter: PatchOp = ...
    Remove: PatchOp = ...
    Add: PatchOp = ...
    Put: PatchOp = ...
    Unspecified: PatchOp = ...
    def _to_python(self) -> PatchOp: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.PatchOp": ...  # type: ignore
    def _to_py_deprecated(self) -> int: ...


class GeneratePatch(_fbthrift_python_types.Struct):
    def __init__(
        self,
    ) -> None: ...

    def __call__(
        self,
    ) -> GeneratePatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[None]]]: ...
    def _to_python(self) -> GeneratePatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.GeneratePatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.GeneratePatch": ...  # type: ignore


class BoolPatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[bool]] = ...
    clear: _typing.Final[bool] = ...
    invert: _typing.Final[bool] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[bool]=...,
        clear: _typing.Optional[bool]=...,
        invert: _typing.Optional[bool]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[bool]=...,
        clear: _typing.Optional[bool]=...,
        invert: _typing.Optional[bool]=...
    ) -> BoolPatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[bool, bool, bool]]]: ...
    def _to_python(self) -> BoolPatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.BoolPatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.BoolPatch": ...  # type: ignore


class BytePatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[int]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[int] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> BytePatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[int, bool, int]]]: ...
    def _to_python(self) -> BytePatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.BytePatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.BytePatch": ...  # type: ignore


class I16Patch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[int]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[int] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> I16Patch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[int, bool, int]]]: ...
    def _to_python(self) -> I16Patch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.I16Patch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.I16Patch": ...  # type: ignore


class I32Patch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[int]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[int] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> I32Patch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[int, bool, int]]]: ...
    def _to_python(self) -> I32Patch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.I32Patch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.I32Patch": ...  # type: ignore


class I64Patch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[int]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[int] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[int]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[int]=...
    ) -> I64Patch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[int, bool, int]]]: ...
    def _to_python(self) -> I64Patch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.I64Patch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.I64Patch": ...  # type: ignore


class FloatPatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[float]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[float] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[float]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[float]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[float]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[float]=...
    ) -> FloatPatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[float, bool, float]]]: ...
    def _to_python(self) -> FloatPatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.FloatPatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.FloatPatch": ...  # type: ignore


class DoublePatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[float]] = ...
    clear: _typing.Final[bool] = ...
    add: _typing.Final[float] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[float]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[float]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[float]=...,
        clear: _typing.Optional[bool]=...,
        add: _typing.Optional[float]=...
    ) -> DoublePatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[float, bool, float]]]: ...
    def _to_python(self) -> DoublePatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.DoublePatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.DoublePatch": ...  # type: ignore


class StringPatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[str]] = ...
    clear: _typing.Final[bool] = ...
    prepend: _typing.Final[str] = ...
    append: _typing.Final[str] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[str]=...,
        clear: _typing.Optional[bool]=...,
        prepend: _typing.Optional[str]=...,
        append: _typing.Optional[str]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[str]=...,
        clear: _typing.Optional[bool]=...,
        prepend: _typing.Optional[str]=...,
        append: _typing.Optional[str]=...
    ) -> StringPatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[str, bool, str, str]]]: ...
    def _to_python(self) -> StringPatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.StringPatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.StringPatch": ...  # type: ignore


class BinaryPatch(_fbthrift_python_types.Struct):
    assign: _typing.Final[_typing.Optional[_fbthrift_iobuf.IOBuf]] = ...
    clear: _typing.Final[bool] = ...
    prepend: _typing.Final[_fbthrift_iobuf.IOBuf] = ...
    append: _typing.Final[_fbthrift_iobuf.IOBuf] = ...
    def __init__(
        self, *,
        assign: _typing.Optional[_fbthrift_iobuf.IOBuf]=...,
        clear: _typing.Optional[bool]=...,
        prepend: _typing.Optional[_fbthrift_iobuf.IOBuf]=...,
        append: _typing.Optional[_fbthrift_iobuf.IOBuf]=...
    ) -> None: ...

    def __call__(
        self, *,
        assign: _typing.Optional[_fbthrift_iobuf.IOBuf]=...,
        clear: _typing.Optional[bool]=...,
        prepend: _typing.Optional[_fbthrift_iobuf.IOBuf]=...,
        append: _typing.Optional[_fbthrift_iobuf.IOBuf]=...
    ) -> BinaryPatch: ...
    def __iter__(self) -> _typing.Iterator[_typing.Tuple[str, _typing.Union[_fbthrift_iobuf.IOBuf, bool, _fbthrift_iobuf.IOBuf, _fbthrift_iobuf.IOBuf]]]: ...
    def _to_python(self) -> BinaryPatch: ...
    def _to_py3(self) -> "apache.thrift.op.patch.types.BinaryPatch": ...  # type: ignore
    def _to_py_deprecated(self) -> "thrift.lib.thrift.patch.ttypes.BinaryPatch": ...  # type: ignore
