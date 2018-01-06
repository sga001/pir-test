#!/usr/bin/env ruby
require 'optparse'
require 'json'

options = {:server_addr => "127.0.0.1", :exp => "dc", :n => 2**16, 
           :c => 1, :i => 10} 

OptionParser.new do |opts|
  opts.banner = "Usage: scp_master.rb [options]"

  opts.on("-h", "--host ADDR", "specify server addr") do |v|
    options[:server_addr] = v
  end

  opts.on("-e", "--exp EXP", "specify experiment") do |v|
    options[:exp] = v
  end

  opts.on("-n", "--num NUM", "specify exponent (2^num)") do |v|
    options[:n] = 2**Integer(v)
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
exp = options[:exp]
c = options[:c]
i = options[:i]

if exp == "tput"
  exp = "tpu_#{c}"
end

date = Time.now.utc

results = []
(0...i).each do |it|
  start = Time.now
  %x(scp #{server_addr}:/home/pir-test/scp_files/#{n} ./)
  finish = Time.now
  results.push((finish-start) * 1000000) # microseconds
  %x(rm #{n})
end

json_obj = {"date" => date, "num" => n, "i" => i, "point" => results}

f = File.open("#{exp}_scp.log", "a")
f.puts(json_obj.to_json())
f.close()
