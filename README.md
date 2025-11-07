# Subway Surfers Rust

This is a WebAssembly version of Subway Surfers, but made in Rust. (if this makes sense)

The way it works is simple, C++ handles the frontend, in this case, the WebView

Rust on the other side, handles the HTTP server backend.

You need an HTTP server because if you open an HTML that loads WebAssembly code, it will simply not work.

I still wanna call this "Subway Surfers Rust" because my intention was to handle both frontend, and backend in Rust.

But I was trying to use only Edge WebView2, since it's already installed by default on Windows, and will not increase 100MB on the final build just because of CEF.

And I am too dumb to know how to use the most popular Edge WebView's crates.

And thanks to this C++ lib (which you also need to download):  [Tiny cross-platform webview library for C/C++](https://github.com/webview/webview) it saved me a lot.

# Changelog
## Version 1.0.0
* Includes all the things on the text above, pretty much nothing to say
## Version 1.0.1
* Added an icon to the exe file
## Version 1.0.2
* The icon now appears on the window
* ```build.rs``` detects if you are building on ```debug``` mode or ```release``` mode and switches between ```console``` subsystem and ```windows``` subsystem