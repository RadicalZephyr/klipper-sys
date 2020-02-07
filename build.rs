fn main() {
    cc::Build::new()
        .file("itersolve.c")
        .file("kin_cartesian.c")
        .file("pyhelper.c")
        .file("serialqueue.c")
        .file("stepcompress.c")
        .compile("klipper")
}
