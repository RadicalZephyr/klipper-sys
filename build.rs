fn main() {
    cc::Build::new()
        .file("itersolve.c")
        .file("kin_cartesian.c")
        .file("kin_corexy.c")
        .file("kin_delta.c")
        .file("kin_extruder.c")
        .file("kin_polar.c")
        .file("kin_winch.c")
        .file("pyhelper.c")
        .file("serialqueue.c")
        .file("stepcompress.c")
        .compile("klipper")
}
