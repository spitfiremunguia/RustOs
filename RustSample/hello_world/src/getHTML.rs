fn getHTML() {
    // Spawn a process. Do not wait for it to return.
    // Process should be mutable if we want to signal it later.
    let mut the_process = Command::new("curl")
        .arg("http://www.hoverbear.org")
        .spawn().ok().expect("Failed to execute.");
    // Do things with `the_process`
}
 