use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub fn database_url() -> String {
    env::var("BORROW_DB_URL").unwrap_or_else(|_| "sqlite:borrow.db?mode=rwc".to_string())
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
