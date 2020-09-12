# asciinama-scenario

![master](https://github.com/garbas/asciinama-scenario/workflows/master/badge.svg)

Create asciinema videos from a text file.

Have you ever re-record your asciinema video over and over again to hit perfect
speed and avoid making typos? I did, too many times and this is why I wrote
this tool.


## Usage

![asciinema-scenario](https://raw.githubusercontent.com/garbas/asciinema-scenario/master/example/demo.gif)


## How to write a .scenario file?

* First list is header, which starts with `#!` and is followed by JSON
  object. The object must include ...

* Empty lines will be skipped.

* Lines starting with `#` will be skipped and can serve as comments.

* Lines starting with `$ ` will be typed out one character at the time. Every
  character after `#` will be brighter.

* Lines starting with "(nix-shell) $ " will be typed out with `(nix-shell) `
  in green color.

* Lines starting with "--" will clear the screen.

* Everything else will be displayed immediately.


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
