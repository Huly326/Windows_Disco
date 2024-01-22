# Windows Disco

## Rust Win32 Api Self-Help Group

This project started as a small demo for myself as a first expedition into the Win32 Api and Rust. I am a beginner in both, so please don’t expect perfect code in either domain.

I am posting this because I was very frustrated that there were so few examples for rust and win32 api applications. If you are new to GUI programming with the Win32 Api, I hope this helps you in some way. 

The projects expands on the windows crate window example (https://github.com/microsoft/windows-rs/blob/0.52.0/crates/samples/windows/create_window/src/main.rs) to create a window that changes its color between blue, red and green based on the click of a button. 

It also prints a very small text to the output, but without any of the fancier text manipulation available in the win32 API, as I haven’t figured out how those work myself yet. 

I hope the comments aren’t too excessive; I added a lot of them assuming the reader is not a Win32 Api expert. I hope they will prevent you from the same unproductive searches for help that I had trying to find fixes for weird artifacts in the Background drawing, naming misunderstandings between me and the win32api and Windows Crate type expectations.

Please feel free to use this code for further experiments.

## MIT License
```
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 
```