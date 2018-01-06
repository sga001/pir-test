#!/usr/bin/env ruby
require 'optparse'

options = {:server_addr => "127.0.0.1:12345", :sys => "seal", :n => 2**16, 
           :d => 2, :a => 16, :t => 20} 

OptionParser.new do |opts|
  opts.banner = "Usage: server_master.rb [options]"

  opts.on("-h", "--host HOST", "specify server addr") do |v|
    options[:server_addr] = v
  end

  opts.on("-s", "--sys SYS", "specify system") do |v|
    options[:sys] = v
  end

  opts.on("-n", "--num NUM", "specify exponent (2^num)") do |v|
    options[:n] = 2**Integer(v)
  end

  opts.on("-d", "--dimension DIM", "specify dimensions") do |v|
    options[:d] = Integer(v)
  end

  opts.on("-a", "--alpha ALPHA", "specify alpha") do |v|
    options[:alpha] = Integer(v)
  end

  opts.on("-t", "--log_plain LOGT", "specify log  of plaintext modulus") do |v|
    options[:t] = Integer(v)
  end

end.parse!


n = options[:n]
server_addr = options[:server_addr]
sys = options[:sys]
d = options[:d]

if sys == "seal"
  t = options[:t]
  puts("./target/release/server_sealpir -h #{server_addr} -n #{n} -d #{d} -t #{t}")
  exec("./target/release/server_sealpir -h #{server_addr} -n #{n} -d #{d} -t #{t}")
elsif sys == "xpir"
  a = options[:a]
  puts("./target/release/server_xpir -h #{server_addr} -n #{n} -d #{d} -a #{a}")
  exec("./target/release/server_xpir -h #{server_addr} -n #{n} -d #{d} -a #{a}")
end
