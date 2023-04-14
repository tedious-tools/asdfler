# asdfler

Manage which [`asdf`](https://asdf-vm.com/#/core-manage-asdf) plugins you have as well as default versions!

NOTE: SUPER UNTESTED AND THROWN TOGETHER IN A COUPLE OF HOURS. **Use at your own risk**

## Installation

### Homebrew

```bash
brew install tedious-tools/formulae/asdfler
```

### Non-Homebrew

1. Install Rust 1.68+ compiler
2. `git clone https://github.com/tedious-tools/asdfler.git`
3. `cd asdfler`
4. `cargo build --release`
5. `mv target/release/asdfler <somewhere in your path like ~/bin>/asdfler`

```yaml
- name: rust
  default_version: 1.68.1
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
  - name: golang
    versions:
      - "1.20" # Note the quotes to ensure YAML sees this as a string
  - name: crystal # Just adds the Crystal asdf plugin
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

You can also specify a path with the `-f` option. Any valid YAML parseable file of the
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
asdfler install -f my_versions.json
```

## Development

`cargo build` is all that should be necessary to get going.

## Contributing

1. Fork it (<https://github.com/tedious-tools/asdfler/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [lirossarvet](https://github.com/lirossarvet) - creator and maintainer
