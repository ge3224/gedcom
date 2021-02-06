use crate::types::{event::HasEvents, Event};

type Xref = String;

/// Family fact, representing a relationship between `Individual`s
///
/// This data representation understands that HUSB & WIFE are just poorly-named
/// pointers to individals. no gender "validating" is done on parse.
#[derive(Debug, HasEvents)]
pub struct Family {
    pub xref: Option<Xref>,
    pub individual1: Option<Xref>, // mapped from HUSB
    pub individual2: Option<Xref>, // mapped from WIFE
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
    events: Vec<Event>,
}

impl Family {
    #[must_use]
    pub fn new(xref: Option<Xref>) -> Family {
        Family {
            xref,
            individual1: None,
            individual2: None,
            children: Vec::new(),
            num_children: None,
            events: Vec::new(),
        }
    }

    pub fn set_individual1(&mut self, xref: Xref) {
        match self.individual1 {
            Some(_) => panic!("First individual of family already exists."),
            None => self.individual1 = Some(xref),
        };
    }

    pub fn set_individual2(&mut self, xref: Xref) {
        match self.individual2 {
            Some(_) => panic!("Second individual of family already exists."),
            None => self.individual2 = Some(xref),
        };
    }

    pub fn add_child(&mut self, xref: Xref) {
        self.children.push(xref);
    }

    pub fn add_event(&mut self, event: Event) {
        let event_type = &event.event;
        for e in &self.events {
            if &e.event == event_type {
                panic!("Family already has a {:?} event", e.event);
            }
        }
        self.events.push(event);
    }
}
