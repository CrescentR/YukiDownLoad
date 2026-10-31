//! Locating the external yt-dlp runtime.
//!
//! Yuki is a standalone desktop front-end and does not vendor the `yt-dlp`
//! source tree or prebundle its executable in installers. At runtime, the
//! launcher resolves `yt-dlp` in priority order:
//!
//! 1. Explicit binary override (`YUKI_YTDLP_BIN` / `YTDLP_STUDIO_BIN`)
//! 2. Yuki-managed standalone binary downloaded into the user config directory
//! 3. `yt-dlp` executable available on `PATH`
//! 4. `python -m yt_dlp` from an installed Python environment (`PYTHON`, `python`, `python3`, `py`)

use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

/// Marker printed by the import probe so a failing `python` cannot be mistaken
/// for a missing module.
const PROBE_MARKER: &str = "YUKI_YTDLP_OK";
const CACHE_TTL: Duration = Duration::from_secs(60);

static RESOURCE_DIR: OnceLock<PathBuf> = OnceLock::new();

#[derive(Clone, PartialEq, Eq)]
struct CacheKey {
    managed_dir: Option<PathBuf>,
    yuki_bin: Option<String>,
    studio_bin: Option<String>,
    python_env: Option<String>,
}

struct CachedLaunch {
    key: CacheKey,
    resolved_at: Instant,
    launch: Launch,
}

struct CachedJsRuntime {
    resolved_at: Instant,
    runtime: Option<String>,
}

static LAUNCH_CACHE: Mutex<Option<CachedLaunch>> = Mutex::new(None);
static JS_RUNTIME_CACHE: Mutex<Option<CachedJsRuntime>> = Mutex::new(None);

pub fn set_resource_dir(path: PathBuf) {
    let _ = RESOURCE_DIR.set(path);
}

/// Clear cached launcher and JS runtime lookups after dependency installs or settings changes.
pub fn invalidate_cache() {
    if let Ok(mut guard) = LAUNCH_CACHE.lock() {
        *guard = None;
    }
    if let Ok(mut guard) = JS_RUNTIME_CACHE.lock() {
        *guard = None;
    }
}

/// Construct a `Command` that suppresses transient console windows on Windows.
pub fn hidden_command<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    command
}

fn compat_plugin_dir() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(dir) = RESOURCE_DIR.get() {
        candidates.push(dir.join("yt-dlp-plugins"));
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("yt-dlp-plugins"));
        }
    }
    #[cfg(any(debug_assertions, test))]
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/yt-dlp-plugins"));
    candidates.into_iter().find(|dir| {
        dir.join("douyin-compat/yt_dlp_plugins/extractor/yuki_douyin.py")
            .is_file()
    })
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    /// "sidecar" | "managed" | "path" | "python" | "missing"
    pub mode: String,
    /// Human readable runner, e.g. `yt-dlp` or `python -m yt_dlp`.
    pub runner: String,
    pub repo_root: String,
    pub ytdlp_version: Option<String>,
    /// Node/Deno used by yt-dlp for YouTube signature solving.
    pub js_runtime: Option<String>,
    /// Actionable guidance when yt-dlp could not be resolved at all.
    pub problem: Option<String>,
}

/// How yt-dlp should be launched for this session.
#[derive(Debug, Clone)]
pub struct Launch {
    pub program: String,
    /// Leading args that belong to the launcher itself, e.g. `-m yt_dlp`.
    pub prefix: Vec<String>,
    pub cwd: PathBuf,
    /// Optional path prepended to the child's `PYTHONPATH`.
    pub python_path: Option<PathBuf>,
    pub mode: &'static str,
    pub runner: String,
    pub problem: Option<String>,
    /// Version reported by the launcher, when it could be determined.
    pub version: Option<String>,
}

pub fn repo_root() -> PathBuf {
    #[cfg(any(debug_assertions, test))]
    {
        // In a dev build, CARGO_MANIFEST_DIR is <repo>/src-tauri.
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if let Some(parent) = manifest.parent() {
            return parent.to_path_buf();
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn first_line(program: &str, args: &[&str]) -> Option<String> {
    let output = hidden_command(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

/// Interpreters worth trying, in priority order.
///
/// `PYTHON` wins so a user can pin an interpreter explicitly.
fn python_candidates() -> Vec<String> {
    let mut candidates: Vec<String> = Vec::new();
    if let Ok(explicit) = env::var("PYTHON") {
        let explicit = explicit.trim().to_string();
        if !explicit.is_empty() {
            candidates.push(explicit);
        }
    }
    for name in ["python", "python3", "py"] {
        candidates.push(name.to_string());
    }
    candidates.dedup();
    #[cfg(target_os = "macos")]
    candidates.retain(|candidate| !is_xcode_stub(candidate));
    candidates
}

/// On macOS without the Command Line Tools, `/usr/bin/python3` is only a shim:
/// running it pops up the "install developer tools" dialog and blocks until
/// the user answers, and it is retried on every runtime check. Such a shim
/// can never have yt-dlp installed, so it is skipped without being run.
#[cfg(target_os = "macos")]
fn is_xcode_stub(program: &str) -> bool {
    let path = if program.contains('/') {
        PathBuf::from(program)
    } else {
        let Some(found) = env::var_os("PATH").and_then(|paths| {
            env::split_paths(&paths)
                .map(|dir| dir.join(program))
                .find(|candidate| candidate.is_file())
        }) else {
            return false;
        };
        found
    };
    if !path.starts_with("/usr/bin") {
        return false;
    }
    // `xcode-select` is a real binary; `-p` fails when no developer directory
    // (Xcode or the Command Line Tools) is installed.
    !Command::new("/usr/bin/xcode-select")
        .arg("-p")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Ask an interpreter whether `yt_dlp` is installed in its Python environment.
fn interpreter_has_ytdlp(program: &str) -> bool {
    let script = format!("import yt_dlp; print('{PROBE_MARKER}')");
    hidden_command(program)
        .args(["-I", "-c", &script])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .output()
        .map(|output| {
            output.status.success()
                && String::from_utf8_lossy(&output.stdout).contains(PROBE_MARKER)
        })
        .or_else(|_| Ok::<bool, std::io::Error>(false))
        .unwrap_or(false)
        || hidden_command(program)
            .args(["-c", &script])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .output()
            .map(|output| {
                output.status.success()
                    && String::from_utf8_lossy(&output.stdout).contains(PROBE_MARKER)
            })
            .unwrap_or(false)
}

#[cfg(test)]
pub fn resolve() -> Launch {
    let config = crate::settings::diagnose_config_dir();
    resolve_with(Some(&config))
}

/// Resolve the launcher, preferring an explicit override or Yuki-managed binary.
///
/// `managed_dir` is the app's own data directory. A binary downloaded there
/// takes precedence over `PATH` or Python installs so an in-app update takes
/// effect immediately without touching the user's system environment.
pub fn resolve_with(managed_dir: Option<&Path>) -> Launch {
    let root = repo_root();

    let version_of = |launch: &Launch| -> Option<String> { probe_version(launch) };

    // 1. An explicit binary always wins (packaged sidecar or manual override).
    if let Ok(bin) = env::var("YUKI_YTDLP_BIN").or_else(|_| env::var("YTDLP_STUDIO_BIN")) {
        let trimmed = bin.trim().to_string();
        if !trimmed.is_empty() {
            let mut launch = Launch {
                runner: trimmed.clone(),
                program: trimmed,
                prefix: Vec::new(),
                cwd: root.clone(),
                python_path: None,
                mode: "sidecar",
                problem: None,
                version: None,
            };
            launch.version = version_of(&launch);
            return launch;
        }
    }

    // 2. A standalone binary Yuki downloaded and manages itself.
    if let Some(dir) = managed_dir {
        let managed = crate::update::managed_ytdlp_path(dir);
        if managed.is_file() {
            let mut launch = Launch {
                runner: managed.display().to_string(),
                program: managed.display().to_string(),
                prefix: Vec::new(),
                cwd: root.clone(),
                python_path: None,
                mode: "managed",
                problem: None,
                version: None,
            };
            launch.version = version_of(&launch);
            if launch.version.is_some() {
                return launch;
            }
            // A downloaded binary that will not run must not shadow a working
            // install; fall through to the other sources.
        }
    }

    // 3. A yt-dlp executable on PATH.
    {
        let mut launch = Launch {
            runner: "yt-dlp".into(),
            program: "yt-dlp".into(),
            prefix: Vec::new(),
            cwd: env::current_dir().unwrap_or_else(|_| root.clone()),
            python_path: None,
            mode: "path",
            problem: None,
            version: None,
        };
        launch.version = version_of(&launch);
        if launch.version.is_some() {
            return launch;
        }
    }

    // 4. An installed Python `yt_dlp` package (`pip install yt-dlp`).
    for candidate in python_candidates() {
        if interpreter_has_ytdlp(&candidate) {
            let mut launch = Launch {
                runner: format!("{candidate} -m yt_dlp"),
                program: candidate,
                prefix: vec!["-m".into(), "yt_dlp".into()],
                cwd: env::current_dir().unwrap_or_else(|_| root.clone()),
                python_path: None,
                mode: "python",
                problem: None,
                version: None,
            };
            launch.version = version_of(&launch);
            if launch.version.is_some() {
                return launch;
            }
        }
    }

    Launch {
        runner: "yt-dlp (not found)".into(),
        program: "yt-dlp".into(),
        prefix: Vec::new(),
        cwd: env::current_dir().unwrap_or_else(|_| repo_root()),
        python_path: None,
        mode: "missing",
        problem: Some(
            "yt-dlp could not be located. Download it in Settings → Runtime, place `yt-dlp` on \
             PATH, or install the Python package with `pip install yt-dlp`."
                .into(),
        ),
        version: None,
    }
}

/// Ask the launcher for its version, applying the full launcher prefix so
/// Python module mode (`python -m yt_dlp`) reports yt-dlp rather than Python itself.
fn probe_version(launch: &Launch) -> Option<String> {
    ytdlp_version(launch)
}

/// Resolve the launcher using a short-lived cache to avoid spawning redundant
/// probes on every queued download or URL analysis.
pub fn resolve_cached(managed_dir: Option<&Path>) -> Launch {
    let key = CacheKey {
        managed_dir: managed_dir.map(Path::to_path_buf),
        yuki_bin: env::var("YUKI_YTDLP_BIN").ok(),
        studio_bin: env::var("YTDLP_STUDIO_BIN").ok(),
        python_env: env::var("PYTHON").ok(),
    };

    if let Ok(guard) = LAUNCH_CACHE.lock() {
        if let Some(cached) = guard.as_ref() {
            let still_valid = cached.key == key
                && cached.resolved_at.elapsed() < CACHE_TTL
                && cached.launch.mode != "missing"
                && (cached.launch.mode != "managed" || Path::new(&cached.launch.program).is_file());
            if still_valid {
                return cached.launch.clone();
            }
        }
    }

    let launch = resolve_with(managed_dir);
    if launch.mode != "missing" {
        if let Ok(mut guard) = LAUNCH_CACHE.lock() {
            *guard = Some(CachedLaunch {
                key,
                resolved_at: Instant::now(),
                launch: launch.clone(),
            });
        }
    }
    launch
}

/// Build a `Command` for yt-dlp with the launcher prefix and environment applied.
///
/// Named `build_command` rather than `command` so local `command` variables at
/// call sites cannot shadow it.
pub fn build_command(launch: &Launch) -> Command {
    let mut command = hidden_command(&launch.program);
    command.args(&launch.prefix);
    // The same adapter is used by analysis, downloads and managed executables;
    // updating yt-dlp does not discard the application compatibility layer.
    if let Some(dir) = compat_plugin_dir() {
        command.arg("--plugin-dirs").arg(dir);
    }
    command.args(["--encoding", "utf-8"]);
    if let Some(path) = &launch.python_path {
        let existing = env::var("PYTHONPATH").unwrap_or_default();
        let joined = if existing.trim().is_empty() {
            path.display().to_string()
        } else {
            format!(
                "{}{}{}",
                path.display(),
                if cfg!(windows) { ";" } else { ":" },
                existing
            )
        };
        command.env("PYTHONPATH", joined);
    }

    // Force UTF-8 on the child's stdio.
    //
    // When stdout is a pipe, Python picks its encoding from the locale (`cp936`
    // on Chinese Windows). Pinning UTF-8 avoids `OSError: [Errno 22] Invalid
    // argument` when video titles contain non-codepage characters.
    command.env("PYTHONIOENCODING", "utf-8");
    command.env("PYTHONUTF8", "1");
    command.env("PYTHONUNBUFFERED", "1");

    command
}

pub fn ytdlp_version(launch: &Launch) -> Option<String> {
    let output = build_command(launch)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

/// yt-dlp needs a JavaScript runtime to solve YouTube's signature challenges.
/// Deno is enabled by default upstream; Node works when passed explicitly.
pub fn detect_js_runtime() -> Option<String> {
    for candidate in ["deno", "node", "bun", "qjs"] {
        if first_line(candidate, &["--version"]).is_some() {
            return Some(candidate.to_string());
        }
    }
    None
}

/// Cached JS runtime lookup so repeated task launches do not re-probe PATH.
pub fn detect_js_runtime_cached() -> Option<String> {
    if let Ok(guard) = JS_RUNTIME_CACHE.lock() {
        if let Some(cached) = guard.as_ref() {
            if cached.resolved_at.elapsed() < CACHE_TTL {
                return cached.runtime.clone();
            }
        }
    }
    let runtime = detect_js_runtime();
    if let Ok(mut guard) = JS_RUNTIME_CACHE.lock() {
        *guard = Some(CachedJsRuntime {
            resolved_at: Instant::now(),
            runtime: runtime.clone(),
        });
    }
    runtime
}

pub fn info_with(managed_dir: Option<&Path>) -> RuntimeInfo {
    let launch = resolve_cached(managed_dir);
    RuntimeInfo {
        mode: launch.mode.into(),
        runner: launch.runner.clone(),
        repo_root: launch.cwd.display().to_string(),
        ytdlp_version: launch.version.clone().or_else(|| ytdlp_version(&launch)),
        js_runtime: detect_js_runtime_cached(),
        problem: launch.problem.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_root_and_compat_plugin_are_discovered_from_crate_location() {
        let root = repo_root();
        assert!(
            root.join("package.json").is_file(),
            "expected package.json in workspace root, found {}",
            root.display()
        );
        assert!(
            root.join("src-tauri").is_dir(),
            "expected src-tauri in workspace root, found {}",
            root.display()
        );
        assert!(
            compat_plugin_dir().is_some(),
            "expected bundled Douyin compatibility plugin under src-tauri/resources"
        );
    }

    #[test]
    fn build_command_exports_utf8_environment_and_plugin_dir() {
        let launch = Launch {
            program: "yt-dlp".into(),
            prefix: Vec::new(),
            cwd: repo_root(),
            python_path: None,
            mode: "path",
            runner: "yt-dlp".into(),
            problem: None,
            version: None,
        };
        let command = build_command(&launch);
        let envs: Vec<(String, Option<String>)> = command
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|val| val.to_string_lossy().into_owned()),
                )
            })
            .collect();
        assert!(envs.contains(&("PYTHONIOENCODING".into(), Some("utf-8".into()))));
        assert!(envs.contains(&("PYTHONUTF8".into(), Some("1".into()))));
        assert!(envs.contains(&("PYTHONUNBUFFERED".into(), Some("1".into()))));

        let args: Vec<String> = command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        assert!(args.contains(&"--plugin-dirs".to_string()));
        assert!(args.contains(&"--encoding".to_string()));
        assert!(args.contains(&"utf-8".to_string()));
    }

    #[test]
    fn an_unresolvable_runtime_reports_a_problem_instead_of_failing_silently() {
        let launch = Launch {
            program: "definitely-not-a-real-binary".into(),
            prefix: Vec::new(),
            cwd: PathBuf::from("."),
            python_path: None,
            mode: "missing",
            runner: "test".into(),
            problem: Some("explained".into()),
            version: None,
        };
        assert!(ytdlp_version(&launch).is_none());
    }

    #[test]
    fn the_resolved_launcher_actually_runs_ytdlp_when_installed() {
        let mut launch = resolve();
        if launch.mode == "missing" {
            // Nothing to assert on a machine without yt-dlp installed yet.
            assert!(launch.problem.is_some());
            return;
        }
        assert!(
            ytdlp_version(&launch).is_some(),
            "resolved launcher {launch:?} could not report a version"
        );
        // Resolution must not depend on the caller's working directory.
        launch.cwd = std::env::temp_dir();
        assert!(
            ytdlp_version(&launch).is_some(),
            "yt-dlp could not run from {}",
            launch.cwd.display()
        );
    }
}
