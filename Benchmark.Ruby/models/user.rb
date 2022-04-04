require 'json'

class User
    attr_accessor :id, :firstname, :lastname, :email

    def initialize(id, firstname, lastname, email)
        @id = id
        @firstname = firstname
        @lastname = lastname
        @email = email
    end

    def to_json
        hash = {}
        self.instance_variables.each do |var|
            hash[var.to_s[1..-1]] = self.instance_variable_get var
        end
        hash.to_json
    end
end