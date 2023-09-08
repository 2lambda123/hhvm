#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from __future__ import absolute_import
import sys
from thrift.util.Recursive import fix_spec
from thrift.Thrift import TType, TMessageType, TPriority, TRequestContext, TProcessorEventHandler, TServerInterface, TProcessor, TException, TApplicationException, UnimplementedTypedef
from thrift.protocol.TProtocol import TProtocolException


import includes.ttypes


import pprint
import warnings
from thrift import Thrift
from thrift.transport import TTransport
from thrift.protocol import TBinaryProtocol
from thrift.protocol import TCompactProtocol
from thrift.protocol import THeaderProtocol
fastproto = None
try:
  from thrift.protocol import fastproto
except ImportError:
  pass

def __EXPAND_THRIFT_SPEC(spec):
    next_id = 0
    for item in spec:
        if next_id >= 0 and item[0] < 0:
            next_id = item[0]
        if item[0] != next_id:
            for _ in range(next_id, item[0]):
                yield None
        yield item
        next_id = item[0] + 1

all_structs = []
UTF8STRINGS = bool(0) or sys.version_info.major >= 3

__all__ = ['UTF8STRINGS', 'MyStruct']

class MyStruct:
  r"""
  Attributes:
   - MyIncludedField
   - MyOtherIncludedField
   - MyIncludedInt
  """

  thrift_spec = None
  thrift_field_annotations = None
  thrift_struct_annotations = None
  __init__ = None
  @staticmethod
  def isUnion():
    return False

  def read(self, iprot):
    if (isinstance(iprot, TBinaryProtocol.TBinaryProtocolAccelerated) or (isinstance(iprot, THeaderProtocol.THeaderProtocolAccelerate) and iprot.get_protocol_id() == THeaderProtocol.THeaderProtocol.T_BINARY_PROTOCOL)) and isinstance(iprot.trans, TTransport.CReadableTransport) and self.thrift_spec is not None and fastproto is not None:
      fastproto.decode(self, iprot.trans, [self.__class__, self.thrift_spec, False], utf8strings=UTF8STRINGS, protoid=0)
      return
    if (isinstance(iprot, TCompactProtocol.TCompactProtocolAccelerated) or (isinstance(iprot, THeaderProtocol.THeaderProtocolAccelerate) and iprot.get_protocol_id() == THeaderProtocol.THeaderProtocol.T_COMPACT_PROTOCOL)) and isinstance(iprot.trans, TTransport.CReadableTransport) and self.thrift_spec is not None and fastproto is not None:
      fastproto.decode(self, iprot.trans, [self.__class__, self.thrift_spec, False], utf8strings=UTF8STRINGS, protoid=2)
      return
    iprot.readStructBegin()
    while True:
      (fname, ftype, fid) = iprot.readFieldBegin()
      if ftype == TType.STOP:
        break
      if fid == 1:
        if ftype == TType.STRUCT:
          self.MyIncludedField = includes.ttypes.Included()
          self.MyIncludedField.read(iprot)
        else:
          iprot.skip(ftype)
      elif fid == 2:
        if ftype == TType.STRUCT:
          self.MyOtherIncludedField = includes.ttypes.Included()
          self.MyOtherIncludedField.read(iprot)
        else:
          iprot.skip(ftype)
      elif fid == 3:
        if ftype == TType.I64:
          self.MyIncludedInt = iprot.readI64()
        else:
          iprot.skip(ftype)
      else:
        iprot.skip(ftype)
      iprot.readFieldEnd()
    iprot.readStructEnd()

  def write(self, oprot):
    if (isinstance(oprot, TBinaryProtocol.TBinaryProtocolAccelerated) or (isinstance(oprot, THeaderProtocol.THeaderProtocolAccelerate) and oprot.get_protocol_id() == THeaderProtocol.THeaderProtocol.T_BINARY_PROTOCOL)) and self.thrift_spec is not None and fastproto is not None:
      oprot.trans.write(fastproto.encode(self, [self.__class__, self.thrift_spec, False], utf8strings=UTF8STRINGS, protoid=0))
      return
    if (isinstance(oprot, TCompactProtocol.TCompactProtocolAccelerated) or (isinstance(oprot, THeaderProtocol.THeaderProtocolAccelerate) and oprot.get_protocol_id() == THeaderProtocol.THeaderProtocol.T_COMPACT_PROTOCOL)) and self.thrift_spec is not None and fastproto is not None:
      oprot.trans.write(fastproto.encode(self, [self.__class__, self.thrift_spec, False], utf8strings=UTF8STRINGS, protoid=2))
      return
    oprot.writeStructBegin('MyStruct')
    if self.MyIncludedField != None:
      oprot.writeFieldBegin('MyIncludedField', TType.STRUCT, 1)
      self.MyIncludedField.write(oprot)
      oprot.writeFieldEnd()
    if self.MyOtherIncludedField != None:
      oprot.writeFieldBegin('MyOtherIncludedField', TType.STRUCT, 2)
      self.MyOtherIncludedField.write(oprot)
      oprot.writeFieldEnd()
    if self.MyIncludedInt != None:
      oprot.writeFieldBegin('MyIncludedInt', TType.I64, 3)
      oprot.writeI64(self.MyIncludedInt)
      oprot.writeFieldEnd()
    oprot.writeFieldStop()
    oprot.writeStructEnd()

  def __repr__(self):
    L = []
    padding = ' ' * 4
    if self.MyIncludedField is not None:
      value = pprint.pformat(self.MyIncludedField, indent=0)
      value = padding.join(value.splitlines(True))
      L.append('    MyIncludedField=%s' % (value))
    if self.MyOtherIncludedField is not None:
      value = pprint.pformat(self.MyOtherIncludedField, indent=0)
      value = padding.join(value.splitlines(True))
      L.append('    MyOtherIncludedField=%s' % (value))
    if self.MyIncludedInt is not None:
      value = pprint.pformat(self.MyIncludedInt, indent=0)
      value = padding.join(value.splitlines(True))
      L.append('    MyIncludedInt=%s' % (value))
    return "%s(%s)" % (self.__class__.__name__, "\n" + ",\n".join(L) if L else '')

  def __eq__(self, other):
    if not isinstance(other, self.__class__):
      return False

    return self.__dict__ == other.__dict__ 

  def __ne__(self, other):
    return not (self == other)

  def __dir__(self):
    return (
      'MyIncludedField',
      'MyOtherIncludedField',
      'MyIncludedInt',
    )

  __hash__ = object.__hash__

  def _to_python(self):
    import importlib
    import thrift.python.converter
    python_types = importlib.import_module("module.thrift_types")
    return thrift.python.converter.to_python_struct(python_types.MyStruct, self)

  def _to_py3(self):
    import importlib
    import thrift.py3.converter
    py3_types = importlib.import_module("module.types")
    return thrift.py3.converter.to_py3_struct(py3_types.MyStruct, self)

  def _to_py_deprecated(self):
    return self

all_structs.append(MyStruct)
MyStruct.thrift_spec = tuple(__EXPAND_THRIFT_SPEC((
  (1, TType.STRUCT, 'MyIncludedField', [includes.ttypes.Included, includes.ttypes.Included.thrift_spec, False], includes.ttypes.Included(**{
    "MyIntField" : 2,
    "MyTransitiveField" : transitive.ttypes.Foo(**{
      "a" : 2,
    }),
  }), 2, ), # 1
  (2, TType.STRUCT, 'MyOtherIncludedField', [includes.ttypes.Included, includes.ttypes.Included.thrift_spec, False], None, 2, ), # 2
  (3, TType.I64, 'MyIncludedInt', None, 42, 2, ), # 3
)))

MyStruct.thrift_struct_annotations = {
}
MyStruct.thrift_field_annotations = {
}

def MyStruct__init__(self, MyIncludedField=MyStruct.thrift_spec[1][4], MyOtherIncludedField=None, MyIncludedInt=MyStruct.thrift_spec[3][4],):
  if MyIncludedField is self.thrift_spec[1][4]:
    MyIncludedField = includes.ttypes.Included(**{
    "MyIntField" : 2,
    "MyTransitiveField" : transitive.ttypes.Foo(**{
      "a" : 2,
    }),
  })
  self.MyIncludedField = MyIncludedField
  self.MyOtherIncludedField = MyOtherIncludedField
  if MyIncludedInt is self.thrift_spec[3][4]:
    MyIncludedInt = 42
  self.MyIncludedInt = MyIncludedInt

MyStruct.__init__ = MyStruct__init__

def MyStruct__setstate__(self, state):
  state.setdefault('MyIncludedField', includes.ttypes.Included(**{
    "MyIntField" : 2,
    "MyTransitiveField" : transitive.ttypes.Foo(**{
      "a" : 2,
    }),
  }))
  state.setdefault('MyOtherIncludedField', None)
  state.setdefault('MyIncludedInt', 42)
  self.__dict__ = state

MyStruct.__getstate__ = lambda self: self.__dict__.copy()
MyStruct.__setstate__ = MyStruct__setstate__

fix_spec(all_structs)
del all_structs
