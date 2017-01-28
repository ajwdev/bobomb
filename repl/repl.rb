$:.unshift File.dirname(__FILE__)

require 'pry'
require_relative 'debugger_server_pb'
require_relative 'debugger_server_services_pb'

class PryDebug
  attr_accessor :cursor

  def initialize(host='localhost', port=6502)
    @stub = Debugger::Stub.new("#{host}:#{port}", :this_channel_is_insecure)
    @cursor = 0 # We should ask for the PC
  end

  def ping(msg)
    @stub.ping(PingRequest.new(message: msg))
  end

  def disassemble(addr=nil, count=1)
    addr = @cursor if addr.nil?
    @stub.disassemble(DisassembleRequest.new(address: addr, count: count))
  end

  def stop
    resp = @stub.stop(StopRequest.new)
    @cursor = resp.message.to_i(16)
    resp
  end

  def continue
    @stub.continue(ContinueRequest.new)
  end

  def breakpoint(addresses=[], action=:SET)
    return if addresses.empty?

    @stub.breakpoint(BreakpointRequest.new(addresses: addresses, action: action.to_sym.upcase))
  end
end


command_set = Pry::CommandSet.new do
  # NOTE We intentionally write to stdout instead of the special
  # "output" variable because I dont want to use Ruby's `inspect`
  # call on the output
  command "ping" do |msg|
    puts target_self.ping(msg).message
  end

  command "cursor" do |addr|
    target_self.send(:cursor=, addr.to_i(16))
  end

  command "dis" do |count|
    addr = target_self.cursor
    if count.nil?
      count = 1
    else
      count = count.to_i
    end

    result = target_self.disassemble(addr, count)
    puts result.disassembly.map(&:line)
    # Check if got less instructions that requested. Likely means
    # that we moved out of TEXT section (or whatever the equivalent is)
    if result.length != count
      puts "** #{result.last_error}"
    end
  end

  command "stop" do
    puts target_self.stop.message
  end

  command "c" do
    target_self.continue
  end

  command "break" do |addr|
    target_self.breakpoint([addr.to_i(16)])
  end

  command "clear" do |addr|
    target_self.breakpoint([addr.to_i(16)], :CLEAR)
  end
end

Pry.config.prompt = [proc { 'Bobomb> ' }, proc { '> ' }]

dbg = PryDebug.new
dbg.pry commands: command_set
