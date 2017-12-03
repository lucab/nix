//! Definitions for network database operations.

use libc;

libc_bitflags!(
    /// Address information flags.
    pub struct AddrInfoFlags: libc::c_int {
        /// Socket address is intended for bind().
        AI_PASSIVE;
        /// Request for canonical name.
        AI_CANONNAME;
        /// Return numeric host address as name.
        AI_NUMERICHOST;
        /// Inhibit service name resolution.
        AI_NUMERICSERV;
        /// If no IPv6 addresses are found, query for IPv4 addresses and return them to the caller as IPv4-mapped IPv6 addresses.
        AI_V4MAPPED;
        /// Query for both IPv4 and IPv6 addresses.
        AI_ALL;
        /// Query for IPv4 addresses only when an IPv4 address is configured;
        /// query for IPv6 addresses only when an IPv6 address is configured.
        AI_ADDRCONFIG;
    }
);

libc_bitflags!(
    /// Name information flags, used by `getnameinfo`.
    pub struct NameInfoFlags: libc::c_int {
        /// Only the nodename portion of the FQDN is returned for local hosts.
        NI_NOFQDN;
        /// The numeric form of the node's address is returned instead of its name.
        NI_NUMERICHOST;
        /// Return an error if the node's name cannot be located in the database.
        NI_NAMEREQD;
        /// The numeric form of the service address is returned instead of its name.
        NI_NUMERICSERV;
        // For IPv6 addresses, the numeric form of the scope identifier is returned instead of its name.
        // NI_NUMERICSCOPE;
        /// Indicates that the service is a datagram service (SOCK_DGRAM).
        NI_DGRAM;
    }
);
