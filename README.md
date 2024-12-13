[![Build Status (nightly)](https://github.com/sigurd4/slice_trait/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/slice_trait/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/slice_trait/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/slice_trait/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/slice_trait/workflows/Test/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/slice_trait/workflows/Lint/badge.svg)](https://github.com/sigurd4/slice_trait/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/slice_trait.svg)](https://crates.io/crates/slice_trait)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/slice_trait)](https://docs.rs/slice_trait)
[![Coverage Status](https://coveralls.io/repos/github/sigurd4/slice_trait/badge.svg?branch=master)](https://coveralls.io/github/sigurd4/slice_trait?branch=master)

# slice_trait

A trait for any slice, with item as an associated type.

## Example

```rust
use slice_trait::*;

let a: &[i32] = [1, 2, 3].as_slice();

fn first<'a, S: Slice + ?Sized>(slice: &'a S) -> Option<&'a S::Item>
where
    S::Item: Copy,
{
    slice.as_slice().first()
}

assert_eq!(first(a), Some(&1));
```