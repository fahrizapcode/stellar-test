#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Outlet {
    pub id: u64,
    pub brand_name: String,
    pub operator: Address,
    pub investor: Address,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct RevenueReport {
    pub id: u64,
    pub outlet_id: u64,
    pub total_revenue: i128,
    pub brand_share: i128,
    pub investor_share: i128,
    pub operator_share: i128,
    pub platform_share: i128,
    pub timestamp: u64,
}

const OUTLET_DATA: Symbol = symbol_short!("OUTLETS");
const REPORT_DATA: Symbol = symbol_short!("REPORTS");

#[contract]
pub struct OutletContract;

#[contractimpl]
impl OutletContract {
    pub fn get_outlets(env: Env) -> Vec<Outlet> {
        env.storage()
            .instance()
            .get(&OUTLET_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_reports(env: Env) -> Vec<RevenueReport> {
        env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn register_outlet(
        env: Env,
        brand_name: String,
        operator: Address,
        investor: Address,
    ) -> String {
        let mut outlets: Vec<Outlet> = env
            .storage()
            .instance()
            .get(&OUTLET_DATA)
            .unwrap_or(Vec::new(&env));

        let outlet = Outlet {
            id: env.prng().gen::<u64>(),
            brand_name,
            operator,
            investor,
        };

        outlets.push_back(outlet.clone());
        env.storage().instance().set(&OUTLET_DATA, &outlets);

        env.events().publish(
            (symbol_short!("OUTLET"),),
            outlet.id,
        );

        String::from_str(&env, "Outlet berhasil didaftarkan")
    }

    pub fn submit_revenue(
        env: Env,
        outlet_id: u64,
        total_revenue: i128,
        brand_share: i128,
        investor_share: i128,
        operator_share: i128,
        platform_share: i128,
    ) -> String {
        let mut reports: Vec<RevenueReport> = env
            .storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        let report = RevenueReport {
            id: env.prng().gen::<u64>(),
            outlet_id,
            total_revenue,
            brand_share,
            investor_share,
            operator_share,
            platform_share,
            timestamp: env.ledger().timestamp(),
        };

        reports.push_back(report.clone());
        env.storage().instance().set(&REPORT_DATA, &reports);

        env.events().publish(
            (symbol_short!("REVENUE"),),
            report.id,
        );

        String::from_str(&env, "Revenue report berhasil dicatat")
    }
}

mod test;