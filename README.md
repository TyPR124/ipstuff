# ipstuff

Basic IP utlities. This is in active development and I will add things as I need them, generally. If you want a specific feature, feel free to open an issue or PR.

This crate commits to both efficiency and safety. This crate includes `#![forbid(unsafe_code)]` and may use platform-specific implementations if they are proven more efficient.

This crate will abide by Rust's semver standards to the best of my ability for versions published to crates.io. The Github repository, on the other hand, may be updated with breaking changes at any time.

## Features

* Ipv4Mask - A 4-byte type representing a subnet mask. This type will always represent a valid subnet mask value.
* MaskedIpv4 - An 8-byte type represenging a combination of Ipv4Addr and Ipv4Mask. The IP can be any IP within the represented network.
* IpBitwiseExt - An extension trait for Ipv4Addr (and soon Ipv6Addr) for bitwise operations.
* IpBitwiseNotExt - An extension trait for Ipv4Addr (and soon Ipv6Addr) for bitwise not operations.

## Contributions

If you have any suggestions, find any bugs, or have other feedback, feel free to open an issue.

If you wish to contribute a new feature, I recommend opening an issue first as there is a chance I could be working on it myself.

If you find something can be made more efficient, I will accept a PR as long as it can be shown to be an improvement is a reasonable way. This could be generated assembly, benchmarks, a sound logical argument for why the original code is inefficient, or any number of other things. I expect most operations are basic enough that the generated assembly will often be the easiest and most definitive proof of an improvement. If an efficiency improvement requires `unsafe`, it must be shown to be a drastic improvement, and even then I will not promise such a change is accepted. If such a chagne is found to both be a drastic improvement and fall within the scope of this crate, I may lift the `#![forbid(unsafe_code)]` requirement as a breaking change.

To contribute a bug fix, you can open an issue or go straight to a PR.

## License

All code and contributions will be licensed under MIT or Apache-2.0, at the users discresion.
