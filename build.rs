extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/@mnom.nim.c")
        .compile("nom");
    cc::Build::new()
        .file("src/stdlib_io.nim.c")
        .compile("stdlib_io_nim");
    cc::Build::new()
        .file("src/stdlib_system.nim.c")
        .compile("stdlib_system_nim");
}
