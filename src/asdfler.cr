require "admiral"
require "log"

require "./asdfler/log_formatter"
require "./asdfler/installer"

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

  class CLI < Admiral::Command
    define_help description: "Entrypoint to asdfler for managing asdf plugins"

    class Install < Admiral::Command
      define_help description: "Uses a .asdfler.yml file to install plugins and default versions"

      def run
        Asdfler::Installer.new.install_plugins
      end
    end

    register_sub_command install : Install, description: "Installs user-defined asdf plugins"

    def run
      puts help
    end
  end
  # TODO: Put your code here
end

begin
  Asdfler::CLI.run
rescue ex : Asdfler::AbortExecution
  exit(ex.status_code)
end
