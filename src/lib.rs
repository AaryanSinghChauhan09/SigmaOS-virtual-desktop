// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-virtual-desktop: virtual desktop component model.
//!
//! Real implementation: desktops, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Desktop {
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
pub enum VirtualDesktopError {
    EmptyDesktopName,
    DuplicateDesktop(String),
    EmptyEventSubject,
}

impl fmt::Display for VirtualDesktopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDesktopName => write!(f, "desktop name must not be empty"),
            Self::DuplicateDesktop(id) => write!(f, "duplicate desktop {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VirtualDesktopError {}

pub struct VirtualDesktop {
    pub name: String,
    desktops: BTreeMap<String, Desktop>,
    events: BTreeMap<String, Event>,
}

impl VirtualDesktop {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            desktops: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_desktop(&mut self, item: Desktop) -> Result<(), VirtualDesktopError> {
        if item.name.trim().is_empty() {
            return Err(VirtualDesktopError::EmptyDesktopName);
        }
        let id = item.id.clone();
        if self.desktops.contains_key(&id) {
            return Err(VirtualDesktopError::DuplicateDesktop(id.clone()));
        }
        self.desktops.insert(id, item);
        Ok(())
    }

    pub fn desktop_count(&self) -> usize {
        self.desktops.len()
    }

    pub fn get_desktop(&self, id: &str) -> Option<&Desktop> {
        self.desktops.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VirtualDesktopError> {
        if event.subject.trim().is_empty() {
            return Err(VirtualDesktopError::EmptyEventSubject);
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

    fn sample_desktop(id: &str, name: &str) -> Desktop {
        Desktop {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = VirtualDesktop::new("VirtualDesktop");
        assert_eq!(m.name, "VirtualDesktop");
        assert_eq!(m.desktop_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_desktop() {
        let mut m = VirtualDesktop::new("VirtualDesktop");
        m.add_desktop(sample_desktop("a1", "Alpha")).unwrap();
        assert_eq!(m.desktop_count(), 1);
        assert!(m.get_desktop("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = VirtualDesktop::new("VirtualDesktop");
        assert!(m.add_desktop(sample_desktop("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = VirtualDesktop::new("VirtualDesktop");
        m.add_desktop(sample_desktop("x", "One")).unwrap();
        assert!(m.add_desktop(sample_desktop("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = VirtualDesktop::new("VirtualDesktop");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
