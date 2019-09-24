# tnfilt

## Overview

Turn your typenum compilation errors into something fit for a
human. `tnfilt` turns this:

```
error[E0271]: type mismatch resolving `<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B0>, typenum::B1>, typenum::B0>, typenum::B0> as typenum::IsLessOrEqual<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B0>, typenum::B1>, typenum::B0>>>::Output == typenum::B1`
```

into this:
```
error[E0271]: type mismatch resolving `<U20 as typenum::IsLessOrEqual<U10>>::Output == typenum::B1`
```

## Getting Started
```shell
$ cargo install --git ssh://git@github.com/auxoncorp/tnfilt.git --force
```

## Usage
```shell
$ cargo build 2>&1 | tnfilt
```

## License

`tnfilt` is licensed under the MIT License (MIT) unless otherwise
noted. Please see [LICENSE](./LICENSE) for more details.
