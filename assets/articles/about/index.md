# About This Blog

This blog is a demonstration of a pure WASM application built with Rust and the Dioxus framework.

- Minimize the use of JavaScript.
- Markdown rendering for articles.
- Custom-styled navigation and theming.

## Navigation Tests

Test SPA navigation with the following links:

**Internal Article Links (should use SPA navigation):**
- [My First Post](/articles/first-post)
- [Exploring Rust and WASM](/articles/rust-and-wasm)

**Internal Route Links (should use SPA navigation):**
- [Home Page](/)
- [Articles List](/articles)

**External Links (should open normally):**
- [GitHub](https://github.com)
- [Rust Lang](https://www.rust-lang.org)
- [Exampe][exmape.com]

The links above marked as "should use SPA navigation" should navigate without page reload and should have the `data-spa-link` attribute when rendered.