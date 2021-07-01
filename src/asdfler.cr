require "admiral"
require "log"

require "./asdfler/log_formatter"
require "./asdfler/installer"
require "./meta"

module Asdfler
  class CLI < Admiral::Command
    class Install < Admiral::Command
      define_help description: "Uses a .asdfler.yml file to install plugins and default versions"
      define_flag path : String,
        description: "Path where your asdfler config lives. Must be valid YAML or JSON (given simple JSON is a superset of YAML).",
        default: ".asdfler.yml",
        short: p

      def run
        Asdfler::Installer.new(filepath: File.expand_path(flags.path)).install_plugins
      end
    end

    define_help description: "Entrypoint to asdfler for managing asdf plugins"
    register_sub_command install : Install, description: "Installs user-defined asdf plugins"

    def run
      puts help
    end
  end
end

begin
  Asdfler::CLI.run
rescue ex : Asdfler::AbortExecution
  exit(ex.status_code)
end
