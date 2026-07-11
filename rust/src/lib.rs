//! Aeluin SSoT for Rust.

/// High-level API to communicate with all Aeluin ecosystem.
pub mod aeluin {
    pub mod interop {
        pub mod v1 {
            include!("aeluin/interop/v1/aeluin.interop.v1.rs");
        }
    }
}

/// High-level API to communicate between drone swarm.
pub mod ringil {
    pub mod swarm {
        pub mod v1 {
            include!("ringil/swarm/v1/ringil.swarm.v1.rs");
        }
    }
}
