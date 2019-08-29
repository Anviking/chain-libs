use crate::ledger::Error;
use crate::value::{Value, ValueError};
use crate::treasury::Treasury;

/// Special pots of money
#[derive(Clone, PartialEq, Eq)]
pub struct Pots {
    pub(crate) fees: Value,
    pub(crate) treasury: Treasury,
}

#[derive(Debug, Clone, Copy)]
pub enum Entry {
    Fees(Value),
    Treasury(Value),
}

#[derive(Debug, Clone, Copy)]
pub enum EntryType {
    Fees,
    Treasury,
}

impl Entry {
    pub fn value(&self) -> Value {
        match self {
            Entry::Fees(v) => *v,
            Entry::Treasury(v) => *v,
        }
    }

    pub fn entry_type(&self) -> EntryType {
        match self {
            Entry::Fees(_) => EntryType::Fees,
            Entry::Treasury(_) => EntryType::Treasury,
        }
    }
}

pub enum IterState {
    Fees,
    Treasury,
    Done,
}

pub struct Entries<'a> {
    pots: &'a Pots,
    it: IterState,
}

pub struct Values<'a>(Entries<'a>);

impl<'a> Iterator for Entries<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.it {
            IterState::Fees => {
                self.it = IterState::Treasury;
                Some(Entry::Fees(self.pots.fees))
            }
            IterState::Treasury => {
                self.it = IterState::Done;
                Some(Entry::Treasury(self.pots.treasury.value()))
            }
            IterState::Done => {
                None
            }
        }
    }
}

impl<'a> Iterator for Values<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(e) => Some(e.value()),
        }
    }
}

impl Pots {
    /// Create a new empty set of pots
    pub fn zero() -> Self {
        Pots {
            fees: Value::zero(),
            treasury: Treasury::initial(Value::zero()),
        }
    }

    pub fn entries<'a>(&'a self) -> Entries<'a> {
        Entries {
            pots: self,
            it: IterState::Fees,
        }
    }

    pub fn values<'a>(&'a self) -> Values<'a> {
        Values(self.entries())
    }

    /// Sum the total values in the pots
    pub fn total_value(&self) -> Result<Value, ValueError> {
        Value::sum(self.values())
    }

    /// Append some fees in the pots
    pub fn append_fees(&mut self, fees: Value) -> Result<(), Error> {
        self.fees = (self.fees + fees).map_err(|error| Error::PotValueInvalid { error })?;
        Ok(())
    }

    pub fn set_from_entry(&mut self, e: &Entry) {
        match e {
            Entry::Fees(v) => self.fees = *v,
            Entry::Treasury(v) => self.treasury = Treasury::initial(*v),
        }
    }
}