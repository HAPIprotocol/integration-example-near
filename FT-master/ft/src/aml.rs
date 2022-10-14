use crate::*;

pub trait CategoryManagment {
    fn update_category(&mut self, category: Category, risk_score: RiskScore);
    fn remove_category(&mut self, category: Category);
}

#[near_bindgen]
impl CategoryManagment for Contract {
    fn update_category(&mut self, category: Category, risk_score: RiskScore) {
        self.assert_owner();
        self.aml.update_category(category, risk_score);
    }

    fn remove_category(&mut self, category: Category) {
        self.assert_owner();
        self.aml.remove_category(category);
    }
}
