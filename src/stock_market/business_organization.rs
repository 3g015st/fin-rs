use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum HashMapValue {
    Str(String),
    Float32(f32),
}

impl HashMapValue {
    pub fn str(&self) -> &String {
        match self {
            HashMapValue::Str(v) => v,
            _ => panic!("Not HashMapValue::Str"),
        }
    }
    pub fn f32(&self) -> &f32 {
        match self {
            HashMapValue::Float32(v) => v,
            _ => panic!("Not HashMapValue::Str"),
        }
    }
}

#[derive(Debug)]
pub struct Owner {
    name: String,
    investment: f32,
}

impl Owner {
    pub fn new(name: String, investment: f32) -> Self {
        Self { name, investment }
    }
}

#[derive(Debug)]
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

    pub fn get_owners_ownership_percentages(&self) -> Vec<HashMap<String, HashMapValue>> {
        let owners_ownership_percentages = self
            .owners
            .iter()
            .map(|owner| {
                let mut owner_hashmap: HashMap<String, HashMapValue> = HashMap::new();
                owner_hashmap.insert("name".to_string(), HashMapValue::Str(owner.name.clone()));
                owner_hashmap.insert(
                    "percentage".to_string(),
                    HashMapValue::Float32(
                        (owner.investment / self.total_investment * 100.0).round(),
                    ),
                );

                owner_hashmap
            })
            .collect::<Vec<HashMap<String, HashMapValue>>>();
        owners_ownership_percentages
    }

    pub fn get_owner_shares_by_ownership_percentage(&self, owner_name: &str) -> u64 {
        let ownership_percentage =
            self.get_owner_ownership_percentage_by_investment(owner_name) as u64;
        (ownership_percentage * self.total_shares) / 100
    }

    pub fn is_owner_majority_shareholder(&self, owner_name: &str) -> bool {
        let owners_ownership_percentages = self.get_owners_ownership_percentages();
        let mut major_shareholder: HashMap<String, HashMapValue> = HashMap::new();

        let first_owner = &owners_ownership_percentages[0];

        println!("{:?}", first_owner);
        for key in first_owner.keys() {
            let key = key.to_string();

            let value_enum = first_owner.get(&key).clone().unwrap();

            match value_enum {
                HashMapValue::Str(value_enum) => {
                    major_shareholder.insert(key, HashMapValue::Str(value_enum.to_string()))
                }
                HashMapValue::Float32(value_enum) => {
                    major_shareholder.insert(key, HashMapValue::Float32(*value_enum))
                }
            };
        }

        let major_shareholder =
            owners_ownership_percentages
                .iter()
                .fold(major_shareholder, |mut acc, owner| {
                    let current_major_shareholder_percentage = acc.get("percentage").unwrap().f32();
                    let owner_percentage = owner.get("percentage").unwrap().f32();
                    let o_name = owner.get("name").unwrap().str();

                    if current_major_shareholder_percentage < owner_percentage {
                        acc.insert("name".to_string(), HashMapValue::Str(o_name.to_string()));
                        acc.insert(
                            "percentage".to_string(),
                            HashMapValue::Float32(owner_percentage.clone()),
                        );
                        return acc;
                    } else {
                        return acc;
                    }
                });

        let major_share_holder_name = major_shareholder.get("name").unwrap().str();

        major_share_holder_name == owner_name
    }
}
