// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-auth-agents: auth agents component model.
//!
//! Real implementation: agents, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agent {
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
pub enum AuthAgentsError {
    EmptyAgentName,
    DuplicateAgent(String),
    EmptyEventSubject,
}

impl fmt::Display for AuthAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAgentName => write!(f, "agent name must not be empty"),
            Self::DuplicateAgent(id) => write!(f, "duplicate agent {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AuthAgentsError {}

pub struct AuthAgents {
    pub name: String,
    agents: BTreeMap<String, Agent>,
    events: BTreeMap<String, Event>,
}

impl AuthAgents {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agents: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_agent(&mut self, item: Agent) -> Result<(), AuthAgentsError> {
        if item.name.trim().is_empty() {
            return Err(AuthAgentsError::EmptyAgentName);
        }
        let id = item.id.clone();
        if self.agents.contains_key(&id) {
            return Err(AuthAgentsError::DuplicateAgent(id.clone()));
        }
        self.agents.insert(id, item);
        Ok(())
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn get_agent(&self, id: &str) -> Option<&Agent> {
        self.agents.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AuthAgentsError> {
        if event.subject.trim().is_empty() {
            return Err(AuthAgentsError::EmptyEventSubject);
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

    fn sample_agent(id: &str, name: &str) -> Agent {
        Agent {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AuthAgents::new("AuthAgents");
        assert_eq!(m.name, "AuthAgents");
        assert_eq!(m.agent_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_agent() {
        let mut m = AuthAgents::new("AuthAgents");
        m.add_agent(sample_agent("a1", "Alpha")).unwrap();
        assert_eq!(m.agent_count(), 1);
        assert!(m.get_agent("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AuthAgents::new("AuthAgents");
        assert!(m.add_agent(sample_agent("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AuthAgents::new("AuthAgents");
        m.add_agent(sample_agent("x", "One")).unwrap();
        assert!(m.add_agent(sample_agent("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AuthAgents::new("AuthAgents");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
