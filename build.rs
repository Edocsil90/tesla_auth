use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    static_vcruntime::metabuild();

    println!("cargo:rerun-if-changed=assets/app.rc");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        embed_windows_icon();
    }
}

/// Compiles `assets/app.rc` with the Windows SDK's `rc.exe` and links the
/// result, so Explorer and the taskbar show the app icon. Uses only std, so it
/// adds no dependency; when `rc.exe` cannot be found the build goes on without
/// an icon instead of failing.
fn embed_windows_icon() {
    let Some(rc) = find_rc() else {
        println!("cargo:warning=rc.exe not found (Windows SDK); building without an icon");
        return;
    };

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo")).join("app.res");
    let status = Command::new(&rc)
        .args(["/nologo", "/fo"])
        .arg(&out)
        .arg("assets/app.rc")
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("cargo:rustc-link-arg-bins={}", out.display());
        }
        other => println!("cargo:warning=rc.exe failed ({other:?}); building without an icon"),
    }
}

/// `RC` wins if set; otherwise the newest `rc.exe` of the installed Windows 10/11 SDKs.
fn find_rc() -> Option<PathBuf> {
    if let Some(rc) = env::var_os("RC") {
        return Some(PathBuf::from(rc));
    }

    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") => "x86",
        Ok("aarch64") => "arm64",
        _ => "x64",
    };

    let roots = [
        env::var_os("ProgramFiles(x86)").map(PathBuf::from),
        env::var_os("ProgramFiles").map(PathBuf::from),
    ];

    let mut found: Vec<PathBuf> = roots
        .into_iter()
        .flatten()
        .map(|root| root.join("Windows Kits").join("10").join("bin"))
        .filter_map(|bin| std::fs::read_dir(bin).ok())
        .flat_map(|entries| entries.flatten().map(|entry| entry.path()))
        .map(|version_dir| version_dir.join(arch).join("rc.exe"))
        .filter(|rc| rc.is_file())
        .collect();

    // Version folders are named like 10.0.22621.0, so the last one sorts newest
    // well enough for picking an SDK.
    found.sort();
    found.pop()
}
