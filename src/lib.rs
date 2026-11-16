// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-layer-6: layer 6 component model.
//!
//! Real implementation: layers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
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
pub enum LayerSixError {
    EmptyLayerName,
    DuplicateLayer(String),
    EmptyEventSubject,
}

impl fmt::Display for LayerSixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLayerName => write!(f, "layer name must not be empty"),
            Self::DuplicateLayer(id) => write!(f, "duplicate layer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LayerSixError {}

pub struct LayerSix {
    pub name: String,
    layers: BTreeMap<String, Layer>,
    events: BTreeMap<String, Event>,
}

impl LayerSix {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_layer(&mut self, item: Layer) -> Result<(), LayerSixError> {
        if item.name.trim().is_empty() {
            return Err(LayerSixError::EmptyLayerName);
        }
        let id = item.id.clone();
        if self.layers.contains_key(&id) {
            return Err(LayerSixError::DuplicateLayer(id.clone()));
        }
        self.layers.insert(id, item);
        Ok(())
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_layer(&self, id: &str) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LayerSixError> {
        if event.subject.trim().is_empty() {
            return Err(LayerSixError::EmptyEventSubject);
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

    fn sample_layer(id: &str, name: &str) -> Layer {
        Layer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LayerSix::new("LayerSix");
        assert_eq!(m.name, "LayerSix");
        assert_eq!(m.layer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_layer() {
        let mut m = LayerSix::new("LayerSix");
        m.add_layer(sample_layer("a1", "Alpha")).unwrap();
        assert_eq!(m.layer_count(), 1);
        assert!(m.get_layer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LayerSix::new("LayerSix");
        assert!(m.add_layer(sample_layer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LayerSix::new("LayerSix");
        m.add_layer(sample_layer("x", "One")).unwrap();
        assert!(m.add_layer(sample_layer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LayerSix::new("LayerSix");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
