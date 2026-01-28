#[macro_export]
macro_rules! dbg_file {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/shortcut-notes-tui.log")
            .unwrap();
        writeln!(f, $($arg)*).unwrap();
    }};
}
