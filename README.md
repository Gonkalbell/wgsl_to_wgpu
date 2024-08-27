# JMB's fork of wgsl_to_wgpu

This is a fork of [wgsl_to_wgpu](https://crates.io/crates/wgsl_to_wgpu) for my own personal use.

The main change I made is removed generated bind group modules and pipeline layouts. WGSL allows users to mix and match which global variables get used in which bind groups, and it is technically impossible to determine what bind group layouts need to be generated just by statically analyzing shader code. `wgsl_to_wgpu` currently will either panic, return errors, or generate invalid code for many of these use cases.

I instead generate a module for each global variable that I can use to compose individual bind groups. This data contains only what can be determined by statically analyzing the shader code.
