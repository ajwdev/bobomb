# Generated by the protocol buffer compiler.  DO NOT EDIT!
# Source: debugger_server.proto for package ''

require 'grpc'
require 'debugger_server_pb'

module Debugger
  class Service

    include GRPC::GenericService

    self.marshal_class_method = :encode
    self.unmarshal_class_method = :decode
    self.service_name = 'Debugger'

    rpc :Ping, PingRequest, OkReply
    rpc :Stop, StopRequest, OkReply
    rpc :Continue, ContinueRequest, OkReply
    rpc :Disassemble, DisassembleRequest, DisassembleReply
  end

  Stub = Service.rpc_stub_class
end
