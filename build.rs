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
    #[cfg(not(target_os = "windows"))]
    cfiles.extend(glob_import("upstream/lib/theora", ".c"));
    //#[cfg(target_os = "windows")]
    {
        let ccs = glob_import("upstream/lib/theora", ".c");
        // delete all 'x86\\'
        let ccs = ccs.into_iter().filter(|x| !x.contains("x86\\"));
        cfiles.extend(ccs);
        let ccs = glob_import("upstream/lib/theora/x86_vc", ".c");
        cfiles.extend(ccs);
    }
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