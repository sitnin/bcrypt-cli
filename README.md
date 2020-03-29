# Bcrypt cli string hashing tool

## Installation

You should have a rust compiler installed to build this tool. So, you definitely know how to build rust binaries. =)

It is possible to create a ppa or some sort of package. Feel free to PR package building script and building instructions to this repo.

Personally I host `bcrypt` binary in `$HOME/bin`. It is what this tool was created for.

## Usage

```bash
$ bcrypt [options] [INPUT]
```

## INPUT

Sets string for hashing.

If not set, program will read string from STDIN (without a prompt).

## Option `--cost=ROUNDS` | `-r ROUNDS`

Sets number of rounds for bcrypt hashing function. Default: 12, minimum: 4.

## Option `--verify=HASH` | `-t HASH`

Switches app to hash verification mode and sets the hash to test `INPUT` against of.

In this case, program's output will be just "YES", "NO" or "ERROR" (with error message).

## Flag `--silent` | `-s`

Switch output to semi-silent mode. If set, tool's output will look like:

```bash
$ bcrypt -s TestStr
$2b$12$lpu4zphyiVtcg3TR3vDxpeu.u/XkA35aE1j9yobJGjUx0EUPwXMjy
```

otherwise it more human-readable:

```bash
$ bcrypt TestStr
[TestStr] (12) => [$2b$12$lpu4zphyiVtcg3TR3vDxpeu.u/XkA35aE1j9yobJGjUx0EUPwXMjy]
```

## ToDo(-da-bee-da-bee-doo)

- [ ] Wrap code with tests (ha-ha!)
- [x] Add hash verification mode
- [x] Replace panic! with human readable error messages
- [ ] Version info printing
- [ ] Make .deb build script and package
- [ ] Create ppa for debian/ubuntu/mint/whatever
- [ ] Use github's ci/cd for release deployment to ppa

## Contributing

Feel free to file issues to https://github.com/sitnin/bcrypt-cli/issues
