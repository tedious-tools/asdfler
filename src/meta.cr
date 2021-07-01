module Asdfler
  VERSION = "0.1.0"

  class AbortExecution < Exception
    getter status_code

    def initialize(@status_code = 1); end
  end

  def self.process_runner
    ::Process
  end

  class_property logger : Log = begin
    backend_with_formatter = Log::IOBackend.new(formatter: Asdfler::LogFormatter.new)
    Log.setup(:debug, backend_with_formatter)
    Log.for("asdfler")
  end

  {% for msg_type in %w(info debug error) %}
    def self.{{msg_type.id}}(msg : String)
      Asdfler.logger.{{msg_type.id}} { msg }
    end
  {% end %}
end
