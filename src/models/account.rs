use serde::Serialize;
use std::fmt;

/// client,available,held,total,locked
/// 1,1.5,0.0,false
/// 2,2.0,0.0,false
#[derive(Debug, Clone, Serialize)]
pub struct Account {
    #[serde(rename(serialize = "client"))]
    id: u16,
    held: f32,
    available: f32,
    pub locked: bool,
    total: f32,
}

impl Account {
    pub fn new(id: u16, held: f32, available: f32, locked: bool) -> Self {
        Account {
            id: id,
            held: held,
            available: available,
            locked: locked,
            total: held + available,
        }
    }

    pub fn get_held(&self) -> f32 {
        self.held
    }

    pub fn set_held(&mut self, val: f32) {
        if !self.locked {
            self.held = val;
            self.total = self.held + self.available;
        }
    }

    pub fn get_available(&self) -> f32 {
        self.available
    }

    pub fn set_available(&mut self, val: f32) {
        if !self.locked {
            self.available = val;
            self.total = self.held + self.available;
        }
    }

    pub fn rounded(&self, decimal_places: u32) -> Self {
        Account {
            id: self.id,
            held: _round(self.held, decimal_places),
            available: _round(self.available, decimal_places),
            locked: self.locked,
            total: _round(self.total, decimal_places),
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "client/id: {}, held: {}, available: {}, locked: {}, total: {:.4}",
            self.id, self.held, self.available, self.locked, self.total
        )
    }
}

fn _round(val: f32, decimal_places: u32) -> f32 {
    (val * 10.0_f32.powf(decimal_places as f32)).round() / (10.0_f32.powf(decimal_places as f32))
}

#[cfg(test)]
mod account_tests {
    use super::*;

    #[test]
    fn should_update_total() {
        let mut acc = Account::new(999_u16, 0_f32, 0_f32, false);
        assert_eq!(acc.total, 0_f32);
        acc.set_available(3_f32);
        assert_eq!(acc.total, 3_f32);
        acc.set_held(2_f32);
        assert_eq!(acc.total, 5_f32);
    }

    #[test]
    fn should_not_update_funds_when_locked() {
        let mut acc = Account::new(999_u16, 0_f32, 0_f32, true);
        acc.set_available(3_f32);
        assert_eq!(acc.total, 0_f32);
        acc.set_held(2_f32);
        assert_eq!(acc.total, 0_f32);
    }

    #[test]
    fn should_round_funds() {
        let acc = Account::new(999_u16, 25.12345_f32, 25.12345_f32, false);
        assert_eq!(acc.available, 25.12345_f32);
        assert_eq!(acc.held, 25.12345_f32);
        assert_eq!(acc.total, 50.24690_f32);
        let rounded_acc = acc.rounded(4);
        assert_eq!(rounded_acc.available, 25.1235_f32);
        assert_eq!(rounded_acc.held, 25.1235_f32);
        assert_eq!(rounded_acc.total, 50.2469_f32);
    }
}
