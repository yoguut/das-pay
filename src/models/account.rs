use serde::Serialize;
use std::fmt;

/// client,available,held,total,locked
/// 1,1.5,0.0,false
/// 2,2.0,0.0,false
#[derive(Debug, Serialize)]
pub struct Account {
    #[serde(rename(serialize = "client"))]
    pub id: u16,
    pub held: f32,
    pub available: f32,
    pub locked: bool,
    pub total: f32,
}

impl Account {
    pub fn new(id: u16, held: f32, available: f32, locked: bool) -> Self {
        Account { id: id, held: held, available: available, locked: locked, total: held + available }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "client/id: {}, held: {}, available: {}, locked: {}", self.id, self.held, self.available, self.locked)
    }
}
