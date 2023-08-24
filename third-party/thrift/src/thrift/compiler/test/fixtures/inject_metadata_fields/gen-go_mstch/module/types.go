// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]

import (
    "fmt"
    "strings"

    foo "foo"
    thrift "github.com/facebook/fbthrift/thrift/lib/go/thrift"
)

var _ = foo.GoUnusedProtection__

// (needed to ensure safety because of naive import list construction)
var _ = fmt.Printf
var _ = thrift.ZERO
var _ = strings.Split


type Fields struct {
    InjectedField string `thrift:"injected_field,100" json:"injected_field" db:"injected_field"`
}
// Compile time interface enforcer
var _ thrift.Struct = &Fields{}

func NewFields() *Fields {
    return (&Fields{}).
        SetInjectedFieldNonCompat("")
}

func (x *Fields) GetInjectedFieldNonCompat() string {
    return x.InjectedField
}

func (x *Fields) GetInjectedField() string {
    return x.InjectedField
}

func (x *Fields) SetInjectedFieldNonCompat(value string) *Fields {
    x.InjectedField = value
    return x
}

func (x *Fields) SetInjectedField(value string) *Fields {
    x.InjectedField = value
    return x
}

func (x *Fields) writeField100(p thrift.Protocol) error {  // InjectedField
    if err := p.WriteFieldBegin("injected_field", thrift.STRING, 100); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetInjectedFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Fields) readField100(p thrift.Protocol) error {  // InjectedField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedFieldNonCompat(result)
    return nil
}

func (x *Fields) toString100() string {  // InjectedField
    return fmt.Sprintf("%v", x.GetInjectedFieldNonCompat())
}


// Deprecated: Use Fields.Set* methods instead or set the fields directly.
type FieldsBuilder struct {
    obj *Fields
}

func NewFieldsBuilder() *FieldsBuilder {
    return &FieldsBuilder{
        obj: NewFields(),
    }
}

func (x *FieldsBuilder) InjectedField(value string) *FieldsBuilder {
    x.obj.InjectedField = value
    return x
}

func (x *FieldsBuilder) Emit() *Fields {
    var objCopy Fields = *x.obj
    return &objCopy
}

func (x *Fields) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("Fields"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField100(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *Fields) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, typ, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if typ == thrift.STOP {
            break;
        }

        switch id {
        case 100:  // injected_field
            if err := x.readField100(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(typ); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *Fields) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("Fields({")
    sb.WriteString(fmt.Sprintf("InjectedField:%s", x.toString100()))
    sb.WriteString("})")

    return sb.String()
}

type FieldsInjectedToEmptyStruct struct {
    InjectedField string `thrift:"injected_field,-1100" json:"injected_field" db:"injected_field"`
}
// Compile time interface enforcer
var _ thrift.Struct = &FieldsInjectedToEmptyStruct{}

func NewFieldsInjectedToEmptyStruct() *FieldsInjectedToEmptyStruct {
    return (&FieldsInjectedToEmptyStruct{}).
        SetInjectedFieldNonCompat("")
}

func (x *FieldsInjectedToEmptyStruct) GetInjectedFieldNonCompat() string {
    return x.InjectedField
}

func (x *FieldsInjectedToEmptyStruct) GetInjectedField() string {
    return x.InjectedField
}

func (x *FieldsInjectedToEmptyStruct) SetInjectedFieldNonCompat(value string) *FieldsInjectedToEmptyStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedToEmptyStruct) SetInjectedField(value string) *FieldsInjectedToEmptyStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedToEmptyStruct) writeField_1100(p thrift.Protocol) error {  // InjectedField
    if err := p.WriteFieldBegin("injected_field", thrift.STRING, -1100); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetInjectedFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedToEmptyStruct) readField_1100(p thrift.Protocol) error {  // InjectedField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedToEmptyStruct) toString_1100() string {  // InjectedField
    return fmt.Sprintf("%v", x.GetInjectedFieldNonCompat())
}


// Deprecated: Use FieldsInjectedToEmptyStruct.Set* methods instead or set the fields directly.
type FieldsInjectedToEmptyStructBuilder struct {
    obj *FieldsInjectedToEmptyStruct
}

func NewFieldsInjectedToEmptyStructBuilder() *FieldsInjectedToEmptyStructBuilder {
    return &FieldsInjectedToEmptyStructBuilder{
        obj: NewFieldsInjectedToEmptyStruct(),
    }
}

func (x *FieldsInjectedToEmptyStructBuilder) InjectedField(value string) *FieldsInjectedToEmptyStructBuilder {
    x.obj.InjectedField = value
    return x
}

func (x *FieldsInjectedToEmptyStructBuilder) Emit() *FieldsInjectedToEmptyStruct {
    var objCopy FieldsInjectedToEmptyStruct = *x.obj
    return &objCopy
}

func (x *FieldsInjectedToEmptyStruct) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("FieldsInjectedToEmptyStruct"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField_1100(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedToEmptyStruct) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, typ, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if typ == thrift.STOP {
            break;
        }

        switch id {
        case -1100:  // injected_field
            if err := x.readField_1100(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(typ); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *FieldsInjectedToEmptyStruct) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("FieldsInjectedToEmptyStruct({")
    sb.WriteString(fmt.Sprintf("InjectedField:%s", x.toString_1100()))
    sb.WriteString("})")

    return sb.String()
}

type FieldsInjectedToStruct struct {
    StringField string `thrift:"string_field,1" json:"string_field" db:"string_field"`
    InjectedField string `thrift:"injected_field,-1100" json:"injected_field" db:"injected_field"`
}
// Compile time interface enforcer
var _ thrift.Struct = &FieldsInjectedToStruct{}

func NewFieldsInjectedToStruct() *FieldsInjectedToStruct {
    return (&FieldsInjectedToStruct{}).
        SetStringFieldNonCompat("").
        SetInjectedFieldNonCompat("")
}

func (x *FieldsInjectedToStruct) GetStringFieldNonCompat() string {
    return x.StringField
}

func (x *FieldsInjectedToStruct) GetStringField() string {
    return x.StringField
}

func (x *FieldsInjectedToStruct) GetInjectedFieldNonCompat() string {
    return x.InjectedField
}

func (x *FieldsInjectedToStruct) GetInjectedField() string {
    return x.InjectedField
}

func (x *FieldsInjectedToStruct) SetStringFieldNonCompat(value string) *FieldsInjectedToStruct {
    x.StringField = value
    return x
}

func (x *FieldsInjectedToStruct) SetStringField(value string) *FieldsInjectedToStruct {
    x.StringField = value
    return x
}

func (x *FieldsInjectedToStruct) SetInjectedFieldNonCompat(value string) *FieldsInjectedToStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedToStruct) SetInjectedField(value string) *FieldsInjectedToStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedToStruct) writeField1(p thrift.Protocol) error {  // StringField
    if err := p.WriteFieldBegin("string_field", thrift.STRING, 1); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetStringFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedToStruct) writeField_1100(p thrift.Protocol) error {  // InjectedField
    if err := p.WriteFieldBegin("injected_field", thrift.STRING, -1100); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetInjectedFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedToStruct) readField1(p thrift.Protocol) error {  // StringField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetStringFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedToStruct) readField_1100(p thrift.Protocol) error {  // InjectedField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedToStruct) toString1() string {  // StringField
    return fmt.Sprintf("%v", x.GetStringFieldNonCompat())
}

func (x *FieldsInjectedToStruct) toString_1100() string {  // InjectedField
    return fmt.Sprintf("%v", x.GetInjectedFieldNonCompat())
}


// Deprecated: Use FieldsInjectedToStruct.Set* methods instead or set the fields directly.
type FieldsInjectedToStructBuilder struct {
    obj *FieldsInjectedToStruct
}

func NewFieldsInjectedToStructBuilder() *FieldsInjectedToStructBuilder {
    return &FieldsInjectedToStructBuilder{
        obj: NewFieldsInjectedToStruct(),
    }
}

func (x *FieldsInjectedToStructBuilder) StringField(value string) *FieldsInjectedToStructBuilder {
    x.obj.StringField = value
    return x
}

func (x *FieldsInjectedToStructBuilder) InjectedField(value string) *FieldsInjectedToStructBuilder {
    x.obj.InjectedField = value
    return x
}

func (x *FieldsInjectedToStructBuilder) Emit() *FieldsInjectedToStruct {
    var objCopy FieldsInjectedToStruct = *x.obj
    return &objCopy
}

func (x *FieldsInjectedToStruct) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("FieldsInjectedToStruct"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField1(p); err != nil {
        return err
    }

    if err := x.writeField_1100(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedToStruct) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, typ, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if typ == thrift.STOP {
            break;
        }

        switch id {
        case 1:  // string_field
            if err := x.readField1(p); err != nil {
                return err
            }
        case -1100:  // injected_field
            if err := x.readField_1100(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(typ); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *FieldsInjectedToStruct) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("FieldsInjectedToStruct({")
    sb.WriteString(fmt.Sprintf("StringField:%s ", x.toString1()))
    sb.WriteString(fmt.Sprintf("InjectedField:%s", x.toString_1100()))
    sb.WriteString("})")

    return sb.String()
}

type FieldsInjectedWithIncludedStruct struct {
    StringField string `thrift:"string_field,1" json:"string_field" db:"string_field"`
    InjectedField string `thrift:"injected_field,-1100" json:"injected_field" db:"injected_field"`
    InjectedStructuredAnnotationField *string `thrift:"injected_structured_annotation_field,-1101,optional" json:"injected_structured_annotation_field,omitempty" db:"injected_structured_annotation_field"`
    InjectedUnstructuredAnnotationField *string `thrift:"injected_unstructured_annotation_field,-1102,optional" json:"injected_unstructured_annotation_field,omitempty" db:"injected_unstructured_annotation_field"`
}
// Compile time interface enforcer
var _ thrift.Struct = &FieldsInjectedWithIncludedStruct{}

func NewFieldsInjectedWithIncludedStruct() *FieldsInjectedWithIncludedStruct {
    return (&FieldsInjectedWithIncludedStruct{}).
        SetStringFieldNonCompat("").
        SetInjectedFieldNonCompat("")
}

func (x *FieldsInjectedWithIncludedStruct) GetStringFieldNonCompat() string {
    return x.StringField
}

func (x *FieldsInjectedWithIncludedStruct) GetStringField() string {
    return x.StringField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedFieldNonCompat() string {
    return x.InjectedField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedField() string {
    return x.InjectedField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedStructuredAnnotationFieldNonCompat() *string {
    return x.InjectedStructuredAnnotationField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedStructuredAnnotationField() string {
    if !x.IsSetInjectedStructuredAnnotationField() {
        return ""
    }

    return *x.InjectedStructuredAnnotationField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedUnstructuredAnnotationFieldNonCompat() *string {
    return x.InjectedUnstructuredAnnotationField
}

func (x *FieldsInjectedWithIncludedStruct) GetInjectedUnstructuredAnnotationField() string {
    if !x.IsSetInjectedUnstructuredAnnotationField() {
        return ""
    }

    return *x.InjectedUnstructuredAnnotationField
}

func (x *FieldsInjectedWithIncludedStruct) SetStringFieldNonCompat(value string) *FieldsInjectedWithIncludedStruct {
    x.StringField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetStringField(value string) *FieldsInjectedWithIncludedStruct {
    x.StringField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedFieldNonCompat(value string) *FieldsInjectedWithIncludedStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedField(value string) *FieldsInjectedWithIncludedStruct {
    x.InjectedField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedStructuredAnnotationFieldNonCompat(value string) *FieldsInjectedWithIncludedStruct {
    x.InjectedStructuredAnnotationField = &value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedStructuredAnnotationField(value *string) *FieldsInjectedWithIncludedStruct {
    x.InjectedStructuredAnnotationField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedUnstructuredAnnotationFieldNonCompat(value string) *FieldsInjectedWithIncludedStruct {
    x.InjectedUnstructuredAnnotationField = &value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) SetInjectedUnstructuredAnnotationField(value *string) *FieldsInjectedWithIncludedStruct {
    x.InjectedUnstructuredAnnotationField = value
    return x
}

func (x *FieldsInjectedWithIncludedStruct) IsSetInjectedStructuredAnnotationField() bool {
    return x.InjectedStructuredAnnotationField != nil
}

func (x *FieldsInjectedWithIncludedStruct) IsSetInjectedUnstructuredAnnotationField() bool {
    return x.InjectedUnstructuredAnnotationField != nil
}

func (x *FieldsInjectedWithIncludedStruct) writeField1(p thrift.Protocol) error {  // StringField
    if err := p.WriteFieldBegin("string_field", thrift.STRING, 1); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetStringFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) writeField_1100(p thrift.Protocol) error {  // InjectedField
    if err := p.WriteFieldBegin("injected_field", thrift.STRING, -1100); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetInjectedFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) writeField_1101(p thrift.Protocol) error {  // InjectedStructuredAnnotationField
    if !x.IsSetInjectedStructuredAnnotationField() {
        return nil
    }

    if err := p.WriteFieldBegin("injected_structured_annotation_field", thrift.STRING, -1101); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := *x.GetInjectedStructuredAnnotationFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) writeField_1102(p thrift.Protocol) error {  // InjectedUnstructuredAnnotationField
    if !x.IsSetInjectedUnstructuredAnnotationField() {
        return nil
    }

    if err := p.WriteFieldBegin("injected_unstructured_annotation_field", thrift.STRING, -1102); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := *x.GetInjectedUnstructuredAnnotationFieldNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) readField1(p thrift.Protocol) error {  // StringField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetStringFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) readField_1100(p thrift.Protocol) error {  // InjectedField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) readField_1101(p thrift.Protocol) error {  // InjectedStructuredAnnotationField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedStructuredAnnotationFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) readField_1102(p thrift.Protocol) error {  // InjectedUnstructuredAnnotationField
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetInjectedUnstructuredAnnotationFieldNonCompat(result)
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) toString1() string {  // StringField
    return fmt.Sprintf("%v", x.GetStringFieldNonCompat())
}

func (x *FieldsInjectedWithIncludedStruct) toString_1100() string {  // InjectedField
    return fmt.Sprintf("%v", x.GetInjectedFieldNonCompat())
}

func (x *FieldsInjectedWithIncludedStruct) toString_1101() string {  // InjectedStructuredAnnotationField
    if x.IsSetInjectedStructuredAnnotationField() {
        return fmt.Sprintf("%v", *x.GetInjectedStructuredAnnotationFieldNonCompat())
    }
    return fmt.Sprintf("%v", x.GetInjectedStructuredAnnotationFieldNonCompat())
}

func (x *FieldsInjectedWithIncludedStruct) toString_1102() string {  // InjectedUnstructuredAnnotationField
    if x.IsSetInjectedUnstructuredAnnotationField() {
        return fmt.Sprintf("%v", *x.GetInjectedUnstructuredAnnotationFieldNonCompat())
    }
    return fmt.Sprintf("%v", x.GetInjectedUnstructuredAnnotationFieldNonCompat())
}

// Deprecated: Use NewFieldsInjectedWithIncludedStruct().GetInjectedStructuredAnnotationField() instead.
var FieldsInjectedWithIncludedStruct_InjectedStructuredAnnotationField_DEFAULT = NewFieldsInjectedWithIncludedStruct().GetInjectedStructuredAnnotationField()

// Deprecated: Use NewFieldsInjectedWithIncludedStruct().GetInjectedUnstructuredAnnotationField() instead.
var FieldsInjectedWithIncludedStruct_InjectedUnstructuredAnnotationField_DEFAULT = NewFieldsInjectedWithIncludedStruct().GetInjectedUnstructuredAnnotationField()


// Deprecated: Use FieldsInjectedWithIncludedStruct.Set* methods instead or set the fields directly.
type FieldsInjectedWithIncludedStructBuilder struct {
    obj *FieldsInjectedWithIncludedStruct
}

func NewFieldsInjectedWithIncludedStructBuilder() *FieldsInjectedWithIncludedStructBuilder {
    return &FieldsInjectedWithIncludedStructBuilder{
        obj: NewFieldsInjectedWithIncludedStruct(),
    }
}

func (x *FieldsInjectedWithIncludedStructBuilder) StringField(value string) *FieldsInjectedWithIncludedStructBuilder {
    x.obj.StringField = value
    return x
}

func (x *FieldsInjectedWithIncludedStructBuilder) InjectedField(value string) *FieldsInjectedWithIncludedStructBuilder {
    x.obj.InjectedField = value
    return x
}

func (x *FieldsInjectedWithIncludedStructBuilder) InjectedStructuredAnnotationField(value *string) *FieldsInjectedWithIncludedStructBuilder {
    x.obj.InjectedStructuredAnnotationField = value
    return x
}

func (x *FieldsInjectedWithIncludedStructBuilder) InjectedUnstructuredAnnotationField(value *string) *FieldsInjectedWithIncludedStructBuilder {
    x.obj.InjectedUnstructuredAnnotationField = value
    return x
}

func (x *FieldsInjectedWithIncludedStructBuilder) Emit() *FieldsInjectedWithIncludedStruct {
    var objCopy FieldsInjectedWithIncludedStruct = *x.obj
    return &objCopy
}

func (x *FieldsInjectedWithIncludedStruct) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("FieldsInjectedWithIncludedStruct"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField1(p); err != nil {
        return err
    }

    if err := x.writeField_1100(p); err != nil {
        return err
    }

    if err := x.writeField_1101(p); err != nil {
        return err
    }

    if err := x.writeField_1102(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *FieldsInjectedWithIncludedStruct) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, typ, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if typ == thrift.STOP {
            break;
        }

        switch id {
        case 1:  // string_field
            if err := x.readField1(p); err != nil {
                return err
            }
        case -1100:  // injected_field
            if err := x.readField_1100(p); err != nil {
                return err
            }
        case -1101:  // injected_structured_annotation_field
            if err := x.readField_1101(p); err != nil {
                return err
            }
        case -1102:  // injected_unstructured_annotation_field
            if err := x.readField_1102(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(typ); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *FieldsInjectedWithIncludedStruct) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("FieldsInjectedWithIncludedStruct({")
    sb.WriteString(fmt.Sprintf("StringField:%s ", x.toString1()))
    sb.WriteString(fmt.Sprintf("InjectedField:%s ", x.toString_1100()))
    sb.WriteString(fmt.Sprintf("InjectedStructuredAnnotationField:%s ", x.toString_1101()))
    sb.WriteString(fmt.Sprintf("InjectedUnstructuredAnnotationField:%s", x.toString_1102()))
    sb.WriteString("})")

    return sb.String()
}

// RegisterTypes registers types found in this file that have a thrift_uri with the passed in registry.
func RegisterTypes(registry interface {
	  RegisterType(name string, obj any)
}) {

}
