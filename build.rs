#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("CompanyName", "Forlenza Industrial");
    res.set("ProductName", "SCADA Control System");
    res.set("FileDescription", "Simulates a realistic industrial control interface, designed to demonstrate common compatibility issues found in legacy SCADA systems.");
    res.set("FileVersion", "2.1.0.0");
    res.set("ProductVersion", "2.1.0.0");
    res.set("LegalCopyright", "© 2010 Forlenza Industrial");
    res.set_icon("src/icon.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
