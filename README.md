# asdfler

Manage which [`asdf`](https://asdf-vm.com/#/core-manage-asdf) plugins you have as well as default versions!

NOTE: SUPER UNTESTED AND THROWN TOGETHER IN A COUPLE OF HOURS. **Use at your own risk**

## Installation

### Homebrew

```bash
brew install tedious-tools/formulae/asdfler
```

### Non-Homebrew

1. Install Crystal compiler
2. `git clone https://github.com/lirossarvet/asdfler.git`
3. `cd asdfler`
4. `shards build --release`
5. `mv bin/asdfler <somewhere in your path like ~/bin>/asdfler`

```yaml
- name: crystal
  default_version: 1.0.0
```
to your shiny new `.asdfler.yml` file and run `asdfler install` :D

## Usage

Create a `.asdfler.yml` wherever, probably your home directory given this is not really per-project (that's what `.tool-versions` are for).

```bash
touch ~/.asdfler.yml
```

Open it and edit! Currently supported config:

```yaml
# top-level key
plugins:
  - name: ruby # Name of the plugin
    default_version: 2.7.2 # Will install this version and run `asdf global ruby 2.7.2`
  - name: golang # Just adds the golang asdf plugin
  - name: crystal
    versions: # Declare multiple versions for a language but no global default
      - 1.2.1
      - 1.3.0
  - name: erlang
    # The list of versions installed is the Union of the default version and any 
    # versions listed.
    default_version: 24.0.3
    versions:
      - 24.1.4

```

While in the home directory or wherever you put the `.asdfler.yml` file, run:

```bash
asdfler install
```

You can also specify a path with the `-p` option. Any valid YAML parseable file of the
provided structure works. Your versions must be strings. JSON, being a superset of YAML,
will work as well:

```json
{
  "plugins": [
    {"name": "ruby", "default_version": "2.7.2"},
    {"name": "golang"}
  ]
}
```

```bash
asdfler install -p my_versions.json
```

## Development

`shards install` is all that should be necessary to get going.

## Contributing

1. Fork it (<https://github.com/your-github-user/asdfler/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [lirossarvet](https://github.com/lirossarvet) - creator and maintainer
