use std::collections::HashMap;

pub struct Owner {
    name: String,
    investment: f32,
}

impl Owner {
    pub fn new(name: String, investment: f32) -> Self {
        Self { name, investment }
    }
}

pub struct Corporation {
    pub name: String,
    pub owners: Vec<Owner>,
    pub total_investment: f32,
    pub total_shares: u64, // max_val 18_446_744_073_709_551_615,
}

impl Corporation {
    pub fn new(owners: Vec<Owner>, name: Option<String>, total_shares: Option<u64>) -> Self {
        let total_investment = Self::get_total_investments(&owners);
        Self {
            name: match name {
                Some(name) => name,
                None => "".to_string(),
            },
            owners,
            total_investment,
            total_shares: match total_shares {
                Some(total_shares) => total_shares,
                None => 0,
            },
        }
    }

    pub fn get_total_investments(owners: &Vec<Owner>) -> f32 {
        owners.iter().fold(0.0, |acc, owner| acc + owner.investment)
    }

    pub fn get_owner_ownership_percentage_by_investment(&self, owner_name: &str) -> f32 {
        let found_owner = self.owners.iter().find(|owner| owner.name == owner_name);

        match found_owner {
            Some(found_owner) => ((found_owner.investment / self.total_investment) * 100.0).round(),
            None => 0.0,
        }
    }

    pub fn get_owners_ownership_percentages(&self) -> Vec<HashMap<String, f32>> {
        let owners_ownership_percentages = self
            .owners
            .iter()
            .map(|owner| {
                let mut book_hashmap = HashMap::new();
                book_hashmap.insert(
                    owner.name.clone(),
                    (owner.investment / self.total_investment * 100.0).round(),
                );
                book_hashmap
            })
            .collect::<Vec<HashMap<String, f32>>>();
        owners_ownership_percentages
    }

    pub fn get_owner_shares_by_ownership_percentage(&self, owner_name: &str) -> u64 {
        let ownership_percentage =
            self.get_owner_ownership_percentage_by_investment(owner_name) as u64;
        (ownership_percentage * self.total_shares) / 100
    }

    pub fn is_owner_majority_shareholder(&self, owner_name: &str) -> bool {
        let ownership_percentage =
            self.get_owner_ownership_percentage_by_investment(owner_name) as u64;

        if ownership_percentage >= 50 {
            return true;
        }

        false
    }
}
