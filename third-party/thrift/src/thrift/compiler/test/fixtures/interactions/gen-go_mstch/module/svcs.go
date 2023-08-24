// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]


import (
    "context"
    "fmt"
    "strings"
    "sync"


    "thrift/lib/go/thrift"
)


// (needed to ensure safety because of naive import list construction)
var _ = context.Background
var _ = fmt.Printf
var _ = thrift.ZERO
var _ = strings.Split
var _ = sync.Mutex{}



type MyService interface {
    Foo(ctx context.Context) (error)
}

// Deprecated: Use MyService instead.
type MyServiceClientInterface interface {
    thrift.ClientInterface
    Foo() (error)
}

type MyServiceChannelClient struct {
    ch thrift.RequestChannel
}
// Compile time interface enforcer
var _ MyService = &MyServiceChannelClient{}

func NewMyServiceChannelClient(channel thrift.RequestChannel) *MyServiceChannelClient {
    return &MyServiceChannelClient{
        ch: channel,
    }
}

func (c *MyServiceChannelClient) Close() error {
    return c.ch.Close()
}

func (c *MyServiceChannelClient) IsOpen() bool {
    return c.ch.IsOpen()
}

func (c *MyServiceChannelClient) Open() error {
    return c.ch.Open()
}

// Deprecated: Use MyServiceChannelClient instead.
type MyServiceClient struct {
    chClient *MyServiceChannelClient
    Mu       sync.Mutex
}
// Compile time interface enforcer
var _ MyServiceClientInterface = &MyServiceClient{}

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceClient(t thrift.Transport, iprot thrift.Protocol, oprot thrift.Protocol) *MyServiceClient {
    return &MyServiceClient{
        chClient: NewMyServiceChannelClient(
            thrift.NewSerialChannel(iprot),
        ),
    }
}

func (c *MyServiceClient) Close() error {
    return c.chClient.Close()
}

func (c *MyServiceClient) IsOpen() bool {
    return c.chClient.IsOpen()
}

func (c *MyServiceClient) Open() error {
    return c.chClient.Open()
}

// Deprecated: Use MyServiceChannelClient instead.
type MyServiceThreadsafeClient = MyServiceClient

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceThreadsafeClient(t thrift.Transport, iprot thrift.Protocol, oprot thrift.Protocol) *MyServiceThreadsafeClient {
    return NewMyServiceClient(t, iprot, oprot)
}

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceClientProtocol(prot thrift.Protocol) *MyServiceClient {
  return NewMyServiceClient(prot.Transport(), prot, prot)
}

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceThreadsafeClientProtocol(prot thrift.Protocol) *MyServiceClient {
  return NewMyServiceClient(prot.Transport(), prot, prot)
}

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceClientFactory(t thrift.Transport, pf thrift.ProtocolFactory) *MyServiceClient {
  iprot := pf.GetProtocol(t)
  oprot := pf.GetProtocol(t)
  return NewMyServiceClient(t, iprot, oprot)
}

// Deprecated: Use NewMyServiceChannelClient() instead.
func NewMyServiceThreadsafeClientFactory(t thrift.Transport, pf thrift.ProtocolFactory) *MyServiceThreadsafeClient {
  return NewMyServiceClientFactory(t, pf)
}


func (c *MyServiceChannelClient) Foo(ctx context.Context) (error) {
    in := &reqMyServiceFoo{
    }
    out := newRespMyServiceFoo()
    err := c.ch.Call(ctx, "foo", in, out)
    if err != nil {
        return err
    }
    return nil
}

func (c *MyServiceClient) Foo() (error) {
    return c.chClient.Foo(nil)
}


type reqMyServiceFoo struct {
}
// Compile time interface enforcer
var _ thrift.Struct = &reqMyServiceFoo{}

type MyServiceFooArgs = reqMyServiceFoo

func newReqMyServiceFoo() *reqMyServiceFoo {
    return (&reqMyServiceFoo{})
}


// Deprecated: Use reqMyServiceFoo.Set* methods instead or set the fields directly.
type reqMyServiceFooBuilder struct {
    obj *reqMyServiceFoo
}

func newReqMyServiceFooBuilder() *reqMyServiceFooBuilder {
    return &reqMyServiceFooBuilder{
        obj: newReqMyServiceFoo(),
    }
}

func (x *reqMyServiceFooBuilder) Emit() *reqMyServiceFoo {
    var objCopy reqMyServiceFoo = *x.obj
    return &objCopy
}

func (x *reqMyServiceFoo) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("reqMyServiceFoo"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *reqMyServiceFoo) Read(p thrift.Protocol) error {
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

func (x *reqMyServiceFoo) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("reqMyServiceFoo({")
    sb.WriteString("})")

    return sb.String()
}
type respMyServiceFoo struct {
}
// Compile time interface enforcer
var _ thrift.Struct = &respMyServiceFoo{}
var _ thrift.WritableResult = &respMyServiceFoo{}

func newRespMyServiceFoo() *respMyServiceFoo {
    return (&respMyServiceFoo{})
}


// Deprecated: Use respMyServiceFoo.Set* methods instead or set the fields directly.
type respMyServiceFooBuilder struct {
    obj *respMyServiceFoo
}

func newRespMyServiceFooBuilder() *respMyServiceFooBuilder {
    return &respMyServiceFooBuilder{
        obj: newRespMyServiceFoo(),
    }
}

func (x *respMyServiceFooBuilder) Emit() *respMyServiceFoo {
    var objCopy respMyServiceFoo = *x.obj
    return &objCopy
}

func (x *respMyServiceFoo) Exception() thrift.WritableException {
    return nil
}

func (x *respMyServiceFoo) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("respMyServiceFoo"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *respMyServiceFoo) Read(p thrift.Protocol) error {
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

func (x *respMyServiceFoo) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("respMyServiceFoo({")
    sb.WriteString("})")

    return sb.String()
}


type MyServiceProcessor struct {
    processorMap       map[string]thrift.ProcessorFunction
    functionServiceMap map[string]string
    handler            MyService
}
// Compile time interface enforcer
var _ thrift.Processor = &MyServiceProcessor{}

func (p *MyServiceProcessor) AddToProcessorMap(key string, processor thrift.ProcessorFunction) {
    p.processorMap[key] = processor
}

func (p *MyServiceProcessor) AddToFunctionServiceMap(key, service string) {
    p.functionServiceMap[key] = service
}

func (p *MyServiceProcessor) GetProcessorFunction(key string) (processor thrift.ProcessorFunction, err error) {
    if processor, ok := p.processorMap[key]; ok {
        return processor, nil
    }
    return nil, nil
}

func (p *MyServiceProcessor) ProcessorMap() map[string]thrift.ProcessorFunction {
    return p.processorMap
}

func (p *MyServiceProcessor) FunctionServiceMap() map[string]string {
    return p.functionServiceMap
}

func NewMyServiceProcessor(handler MyService) *MyServiceProcessor {
    p := &MyServiceProcessor{
        handler:            handler,
        processorMap:       make(map[string]thrift.ProcessorFunction),
        functionServiceMap: make(map[string]string),
    }
    p.AddToProcessorMap("foo", &procFuncMyServiceFoo{handler: handler})
    p.AddToFunctionServiceMap("foo", "MyService")

    return p
}


type procFuncMyServiceFoo struct {
    handler MyService
}
// Compile time interface enforcer
var _ thrift.ProcessorFunction = &procFuncMyServiceFoo{}

func (p *procFuncMyServiceFoo) Read(iprot thrift.Protocol) (thrift.Struct, thrift.Exception) {
    args := newReqMyServiceFoo()
    if err := args.Read(iprot); err != nil {
        return nil, err
    }
    iprot.ReadMessageEnd()
    return args, nil
}

func (p *procFuncMyServiceFoo) Write(seqId int32, result thrift.WritableStruct, oprot thrift.Protocol) (err thrift.Exception) {
    var err2 error
    messageType := thrift.REPLY
    switch result.(type) {
    case thrift.ApplicationException:
        messageType = thrift.EXCEPTION
    }

    if err2 = oprot.WriteMessageBegin("foo", messageType, seqId); err2 != nil {
        err = err2
    }
    if err2 = result.Write(oprot); err == nil && err2 != nil {
        err = err2
    }
    if err2 = oprot.WriteMessageEnd(); err == nil && err2 != nil {
        err = err2
    }
    if err2 = oprot.Flush(); err == nil && err2 != nil {
        err = err2
    }
    return err
}

func (p *procFuncMyServiceFoo) Run(reqStruct thrift.Struct) (thrift.WritableStruct, thrift.ApplicationException) {
    result := newRespMyServiceFoo()
    err := p.handler.Foo()
    if err != nil {
        x := thrift.NewApplicationExceptionCause(thrift.INTERNAL_ERROR, "Internal error processing Foo: " + err.Error(), err)
        return x, x
    }

    return result, nil
}


