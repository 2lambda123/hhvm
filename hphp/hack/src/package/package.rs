// Copyright (c) Meta, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use anyhow::Context;
use anyhow::Result;
use hash::IndexMap;
use serde::Deserialize;
use toml::Spanned;

// Preserve the order for ease of testing
// Alternatively, we could use HashMap for performance
type PackageMap = IndexMap<String, Package>;

#[derive(Debug, Deserialize)]
struct Config {
    packages: PackageMap,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub uses: Option<Vec<Spanned<String>>>,
    pub includes: Option<Vec<Spanned<String>>>,
}

#[derive(Debug)]
pub struct PackageInfo {
    packages: PackageMap,
    line_offsets: Vec<usize>,
}

impl PackageInfo {
    pub fn from_text(contents: &str) -> Result<PackageInfo> {
        let config: Config = toml::from_str(contents)
            .with_context(|| format!("Failed to parse config file with contents: {}", contents))?;
        let line_offsets = contents
            .char_indices()
            .filter(|&(_i, c)| c == '\n')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        Ok(Self {
            packages: config.packages,
            line_offsets,
        })
    }

    pub fn packages(&self) -> &PackageMap {
        &self.packages
    }

    pub fn line_number(&self, byte_offset: usize) -> usize {
        match self.line_offsets.binary_search(&byte_offset) {
            Ok(n) | Err(n) => n + 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_basic_file() {
        let contents = include_str!("tests/package-1.toml");
        let info = PackageInfo::from_text(contents).unwrap();

        let foo = &info.packages()["foo"];
        assert_eq!(foo.uses.as_ref().unwrap()[0].get_ref(), "a.*");
        assert!(foo.includes.is_none());
        assert_eq!(info.line_number(foo.uses.as_ref().unwrap()[0].span().0), 4);

        let bar = &info.packages()["bar"];
        assert_eq!(bar.uses.as_ref().unwrap()[0].get_ref(), "b.*");
        assert_eq!(bar.includes.as_ref().unwrap()[0].get_ref(), "foo");

        let baz = &info.packages()["baz"];
        assert_eq!(baz.uses.as_ref().unwrap()[0].get_ref(), "x.*");
        assert_eq!(baz.uses.as_ref().unwrap()[1].get_ref(), "y.*");
        assert_eq!(baz.includes.as_ref().unwrap()[0].get_ref(), "foo");
        assert_eq!(baz.includes.as_ref().unwrap()[1].get_ref(), "bar");
        assert_eq!(info.line_number(baz.uses.as_ref().unwrap()[0].span().0), 11);
        assert_eq!(info.line_number(baz.uses.as_ref().unwrap()[1].span().0), 11);
    }

    #[test]
    fn test_multiline_uses() {
        let contents = include_str!("tests/package-2.toml");
        let info = PackageInfo::from_text(contents).unwrap();

        let foo = &info.packages()["foo"];
        let foo_uses = &foo.uses.as_ref().unwrap();
        assert_eq!(foo_uses[0].get_ref(), "a.*");
        assert_eq!(foo_uses[1].get_ref(), "b.*");
        assert_eq!(info.line_number(foo_uses[0].span().0), 7);
        assert_eq!(info.line_number(foo_uses[1].span().0), 9);

        let foo_includes = &foo.includes.as_ref().unwrap();
        assert_eq!(foo_includes[0].get_ref(), "bar");
        assert_eq!(info.line_number(foo_includes[0].span().0), 12);
    }
}
