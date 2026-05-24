# use-key

Key signature metadata primitives for `RustUse`.

## Experimental

use-key is experimental while use-music remains below version 0.3.0.

## Example

~~~rust
use use_key::{KeyMode, KeySignature};

let signature = KeySignature::new(1, KeyMode::Major)?;

assert!(signature.is_sharp_key());
assert_eq!(signature.accidental_count().value(), 1);
# Ok::<(), use_key::KeyError>(())
~~~

## Scope

- Key names, tonics, modes, key signatures, accidental counts, circle-of-fifths positions, and relative/parallel labels.
- Accidental counts use negative values for flats and positive values for sharps.

## Relationship to use-acoustics

`use-music` keeps symbolic music metadata separate from `use-acoustics`, which owns physical sound calculations such as frequency, wavelength, decibels, sound pressure, sound intensity, period, angular frequency, and speed-of-sound approximations.

## Non-goals

- Full tonal analysis, modulation analysis, generated harmony, or audio playback.

## License

Licensed under either Apache-2.0 or MIT.
