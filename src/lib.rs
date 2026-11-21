// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-wrapper: mint service wrapper component model.
//!
//! Real implementation: wrappers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapper {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintServiceWrapperError {
    EmptyWrapperName,
    DuplicateWrapper(String),
    EmptyEventSubject,
}

impl fmt::Display for MintServiceWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWrapperName => write!(f, "wrapper name must not be empty"),
            Self::DuplicateWrapper(id) => write!(f, "duplicate wrapper {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintServiceWrapperError {}

pub struct MintServiceWrapper {
    pub name: String,
    wrappers: BTreeMap<String, Wrapper>,
    events: BTreeMap<String, Event>,
}

impl MintServiceWrapper {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            wrappers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_wrapper(&mut self, item: Wrapper) -> Result<(), MintServiceWrapperError> {
        if item.name.trim().is_empty() {
            return Err(MintServiceWrapperError::EmptyWrapperName);
        }
        let id = item.id.clone();
        if self.wrappers.contains_key(&id) {
            return Err(MintServiceWrapperError::DuplicateWrapper(id.clone()));
        }
        self.wrappers.insert(id, item);
        Ok(())
    }

    pub fn wrapper_count(&self) -> usize {
        self.wrappers.len()
    }

    pub fn get_wrapper(&self, id: &str) -> Option<&Wrapper> {
        self.wrappers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintServiceWrapperError> {
        if event.subject.trim().is_empty() {
            return Err(MintServiceWrapperError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wrapper(id: &str, name: &str) -> Wrapper {
        Wrapper {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintServiceWrapper::new("MintServiceWrapper");
        assert_eq!(m.name, "MintServiceWrapper");
        assert_eq!(m.wrapper_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_wrapper() {
        let mut m = MintServiceWrapper::new("MintServiceWrapper");
        m.add_wrapper(sample_wrapper("a1", "Alpha")).unwrap();
        assert_eq!(m.wrapper_count(), 1);
        assert!(m.get_wrapper("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintServiceWrapper::new("MintServiceWrapper");
        assert!(m.add_wrapper(sample_wrapper("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintServiceWrapper::new("MintServiceWrapper");
        m.add_wrapper(sample_wrapper("x", "One")).unwrap();
        assert!(m.add_wrapper(sample_wrapper("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintServiceWrapper::new("MintServiceWrapper");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
