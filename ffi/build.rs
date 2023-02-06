fn main() {
    cc::Build::new()
        .file("../lib/world/library.c")
        .compile("world");

    cc::Build::new()
        .cpp(true)
        .file("../lib/hello/library.cpp")
        .compile("library");
}
