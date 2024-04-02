use super::Identifier;
use crate::{
    native_types::{
        pthread_barrier_t, pthread_cond_t, pthread_mutex_t, pthread_once_t, pthread_rwlock_t,
        pthread_spinlock_t,
    },
    TypeValueEncodable,
};

/// Identifiers used for PThread resources
pub enum PThreadIdentifier {
    /// PThread mutex
    Mutex(*const pthread_mutex_t),
    /// PThread condition_variable
    Condition(*const pthread_cond_t),
    /// PThread rwlock
    RWLock(*const pthread_rwlock_t),
    /// PThread barrier
    Barrier(*const pthread_barrier_t),
    /// PThread spinlock
    Spinlock(*const pthread_spinlock_t),
    /// PThread once
    Once(*const pthread_once_t),
}

impl From<PThreadIdentifier> for Identifier {
    fn from(value: PThreadIdentifier) -> Self {
        Self::PThread(value)
    }
}

impl TypeValueEncodable for PThreadIdentifier {
    type Type = u32;
    type Value = nvtx_sys::ResourceAttributesIdentifier;

    fn encode(&self) -> (Self::Type, Self::Value) {
        match self {
            Self::Mutex(m) => (
                nvtx_sys::resource_type::PTHREAD_MUTEX,
                Self::Value { pValue: m.cast() },
            ),
            Self::Condition(cv) => (
                nvtx_sys::resource_type::PTHREAD_CONDITION,
                Self::Value { pValue: cv.cast() },
            ),
            Self::RWLock(rwl) => (
                nvtx_sys::resource_type::PTHREAD_RWLOCK,
                Self::Value { pValue: rwl.cast() },
            ),
            Self::Barrier(bar) => (
                nvtx_sys::resource_type::PTHREAD_BARRIER,
                Self::Value { pValue: bar.cast() },
            ),
            Self::Spinlock(s) => (
                nvtx_sys::resource_type::PTHREAD_SPINLOCK,
                Self::Value { pValue: s.cast() },
            ),
            Self::Once(o) => (
                nvtx_sys::resource_type::PTHREAD_ONCE,
                Self::Value { pValue: o.cast() },
            ),
        }
    }

    fn default_encoding() -> (Self::Type, Self::Value) {
        Identifier::default_encoding()
    }
}