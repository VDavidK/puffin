# Puffin - The friendly TUI programming language!

!> Puffin is not feature complete or ready for production. Use it only if you want to experiment and have fun.

Puffin is a simple programming language that offers a convenient and user friendly way to program your own TUI applications.

## Installing Puffin

In order to install the application and make it available under the `puffin` command line alias. Run the following command:

```sh
./install.sh
```

Below are the same instructions as contained within the `install.sh` script in case you want to manually write the commands.

```sh
cargo install --path puffin_cli --root ./tmp
mv ./tmp/bin/puffin_cli ~/.cargo/bin/puffin
rm -r ./tmp
```

## Using puffin

In order to run puffin files, use the `puffin run` subcommand with a file passed in as the third argument (e.g. `puffin run Main.puff`).
In case you do not want to install Puffin, it is possible to use the `cargo run --` command in the `puffin_cli` directory as a substitute. The above example would then become `cargo run -- run Main.puff`

## Building the docs

The docs are generated using `docsify`. In order to build the docs you must have `docsify` installed. With it installed, run `docsify serve ./docs` command to serve the docs locally.
