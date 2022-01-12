require "./definition"

module Asdfler
  class Installer
    DEFAULT_FILEPATH = "./.asdfler.yml"

    getter filepath

    def initialize(@filepath : String); end

    def install_plugins
      # Find asdfler file
      unless File.exists?(filepath)
        Asdfler.error("File #{filepath} not found")
        raise Asdfler::AbortExecution.new
      end

      begin
        definition = Definition.from_yaml(File.read(filepath))
      rescue ex : YAML::ParseException
        Asdfler.error("Failed to parse YAML. Full message: \n #{ex.message}")
        raise Asdfler::AbortExecution.new
      end

      definition.plugins.each do |plugin|
        context_str = "[[#{plugin.name}]]:"

        Asdfler.info("=====")
        Asdfler.info("#{context_str} installing")
        Asdfler.info("=====")

        mem = IO::Memory.new

        add_plugin_result = Asdfler.process_runner.run(
          command: asdf_path,
          args: ["plugin", "add", plugin.name],
          shell: true,
          input: Process::Redirect::Inherit,
          output: mem,
          error: mem
        )

        case add_plugin_result.exit_code
        when 0 then Asdfler.info("#{context_str} successfully installed")
        when 2 then Asdfler.debug("#{context_str} already installed")
        else
          Asdfler.error("#{context_str} failed to install:\n#{mem}\n")
          raise Asdfler::AbortExecution.new # Not sure I wanna bail this hard, we'll see
        end

        versions_to_install = [plugin.default_version, *(plugin.versions || Array(String?).new)].compact.uniq

        versions_to_install.each do |version_to_install|
          Asdfler.info("#{context_str} installing #{version_to_install}...")

          mem = IO::Memory.new

          result = Asdfler.process_runner.run(
            command: asdf_path,
            args: ["install", plugin.name, version_to_install],
            shell: true,
            input: Process::Redirect::Inherit,
            output: mem,
            error: mem
          )

          if result.success?
            Asdfler.info("#{context_str} version #{version_to_install} is available")

            if version_to_install == plugin.default_version
              result = Asdfler.process_runner.run(
                command: asdf_path,
                args: ["global", plugin.name, version_to_install],
                shell: true,
              )

              if result.success?
                Asdfler.info("#{context_str} Set ~/.tool-versions version to #{version_to_install}")
              else
                Asdfler.error("#{context_str} Unable to set ~/.tool-versions version")
              end
            end
          else
            Asdfler.error("#{context_str} could not install version #{version_to_install}\n#{mem}\n")
          end
        end
      
        Asdfler.trace("")
        Asdfler.trace("")
      end
    end

    private def asdf_path
      "asdf"
    end
  end
end
