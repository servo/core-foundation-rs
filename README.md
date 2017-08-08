# core-foundation-rs

[![Build Status](https://travis-ci.org/servo/core-foundation-rs.svg?branch=master)](https://travis-ci.org/servo/core-foundation-rs)

## Compatibility

By default it targets macOS 10.7.

To enable features added in macOS 10.8, set Cargo feature `mac_os_10_8_features`. To have both 10.8 features and 10.7 compatibility, also set `mac_os_10_7_support`. Setting both requires weak linkage, which is is a nighty-only feature as of Rust 1.19.
