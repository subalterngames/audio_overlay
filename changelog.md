# 0.1.5

- Cleaned up the code format and clippy warnings.

# 0.1.4

- Made all integer min/max values constants instead of casting integer types on the fly.

# 0.1.3

- Fixed: Wrong URL for the example in the README.

# 0.1.2

- Fixed: If `time` is greater than the duration of `dst`, `src` will still be appended immediately after `dst`. Now, `dst` will be filled with zeros up to `time`.
- Significant speed improvement to adding data to the end of `dst`.
- Renamed parameter `push` to `add`.
- Turned the example `mix.rs` script into its own compilable src/ directory.