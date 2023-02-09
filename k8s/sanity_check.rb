#!/usr/bin/env ruby

# as we use a distributed cache
# we need to check if invalidation is correct

require 'rest-client'
require 'date'
require 'json'

URL = 'https://birthdays.razum2um.me/hello/username'

def run
  days_until = rand(1..364)
  new_birthday = Date.today + days_until
  birthday_last_year = Date.new(new_birthday.year - 1, new_birthday.month, new_birthday.day)
  
  put = RestClient.put(URL, JSON('dateOfBirth' => birthday_last_year), content_type: :json)
  raise "PUT failed with #{new_birthday}: #{put.body}, #{put.header.inspect}" if put.code != 204
  saved = JSON(RestClient.get(URL, content_type: :json).body).fetch('message')
  if saved != "Hello, username! Your birthday is in #{days_until} day(s)"
    raise "GET bad cache: set=#{birthday_last_year}, need=#{days_until}, got=#{saved}"
  end
  true
end

loop { run && print('.') }

