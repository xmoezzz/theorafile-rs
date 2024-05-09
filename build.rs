use std::path::Path;
use walkdir::WalkDir;

fn glob_import<P: AsRef<Path>>(root: P, extenstion: &str) -> Vec<String> {
    WalkDir::new(root)
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.path().to_str().unwrap().ends_with(extenstion))
        .map(|x| x.path().to_str().unwrap().to_string())
        // .filter(|x| !x.contains(exclude))
        .collect()
}

fn main() {
    let mut cc_build = cc::Build::new();
    let mut cfiles = vec![];
    cfiles.extend(glob_import("upstream/lib/ogg", ".c"));
    cfiles.extend(glob_import("upstream/lib/theora", ".c"));
    cfiles.extend(glob_import("upstream/lib/vorbis", ".c"));
    cfiles.push("upstream/theorafile.c".to_string());
    
    cc_build
        .include("upstream/lib/")
        .include("upstream/")
        .cpp(true)
        .warnings(false)
        .files(cfiles) 
        .cpp(false)
        .pic(true);

    cc_build.compile("theorafile");

    println!("cargo:rustc-link-lib=static=theorafile");
}