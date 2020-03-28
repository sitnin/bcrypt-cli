# Bcrypt cli string hashing tool

## Installation

You should have a rust compiler installed to build this tool. So, you definitely know how to build rust binaries. =)

It is possible to create a ppa or some sort of package. Feel free to PR package building script and building instructions to this repo.

Personally I host `bcrypt` binary in `$HOME/bin`. It is what this tool was created for.

## Usage

```bash
bcrypt [-s] [-c ROUNDS] INPUT_STRING
```

to hash string from CLI args, or

```bash
bcrypt [-s] [-c ROUNDS] -
```

to hash string read from stdin.

### -c | --cost ROUNDS

Sets number of rounds for bcrypt hashing function. Default: 12, minimum: 4.

### -s | --short

This flag switches output. If short mode is enabled, tool's output will be:

```bash
$ bcrypt -s TestStr
$2b$12$lpu4zphyiVtcg3TR3vDxpeu.u/XkA35aE1j9yobJGjUx0EUPwXMjy
```

otherwise this will look like:

```bash
$ bcrypt TestStr
[TestStr] (12) => [$2b$12$lpu4zphyiVtcg3TR3vDxpeu.u/XkA35aE1j9yobJGjUx0EUPwXMjy]
```

## ToDo(-da-bee-da-bee-doo)

- [ ] Wrap code with tests (ha-ha!)
- [ ] Replace panic! with human readable error messages
- [ ] Make .deb build script and package
- [ ] Create ppa for debian/ubuntu/mint/whatever
- [ ] Use github's ci/cd for release deployment to ppa

## Contributing

Feel free to file issues to https://github.com/sitnin/bcrypt-cli/issues
