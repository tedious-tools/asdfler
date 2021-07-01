require "yaml"

module Asdfler
  class Definition
    include YAML::Serializable
    include YAML::Serializable::Strict

    class PluginDefinition
      include YAML::Serializable
      include YAML::Serializable::Strict

      property! name : String
      property default_version : String?
    end

    property! plugins : Array(PluginDefinition)
  end
end
