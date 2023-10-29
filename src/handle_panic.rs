use std::{fs::OpenOptions, io, io::Write, iter, panic, string::String};

use backtrace::Backtrace;

pub fn handle_panic(info: &panic::PanicInfo) {
    let trace = Backtrace::new();

    let mut msg = String::new();

    let payload = info.payload();
    let payload_string = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()));

    msg.push('\n');
    if let Some(panic_message) = payload_string {
        msg.push_str(&format!("\n--\n{}\n--\n\n", panic_message));
    }
    if let Some(location) = info.location() {
        msg.push_str(&format!(
            "Location {}:{}:{}\n\n",
            location.file(),
            location.line(),
            location.column()
        ));
    }
    msg.push_str(&format!("{:?}\n", trace));
    for ch in iter::repeat('=').take(99) {
        msg.push(ch);
    }

    eprintln!("\nPanic happened{}", &msg);
    write_to_file(&msg).expect("Could not write panic to file.");
}

fn write_to_file(msg: &str) -> io::Result<()> {
    let data_dir = crate::configuration::data_dir();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(data_dir.join("panic.txt"))?;
    write!(file, "{msg}")?;
    Ok(())
}
