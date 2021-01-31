# Optimizations

This file exists to track the status of optimizing functionality within this crate.

Some things to keep in mind when optmizing:

* This crate prefers using safe abstractions over using unsafe code directly.
  * It is permitted to include a dependency for the sake of optimization.
* Optimizations may target specific processor features.
* Features named in code should be listed in this document, and kept in sync.
* To the extent possible, try to keep optimizations reliant on a single feature.
* To the extent possible, implementation code should match the desired assembly.
* Assembly snippets are using x86_64, unless otherwise noted.

## Table of Contents

Items will be checked off only when I feel there is little to no room for further
optimizations.

* [Template](#template)
* Ipv4Mask
  * [ ] [`new`](#ipv4masknew)
  * [X] ~~*[`new_unchecked`](#ipv4masknew_unchecked)*~~ [2021-01-25]
  * [X] ~~*[`new_saturating`](#ipv4masknew_saturating)*~~ [2021-01-25]
  * [ ] `from_u32`
  * [ ] `from_bytes`
  * [ ] [`len`](#ipv4masklen)
  * [X] ~~*[`is_empty`](#ipv4maskis_empty)*~~ [2021-01-25]
  * [X] ~~*[`is_full`](#ipv4maskis_full)*~~ [2021-01-25]
* Ipv6Mask
  * [ ] `new`
  * [ ] `new_unchecked`
  * [ ] `new_saturating`
  * [ ] `from_u32`
  * [ ] `from_bytes`
  * [ ] `len`
  * [ ] `is_empty`
  * [ ] `is_full`

## Template

```md
## `Type::method`

Make sure to add a link in the TOC.

Then add an overview of optimization status here.

### Target Features

Check to ensure the feature is tested. See [Testing](#testing).

* [ ] `example-feature-name`
```

## Testing

Some stuff about testing

## `Ipv4Mask::new`

Not yet optimized

### Target Features

Check to ensure the feature is tested. See [Testing](#testing).

<!-- * [ ] `feature-name` -->

## `Ipv4Mask::new_unchecked`

This is the same as [`Ipv4Mask::new_saturating`](#ipv4masknew_saturating) except
with a debug_assertions check that a valid length was provided.

## `Ipv4Mask::new_saturating`

This is sufficiently optmized. Most optimal assembly is generated with the `bmi2`
feature, however the implementation does not rely on this feature and is well
optimized without this feautre enabled.

### With `bmi2`

```x86asm
cmp     dil, 31
mov     ecx, -1
shrx    eax, ecx, edi
not     eax
bswap   eax
cmova   eax, ecx
ret
```

### Without `bmi2`

```x86asm
mov     ecx, edi
mov     edx, -1
mov     eax, -1
shr     eax, cl
cmp     cl, 31
not     eax
bswap   eax
cmova   eax, edx
ret
```

## `Ipv4Mask::len`

This is well optimized with either `popcnt` or `lzcnt` features, but otherwise
might be improved.

### Target Features

Check to ensure the feature is tested. See [Testing](#testing).

* [ ] `popcnt`
* [ ] `lzcnt`

### With `popcnt`

```x86asm
popcnt  eax, edi
ret
```

### With `lzcnt`

```x86asm
not     edi
bswap   edi
lzcnt   eax, edi
ret
```

## `Ipv4Mask::is_empty`

```x86asm
test    dil, dil
sete    al
ret
```

## `Ipv4Mask::is_full`

```x86asm
cmp     edi, -16777217
seta    al
ret
```
