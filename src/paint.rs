use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*,
};
/*
These are the default colors from the COLORREF Reference

https://learn.microsoft.com/en-us/windows/win32/gdi/colorref
 */
const RED: COLORREF = COLORREF(0x000000FF);
const BLUE: COLORREF = COLORREF(0x00FF0000);
const GREEN: COLORREF = COLORREF(0x0000FF00);
//For now we have just three colors
//A more complicated application would use a struct to handle app state
//see main.rs
#[derive(Clone)]
pub enum WindowState {
    RED,
    GREEN,
    BLUE,
}

use crate::window_state;

pub unsafe fn handling_paintmessage(window: HWND) {
    unsafe {
        //Contains Data for the drawing
        //https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct
        let mut ps = PAINTSTRUCT::default();

        //A handle to the client area (the inside of your window)
        //Even Microsoft call the exact structure of this type “opaque”, so I think we can
        //safely ignore the details and just use it.
        //https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc
        let my_hdc = GetDC(window);

        //A RECT (Rectangle) Structure is a structure that contains an area in which we can draw.
        //This structure can contain a subset of the entire window, but we want to fill
        //the entire window, so we call GetWindowRect to find the lower right corner coordinates
        let mut rect = RECT::default();
        let _do_we_get_a_rect = GetWindowRect(window, &mut rect).unwrap();
        /*
        Okay, quick hint at a potential weird thing with the windows crate. None of the cpp examples I found needed to explicitly zero out the upper left corner
        of our Rectangle. Actually, the GetWindowRect documentation seems to suggest that they only update the lower right corner of the RECT, but doesn’t say so
        explicitly.

        Your window will work if you don’t do this explicitly, but it will have weird artifacts if you resize or maximise it, and draw black voids in the bottom left
        and upper right of your client area. 
         */
        rect.left = 0;
        rect.top = 0;

        //This function is necessary and needs to form a "bracket" with EndPaint around any painting functions. 
        BeginPaint(window, &mut ps);

        //
        //The way we have set up our app, the global enum will change upon one click of a button and so we can just use it here to see what needs repainting
        //
        match window_state {
            WindowState::RED => {
                //The Brush Function is what is necessary for filling the entire screen with a color with the FillRect function.

                let my_brush = CreateSolidBrush(RED);

                //SetBkColor doesnt actually fill our screen with a color, this function is used to draw the background for our text output below, which is otherwise white by default.
                //Yes, the documentation could state that explicitly, but where would the fun in finding out yourself in an hour long brainstorming session be?
                //https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor
                SetBkColor(my_hdc, RED);

                //This is what is  a c t u a l l y  filling the background.
                FillRect(my_hdc, &rect, my_brush);
            }

            WindowState::BLUE => {
                let my_brush = CreateSolidBrush(BLUE);
                //Unlike with the other colors, we want our Background to be white, since black text on blue background is very hard to read.
                //White Background is also the default behaviour, so this is technically not necessary.
                SetBkColor(my_hdc, COLORREF(0x00FFFFFF));

                FillRect(my_hdc, &rect, my_brush);
            }

            WindowState::GREEN => {
                let my_brush = CreateSolidBrush(GREEN);

                SetBkColor(my_hdc, GREEN);
                FillRect(my_hdc, &rect, my_brush);
            }
        }

        //The Windows Crate maintainers want you to use HSTRING as in intermediary between Rust utf-8-strings and the Windows APIs.
        //I’m not positive mine is the  b e s t  implementation, but it works…
        //String interop between Rust and Windows is definetly one of the pain points, and it is not well documented.
        //The macros you find in the windows crate s!, w!, h!, only work on string literals known at compile time, but they are more convenient.
        //
        //The best tip I can give you is to just look at the windows crate github issues (remember the closed ones!), Rust Analyzer and Windows Documentation
        //Even experienced devs find this whole mess difficult to work with:
        //https://github.com/microsoft/windows-rs/issues/2762
        //I’m not sure why the maintainers of the windows crate don’t provide more convenience here, I’m sure they have valid reasons
        //but we’ll have to work with what we got
        let text: &str = "Please click the Button to change the window’s color";
        let hstring: HSTRING = HSTRING::from(text);

        //The mut is necessary only for DrawTextW
        let mut wide_char = hstring.as_wide().to_owned();

        //The alternative method for drawing text on the Screen, TextOutW (W means Unicode String), will ignore line breaks like\n.
        //DrawText however doesn’t allow you to specify a location, it wants a RECT, which is a bit more complicated.
        //DrawText also operates with a more powerful, but also presumably more complicated IDWriteFormat
        //for choosing fonts, colors, etc.
        //https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nn-dwrite-idwritetextformat
        //I haven’t checked out the details of that yet

        TextOutW(my_hdc, 0, 0, &wide_char);
        //DrawTextW(my_hdc, &mut wide_char, &mut rect, DT_LEFT);

        EndPaint(window, &ps);
    }
}
