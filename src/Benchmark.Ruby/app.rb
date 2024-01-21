require 'sinatra'
require 'json'
require 'securerandom'
require './models/user.rb'

set :port, 8080

before do
    content_type 'application/json'
end

get '/' do
    { name: 'Benchmark.Ruby' }.to_json
end

get '/api/user' do
    user = User.new(SecureRandom.uuid, "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com")
    user.to_json
end
