require "log"

class Asdfler::LogFormatter
  include Log::Formatter

  def initialize
  end

  def format(entry : Log::Entry, io : IO)
    str = entry.message

    case entry.severity
    when .debug? then io << "\033[0;33mDEBUG: #{str}\033[0m"    # Yellow
    when .info?  then io << "\033[0;34mINFO:  #{str}\033[0m"    # Blue
    when .error? then io << "\033[0;31m===> ERR: #{str}\033[0m" # Red
    else
      io << str # Plain, fallthrough for safety
    end
  end
end
