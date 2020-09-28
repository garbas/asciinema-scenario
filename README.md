# asciinama-scenario

![Build](https://github.com/garbas/asciinema-scenario/workflows/Build/badge.svg)

Create asciinema videos from a text file.

Have you ever re-record your asciinema video over and over again to hit perfect
speed and avoid making typos? I did, too many times and this is why I wrote
this tool.


## Usage

![asciinema-scenario](https://raw.githubusercontent.com/garbas/asciinema-scenario/master/example/demo.gif)


## How to write a .scenario file?

* If first line starts with `#!` it must be followed by JSON object. The object
  can include:

    | Name | Type | Default | Description |
    | --- | --- | --- | --- |
    | step | float | 0.10 | A time in seconds of typing speed of a single event. |
    | width | int | 77 | Maximum number of characters in one line. |
    | height | int | 20 | Number of lines of the video |

* Empty lines will add timeout of `3 x step`.

* Lines starting with `#` will be skipped and can serve as comments.

* Lines starting with `$ ` will be typed out one character at the time with 
  `step` timeout in between. Every character after `#` will be brighter.

* Lines starting with "(nix-shell) $ " will be typed out with `(nix-shell) `
  in green color.

* Lines starting with "--" will clear the screen. A timeout of `18 * step` will
  be there before the terminal screen clears.

* Everything else will be displayed immediately.


## Releases

Detailed release notes are available in this repo at [CHANGES.md](CHANGES.md).


## Reporting issues

Found a bug? I'd love to know about it!

Please report all issues on the GitHub [issue tracker](https://github.com/garbas/asciinema-scenario/issues).


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
