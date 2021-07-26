# mxadm: a simple CLI to automate Matrix room admin tasks

`mxadm` is a simple command-line tool
to do some basic Matrix room admin tasks,
especially things which don't currently have a UI in [Element][],
or which are possible but require a fair bit of effort
(e.g. using Element's devtools).

[Element]: https://matrix.org/docs/projects/client/element

It should be considered unstable:
the command-line arguments will evolve
as the tool itself evolves,
but the functionality should roughly the same.

## Getting started

### Installation

Not yet packaged for any OS as far as I'm aware.
You can install it with Rust's `cargo` tool:

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Install `mxadm`:

   ``` shellsession
   $ cargo install mxadm
   ```

### Usage

The only documentation is currently via the tool itself:

``` shellsession
$ mxadm help
mxadm 0.1.0
Jez Cope <j.cope@erambler.co.uk>
A simple CLI interface to basic Matrix room admin tasks

USAGE:
    mxadm [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    alias     alias subcommands
    help      Prints this message or the help of the given subcommand(s)
    login     authenticates and saves the session details
    logout    ends the current session
    room      room subcommands
    status    displays current session status
```

## Development & contributions

[_See also license info_](#license)

This is a hobby project of mine,
developed for my own use case,
but I would be delighted if someone else wanted to use it!
If you find bugs or want to request new features
please feel free to [create a new issue](https://tildegit.org/petrichor/mxadm/issues/new)
(it's running gitea and you can sign in with GitHub or Twitter)
but bear in mind that it will probably take me some time
to get around to responding or doing something about it!
If you want something changing and have the skills,
a pull request/patch will probably be quicker,
but there will still be delays before I can review and merge it.

I'm also pretty new to Rust,
so my code may not be the best;
constructive code review very welcome!

### Contact me

- Matrix: [@jez:petrichor.me](https://matrix.to/#/@jez:petrichor.me)
- Fedi: [@petrichor@scholar.social](https://scholar.social/@petrichor)
- Twitter: [@jezcope](https://twitter.com/jezcope)

## TODO

- [x] Login and store session info
- [ ] Log out
- [x] Show login status
- [x] Tombstone room
- [x] Add room alias
- [x] Remove room alias
- [ ] Set canonical room alias
    - [ ] Optionally create alias in one command
- [ ] Remove canonical room alias
- [x] List rooms
- [ ] Filter room list in various ways (esp. Spaces!)
- [ ] Handle multiple accounts on different homeservers
- [ ] Upgrade room
- [ ] Make it easier to script workflows like "configure ACL permissions -> invite mjolnir -> op mjolnir" in multiple rooms

## License
 
Copyright (C) 2021 Jez Cope

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.   
