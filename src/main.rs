use windows::{
    core::*, //Core is the windows crate’s way to handle types in the windows apis that Rust treats differently than C/C++ (where many are just integers)
    Win32::Foundation::*, //Foundation too
    Win32::UI::WindowsAndMessaging::*, 
    Win32::{Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleA},
};
mod paint;
/* This enum is what controls the background color that should be drawn the next time the app redraws the window.

Using a global mutable enum like this is not idiomatic to Rust, however, since the Window Procedure has a fixed signature,
we would need to otherwise use an advanced concept like using the app data struct (see: https://learn.microsoft.com/en-us/windows/win32/learnwin32/managing-application-state-),
which I have not explored myself yet and is also using side effects as far as I can tell, which is another Rust anti-pattern. I guess we’ll have to die some deaths here.
*/
static mut window_state: paint::WindowState = paint::WindowState::BLUE;

//I left the main function mostly unchanged from the windows crate sample.
//Most of this is boilerplate code that sets up a standard window with sensible defaults
fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let x = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Windows Disco"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

/*
The Window Procedure Function, expanded from the example in the windows crate.
This function handles the messageloop
 */
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        //The main message loop.
        // Unfortunately, every function in the win32api is unsafe.
        //This should mean that for a better app, we do as much code outside of the api calls
        //in safe Rust methods that we call within out WM_PAINT handle function in order to avoid 
        //not using rusts memory safety guarantees.


        match message {
            WM_PAINT => {
                //WM_PAINT is the most important message for our App. This message is the message that gets called to redraw the 
                //Client Area (the area inside the window). 
                //In our example, we have outsourced this functionality to paint.rs, since this function is quite large

                println!("WM_PAINT");

                paint::handling_paintmessage(window);

                //Binding for: return SYSTEM_SUCCESS;
                LRESULT(0)
            }
            WM_SIZE => {
                println!("WM_SIZE");
                //In an actual application, we would have to write logic here that resizes our client area if the window is resized.
                //currently, it is possible to resize the window such that our button is hidden.
                //That should probably be fixed
                LRESULT(0)
            }

            WM_COMMAND => {
                println!("WM_COMMAND");
                /*
                WM_COMMAND gets sent when the Button is pressed.
                When our Window Procedure gets called with a WM_COMMAND message, it sends two context parameters as well. The first is called wparam, the
                second lparam. wparam = 0 should indicate the message came from a menu or a control item on screen, like our button.
                lparam appears to be the handle to our button.

                We are currently expecting only our button to send WM_Command messages, and if we get anything else, our app panics, so we can find out what
                else was sent.
                
                https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command
                 */
                match wparam {
                    WPARAM(0) => {
                        //This should be identical to the handle to our button (see WM_CREATE)
                        println!("LPARAM: {}", lparam.0 as i64);

                        //Switch BLUE => RED => GREEN (=> BLUE)
                        match &window_state {
                            paint::WindowState::BLUE => {
                                window_state = paint::WindowState::RED;
                            }

                            paint::WindowState::RED => {
                                window_state = paint::WindowState::GREEN;
                            }
                            paint::WindowState::GREEN => {
                                window_state = paint::WindowState::BLUE;
                            }
                        }

                        /*
                        It took me a long while to debug an issue with this: you always need to initialise the RECT to the correct place here (which for us is the entire window), otherwise
                        nothing happens… (I just used RECT::default(), which leaves you with the funny bug of only updating the color change when resizing or maximising the window.)
                         */
                        let mut f: RECT = RECT::default();
                        
                        let j = GetWindowRect(window, &mut f);
                        match j {
                            Ok(..) => {
                                RedrawWindow(
                                    window,
                                    Some(&mut f),
                                    HRGN::default(),
                                    RDW_ERASE | RDW_INVALIDATE | RDW_ALLCHILDREN,
                                );
                            }

                            Err(..) => {
                                panic!("Found no RECT for our window!");
                            }
                        }
                    }

                    WPARAM(_) => {
                        //For now this App has only one button, if there are WM_Command messages from somewhere else, this should panic the app, so we can find out
                        //where they are coming from
                        //If you are asking why one is signed and the pointer type isn’t, well apparently in MSVC/Windows C/C++ at least,
                        //pointers are signed. https://learn.microsoft.com/en-us/windows/win32/WinProg64/rules-for-using-pointers
                        let panic_context: u64 = wparam.0 as u64;
                        let panic_context2: i64 = lparam.0 as i64; 
                        panic!(
                            "An unexpected WM_COMMAND-Message! WPARAM: {} \n LPARAM: {}",
                            panic_context, panic_context2
                        );
                    }
                }

                LRESULT(0)
            }

            WM_NOTIFY => {
                println!("WM_NOTIFY");
                LRESULT(0)
            }

            WM_CREATE => {
                //Message gets called when our window gets first created.
                println!("WM_CREATE");
                let mut f = RECT::default();

                //In cpp, this looks like:
                //HINSTANCE h_in = (HINSTANCE)GetWindowLongPtr(m_hwnd, GWLP_HINSTANCE)
                //In general, trying how to convert types by looking at Rust analyser is usually going to be quicker and more helpful than looking
                //at the windows crate docs, most of which are auto-generated.
                //You might find what you need in the (closed) github issues on their page, though

                let h_in: HINSTANCE = HINSTANCE(GetWindowLongPtrW(window, GWLP_HINSTANCE));
                /*
                I copied the default button from MS-Documentation and it took me an embarrassingly long time to get this button to work correctly. Please declare this button here, not in the main() function and not in
                your WM_PAINT-handler. I did the WM_PAINT handler at first and that has the annoying effect of not showing it initially, only when resizing for some
                reason and it disappeared when you maximised the window.

                Why on earth MS would not document this anywhere is beyond me, but I guess they want to give ancient forum posts on codeguru.com a Google Search boost? Content last
                updated in 1999 rarely makes it to the top of Google Search these days, so I guess codeguru appreciated that.
                 */
                let my_button: HWND = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("BUTTON"), // w! macro converts string literals to a u16-Windows unicode string
                    w!("OK"),     // Button text
                    WS_TABSTOP //The windows styles confused me, too. 
                        | WS_VISIBLE
                        | WS_CHILD
                        | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32) //This is another peculiarity of the windows crate. In C, no explicit type conversion is necessary
                        | WINDOW_STYLE(BS_PUSHBUTTON as u32),   //That is very unfortunate, as it does mean many Cpp examples are not trivially ported to Rust
                    300,          // x position
                    300,          // y position
                    100,          // Button width,
                    100,          // Button height
                    window,       // Parent window
                    None,         // No menu.
                    h_in,
                    None,
                );
                //For handling the button correctly, we should actually use its HWND, I think. Since we aren’t doing anything fancy yet, this works for now.
                println!("Button-HWND: {:?}", my_button);
                RedrawWindow(
                    window,
                    Some(&mut f),
                    HRGN::default(),
                    RDW_UPDATENOW | RDW_ALLCHILDREN,
                );
                LRESULT(0)
            }

            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => {
                //We are currently only handling a small minority of window messages explicitly. 
                //I found this print spam to be somewhat helpful for debugging.
                //There are tables online for which message has what name, but there are hundreds, so you might just be confused.
                //Most messages can be safely ignored because DefWindowProc handles them for us, below.
                //
                println!("Window-Message: {}", message);

                //Any Window Message has a default handling with this function.
                //If we expand our app, we would need to handle more messages ourselves.

                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}
