use core::fmt::Arguments;

#[lang="stack_exhausted"]
pub fn stack_exhausted() {
    loop { }
}

#[lang="eh_personality"]
pub fn eh_personality() {
    loop { }
}

#[lang="eh_unwind_resume"]
pub fn eh_unwind_resume() {
    loop { }
}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &Arguments, _file_line: &(&'static str, usize)) -> ! {
    loop { }
}
