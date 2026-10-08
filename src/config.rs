use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;

pub fn database_url() -> String {
    env::var("BORROW_DB_URL").unwrap_or_else(|_| "sqlite:borrow.db?mode=rwc".to_string())
}

/// The desktop app's database file.
///
/// The web keeps [`database_url`]'s relative path: it runs from the repository,
/// where a `borrow.db` beside the process is a feature. The installed app has
/// no such directory — its cwd is wherever the launcher put it (`/` on macOS
/// when opened from Finder, the shortcut's "Start in" on Windows) — so
/// progress lives in a directory the user owns instead.
///
/// `BORROW_DB_URL` still wins when it is set; see `db::init_app`.
#[allow(dead_code)] // called by the desktop app only; the web has its own URL
pub fn app_database_path() -> PathBuf {
    app_data_dir().join("borrow.db")
}

/// The directory [`app_database_path`] lives in, per platform convention:
/// never beside the executable, which is not ours to write to.
///
/// Falls back to the current directory when no home can be found, which is
/// the pre-0.1 behaviour — better to write somewhere than to refuse to start.
#[allow(dead_code)] // called by the desktop app only
pub fn app_data_dir() -> PathBuf {
    let platform = if cfg!(target_os = "macos") {
        Platform::Mac
    } else if cfg!(windows) {
        Platform::Windows
    } else {
        Platform::Other
    };

    let home = env::var_os("HOME").map(PathBuf::from);
    let xdg = env::var_os("XDG_DATA_HOME").map(PathBuf::from);
    let appdata = env::var_os("APPDATA").map(PathBuf::from);

    data_dir_for(platform, home, xdg, appdata)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

/// Which platform's rules apply, so all three can be tested on any one of
/// them — the environment is read here and handed in, never read twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    Mac,
    Windows,
    Other,
}

/// macOS: `~/Library/Application Support/BORROW`
/// Windows: `%APPDATA%\BORROW`, with `HOME` as the belt-and-braces case
/// Linux/other: `$XDG_DATA_HOME/borrow`, with `~/.local/share/borrow`
fn data_dir_for(
    platform: Platform,
    home: Option<PathBuf>,
    xdg_data: Option<PathBuf>,
    appdata: Option<PathBuf>,
) -> Option<PathBuf> {
    match platform {
        Platform::Mac => home.map(|home| {
            home.join("Library")
                .join("Application Support")
                .join("BORROW")
        }),
        Platform::Windows => appdata
            .or_else(|| home.map(|home| home.join("AppData").join("Roaming")))
            .map(|dir| dir.join("BORROW")),
        Platform::Other => xdg_data
            .map(|dir| dir.join("borrow"))
            .or_else(|| home.map(|home| home.join(".local").join("share").join("borrow"))),
    }
}

pub fn address() -> SocketAddr {
    let host = env::var("BORROW_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("BORROW_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    format!("{host}:{port}")
        .parse()
        .expect("BORROW_HOST:BORROW_PORT must be a valid socket address")
}

/// Whether the code playground may be used.
///
/// The playground compiles and runs code the user typed, with the same
/// privileges as the server process. That is fine on a loopback address where
/// only the person sitting at the machine can reach it, and not fine on a
/// public interface, where it would be an unauthenticated remote code execution
/// endpoint.
///
/// So the answer is derived from where the server is bound rather than from a
/// flag: binding anywhere but loopback is treated as opting in, and the app
/// says so loudly at boot instead of failing silently. Network-level isolation
/// is *not* attempted — see `services::sandbox` for what the mitigations do and
/// do not cover.
pub fn playground_allowed() -> bool {
    let opt_in = matches!(
        env::var("BORROW_ALLOW_PLAYGROUND").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    );

    playground_allowed_on(address().ip(), opt_in)
}

/// The decision itself, separated from the environment so it can be tested
/// without mutating process-wide state.
///
/// `opt_in` only means anything for a non-loopback address; on loopback the
/// playground is always available, because the only thing that can reach it is
/// the person sitting at the machine.
fn playground_allowed_on(ip: IpAddr, opt_in: bool) -> bool {
    match ip {
        IpAddr::V4(Ipv4Addr::LOCALHOST) => true,
        IpAddr::V6(Ipv6Addr::LOCALHOST) => true,
        _ => opt_in,
    }
}

#[cfg(test)]
mod playground_guard_tests {
    use super::*;

    /// The guard exists to stop the playground being reachable by anyone but
    /// the person at the keyboard, so every case here is a bind address that
    /// must refuse unless someone explicitly opts in.
    #[test]
    fn non_loopback_addresses_refuse_by_default() {
        let exposed: [SocketAddr; 4] = [
            "0.0.0.0:3000".parse::<SocketAddr>().unwrap(),
            "192.168.1.10:3000".parse::<SocketAddr>().unwrap(),
            "[::]:3000".parse::<SocketAddr>().unwrap(),
            "10.0.0.5:3000".parse::<SocketAddr>().unwrap(),
        ];

        for addr in exposed {
            assert!(
                !playground_allowed_on(addr.ip(), false),
                "binding {addr} must not arm the playground"
            );
        }
    }

    #[test]
    fn loopback_addresses_allow_the_playground() {
        for host in ["127.0.0.1:3000", "[::1]:3000"] {
            let addr: SocketAddr = host.parse().unwrap();
            assert!(
                playground_allowed_on(addr.ip(), false),
                "binding {host} should arm the playground"
            );
        }
    }

    #[test]
    fn a_non_loopback_bind_can_opt_in_explicitly() {
        let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
        assert!(playground_allowed_on(addr.ip(), true));
    }
}

#[cfg(test)]
mod data_dir_tests {
    use super::*;
    use std::path::PathBuf;

    fn some(path: &str) -> Option<PathBuf> {
        Some(PathBuf::from(path))
    }

    /// All three platforms' rules, exercised wherever the tests run: the
    /// installed app must land in a directory the user owns on every one of
    /// them, never beside the executable.
    #[test]
    fn each_platform_has_its_own_data_directory() {
        assert_eq!(
            data_dir_for(Platform::Mac, some("/Users/ada"), None, None),
            some("/Users/ada/Library/Application Support/BORROW")
        );
        assert_eq!(
            data_dir_for(
                Platform::Windows,
                None,
                None,
                some("C:/Users/ada/AppData/Roaming")
            ),
            some("C:/Users/ada/AppData/Roaming/BORROW")
        );
        assert_eq!(
            data_dir_for(Platform::Windows, some("C:/Users/ada"), None, None),
            some("C:/Users/ada/AppData/Roaming/BORROW")
        );
        assert_eq!(
            data_dir_for(Platform::Other, some("/home/ada"), some("/ada/.data"), None),
            some("/ada/.data/borrow")
        );
        assert_eq!(
            data_dir_for(Platform::Other, some("/home/ada"), None, None),
            some("/home/ada/.local/share/borrow")
        );
    }

    /// No home, no environment: `None`, and the caller falls back to the
    /// current directory rather than refusing to start.
    #[test]
    fn an_empty_environment_falls_back_instead_of_failing() {
        assert_eq!(data_dir_for(Platform::Mac, None, None, None), None);
        assert_eq!(data_dir_for(Platform::Windows, None, None, None), None);
        assert_eq!(data_dir_for(Platform::Other, None, None, None), None);
    }

    /// A home directory with a space in it is the normal case on every
    /// platform — which is exactly why the path never becomes a URL.
    #[test]
    fn a_home_with_spaces_is_a_path_not_a_url() {
        let path = data_dir_for(
            Platform::Windows,
            None,
            None,
            some("C:/Users/Ada Lovelace/AppData/Roaming"),
        )
        .expect("appdata is set")
        .join("borrow.db");

        // Compared as paths, not as strings: Windows writes `\\` where the
        // fixture writes `/`, and a string comparison fails on the platform
        // the rule is actually for. What the test is really about is that the
        // space survives the whole way to the file.
        assert_eq!(
            path,
            PathBuf::from("C:/Users/Ada Lovelace/AppData/Roaming")
                .join("BORROW")
                .join("borrow.db")
        );
        assert!(
            path.to_string_lossy().contains("Ada Lovelace"),
            "the space in the home directory survives: {path:?}"
        );
    }
}
