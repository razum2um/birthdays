FROM ruby:3.1

RUN gem install rest-client
COPY ./sanity_check.rb /sanity_check.rb

CMD ['/sanity_check.rb']