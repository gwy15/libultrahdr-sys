# libultrahdr-sys

A rust bindgen crate for Google's libultrahdr.

## Building with installed libraries
Since libultrahdr depends on libjpeg,
- On *nix systems, `pkg-config` will be used to find libjpeg-turbo.
- On windows systems, `vcpkg` will be used to find libjpeg-turbo.

You need to manually compile and install libultrahdr per its [instruction]. After that, either
- use `pkg-config` use automatically find libultrahdr, or
- set environment variables to skip `pkg-config` search.
```
UHDR_LIB_PATH=/path/to/lib
UHDR_HEADER=/path/to/uhdr_api.h
# for static linking
UHDR_STATIC=1
```

[instruction]: https://github.com/google/libultrahdr/blob/main/docs/building.md

## Building from source
Not implemented yet. PRs welcome!

