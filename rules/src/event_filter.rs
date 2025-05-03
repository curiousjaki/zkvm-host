use super::*;

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct FollowedBy {
    pub prior: [u32; 8],
    pub next: [u32; 8],
}

pub trait InsertEvent {
    fn insert_ordered_event(
        &mut self,
        event: [u32; 8],
        rules: Vec<&OrderingRule>,
    ) -> Result<(), qfilter::Error>;
    fn insert_event(&mut self, event: [u32; 8]) -> Result<(), qfilter::Error>;
}

impl InsertEvent for qfilter::Filter {
    /// Inserts an event into the filter and adds a FollowedBy relation if an ordering rule is fulfilled
    fn insert_ordered_event(
        &mut self,
        event: [u32; 8],
        rules: Vec<&OrderingRule>,
    ) -> Result<(), qfilter::Error> {
        for rule in rules.iter() {
            if rule.next == event {
                if self.contains(rule.prior) {
                    self.insert_duplicated(FollowedBy {
                        prior: rule.prior,
                        next: rule.next,
                    })
                    .unwrap();
                }
            }
        }
        return self.insert_duplicated(event);
    }
    /// Inserts an event into the filter and adds a FollowedBy relation if an ordering rule is fulfilled
    fn insert_event(&mut self, event: [u32; 8]) -> Result<(), qfilter::Error> {
        return self.insert_duplicated(event);
    }
}
