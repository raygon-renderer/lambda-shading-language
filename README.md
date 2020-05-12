`SL(λ)`: Lambda Shading Language
================================

Based on the syntax of the Rust programming language, `SL(λ)` (Lambda Shading Language) a JIT-compiled language designed for high-performance shading programs in offline Monte-Carlo renderers and is used in the Raygon renderer.

`SL(λ)` is designed with spectral rendering in mind, and intends to make it easy to integrate into any modern renderer. It will support arbitrary number of wavelengths per-shader and will make use of SIMD instructions to manipulate them performantly.

## Features (Planned and WIP)
* Optimized JIT-Compilation via LLVM (Inkwell)
* Automatic differentiation of arbitrary values in higher dimensions via hyperdual numbers.
* Built-in spectral rendering tools
    * Spectral upsampling from sRGB via Jakob19
* Automatic intervals of arbitrary values in higher dimensions, useful for calculating bounds on procedural effects such as displacement.
* .etc, this list is a work-in-progress