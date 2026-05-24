# use-mode

Musical mode metadata primitives for `RustUse`.

## Experimental

use-mode is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_mode::{ModeKind, ModeName};

let name = ModeName::new("D dorian")?;
let kind: ModeKind = "dorian".parse()?;

assert_eq!(name.as_str(), "D dorian");
assert_eq!(kind, ModeKind::Dorian);
# Ok::<(), use_mode::ModeError>(())
~~~

## Scope

- Modal names, kinds, church modes, mode degrees, mode families, and brightness labels.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Advanced modal composition, modal interchange analysis, or generated harmony.

## License

Licensed under either Apache-2.0 or MIT.
