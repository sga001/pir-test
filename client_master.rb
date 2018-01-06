#!/usr/bin/env ruby
require 'optparse'

options = {:server_addr => "127.0.0.1:12345", :sys => "seal", :exp => "dc", :n => 2**16, 
           :d => 2, :a => 16, :t => 20, :c => 1, :i => 10} 

OptionParser.new do |opts|
  opts.banner = "Usage: client_master.rb [options]"

  opts.on("-h", "--host ADDR", "specify server addr") do |v|
    options[:server_addr] = v
  end

  opts.on("-s", "--sys SYS", "specify system") do |v|
    options[:sys] = v
  end

  opts.on("-e", "--exp EXP", "specify experiment") do |v|
    options[:exp] = v
  end

  opts.on("-n", "--num NUM", "specify exponent (2^num)") do |v|
    options[:n] = 2**Integer(v)
  end

  opts.on("-d", "--dimension DIM", "specify dimensions") do |v|
    options[:d] = Integer(v)
  end

  opts.on("-a", "--alpha ALPHA", "specify alpha") do |v|
    options[:a] = Integer(v)
  end

  opts.on("-t", "--log_plain LOGT", "specify log of plaintext modulus") do |v|
    options[:t] = Integer(v)
  end

  opts.on("-c", "--threads THREADS", "number of client threads") do |v|
    options[:c] = Integer(v)
  end

  opts.on("-i", "--iterations ITER", "number of iterations") do |v|
    options[:i] = Integer(v)
  end

end.parse!


n = options[:n]
server_addr = options[:server_addr]
sys = options[:sys]
exp = options[:exp]
d = options[:d]
c = options[:c]
i = options[:i]

if exp == "tput"
  exp = "tpu_#{c}"
end

if sys == "seal"
  t = options[:t]
  puts("./target/release/client_sealpir -h #{server_addr} -n #{n} -d #{d} -t #{t} -c #{c} -i #{i} >> #{exp}_#{sys}.log")
  %x(./target/release/client_sealpir -h #{server_addr} -n #{n} -d #{d} -t #{t} -c #{c} -i #{i} >> #{exp}_#{sys}.log)
elsif sys == "xpir"
  a = options[:a]
  puts("./target/release/client_xpir -h #{server_addr} -n #{n} -d #{d} -a #{a} -c #{c} -i #{i} >> #{exp}_#{sys}.log")
  %x(./target/release/client_xpir -h #{server_addr} -n #{n} -d #{d} -a #{a} -c #{c} -i #{i} >> #{exp}_#{sys}.log)
end
