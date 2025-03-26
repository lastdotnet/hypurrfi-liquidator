use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_aave::{
    i_aave_oracle::IAaveOracle,
    i_pool_data_provider::IPoolDataProvider,
    ierc20::IERC20,
    pool::{BorrowFilter, Pool, SupplyFilter},
};
use bindings_liquidator::liquidator::Liquidator;
use clap::{Parser, ValueEnum};
use ethers::{
    abi::{encode_packed, Token},
    contract::builders::ContractCall,
    providers::Middleware,
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes, ValueOrArray, I256, U256, U64,
    },
};
use ethers_contract::Multicall;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, info};
use crate::strategies::liq_path_config::LiqPathConfig;
use hex;

use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    state_cache_file: String,
    pool_address: Address,
    pool_data_provider: Address,
    oracle_address: Address,
    whype_address: Address,
    multicall3_address: Address,
    usdxl_address: Address,
    liq_paths_config_file: String,
    default_liq_path: String,
    creation_block: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    MOCKNET,
    HYFI,
}

pub const LIQUIDATION_CLOSE_FACTOR_THRESHOLD: &str = "950000000000000000";
pub const MAX_LIQUIDATION_CLOSE_FACTOR: u64 = 10000;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: u64 = 5000;

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1000;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const PRICE_ONE: u64 = 100_000_000;

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::MOCKNET => DeploymentConfig {
            state_cache_file: "borrowers-mocknet.json".to_string(),
            pool_address: Address::from_str("0x32467b43BFa67273FC7dDda0999Ee9A12F2AaA08").unwrap(),
            pool_data_provider: Address::from_str("0x0B306BF915C4d645ff596e518fAf3F9669b97016").unwrap(),
            oracle_address: Address::from_str("0x0E801D84Fa97b50751Dbf25036d067dCf18858bF").unwrap(),
            whype_address: Address::from_str("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0").unwrap(),
            multicall3_address: Address::from_str("0x720472c8ce72c2A2D711333e064ABD3E6BbEAdd3").unwrap(),
            usdxl_address: Address::from_str("0xca79db4B49f608eF54a5CB813FbEd3a6387bC645").unwrap(),
            liq_paths_config_file: "liq_paths.json".to_string(),
            default_liq_path: "hyperswap".to_string(),
            creation_block: 0,
        },
        Deployment::HYFI => DeploymentConfig {
            state_cache_file: "borrowers-hyperevm-mainnet.json".to_string(),
            pool_address: Address::from_str("0xceCcE0EB9DD2Ef7996e01e25DD70e461F918A14b").unwrap(),
            pool_data_provider: Address::from_str("0x895C799a5bbdCb63B80bEE5BD94E7b9138D977d6").unwrap(),
            oracle_address: Address::from_str("0x9BE2ac1ff80950DCeb816842834930887249d9A8").unwrap(),
            whype_address: Address::from_str("0x5555555555555555555555555555555555555555").unwrap(),
            multicall3_address: Address::from_str("0xA66AEb1c0A579Ad95bA3940d18FAad02C368A383").unwrap(),
            usdxl_address: Address::from_str("0xca79db4B49f608eF54a5CB813FbEd3a6387bC645").unwrap(),
            liq_paths_config_file: "liq_paths.json".to_string(),
            default_liq_path: "kittenswap".to_string(),
            creation_block: 82245,
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
}

#[derive(Debug)]
pub struct PoolState {
    pub prices: HashMap<Address, U256>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    collateral: HashSet<Address>,
    debt: HashSet<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    address: Address,
    a_address: Address,
    decimals: u64,
    ltv: u64,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    reserve_factor: u64,
    protocol_fee: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AaveStrategy<M> {
    /// Ethers client.
    archive_client: Arc<M>,
    write_client: Arc<M>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    tokens: HashMap<Address, TokenConfig>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    pool_state: PoolState,
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    pub fn new(
        archive_client: Arc<M>,
        write_client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
    ) -> Self {
        Self {
            archive_client,
            write_client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            tokens: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment),
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
            pool_state: PoolState { prices: HashMap::new() },
        }
    }
}

struct LiquidationOpportunity {
    borrower: Address,
    collateral: Address,
    collateral_to_liquidate: U256,
    debt: Address,
    debt_to_cover: U256,
    profit_usd: I256,
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for AaveStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_token_configs().await?;
        self.approve_tokens().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            // Event::NewBlock(block) => self.process_new_block_event(block).await,
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    /// Process new block events, updating the internal state.
    // async fn process_new_block_event(&mut self, event: NewBlock) -> Option<Action> {
    //     info!("received new block: {:?}", event);
    //     self.last_block_number = event.number.as_u64();
    //     None
    // }

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Vec<Action> {
        info!("received new tick: {:?}", event);
        if let Err(e) = self.update_state().await {
            error!("Update State error: {}", e);
            return vec![];
        }

        info!("Total borrower count: {}", self.borrowers.len());
        let op = match self.get_best_liquidation_op().await {
            Ok(Some(op)) => op,
            Ok(None) => {
                info!("No liquidation opportunities found");
                return vec![];
            }
            Err(e) => {
                error!("Error finding liq ops: {}", e);
                return vec![];
            }
        };

        info!("Best op - profit: {}", op.profit_usd);

        if op.profit_usd < I256::from(0) {
            info!("No profitable ops, passing");
            return vec![];
        }

        let tx = match self.build_liquidation(&op).await {
            Ok(tx) => tx,
            Err(e) => {
                error!("Error building liquidation: {}", e);
                return vec![];
            }
        };

        let total_profit = match U256::from_dec_str(&op.profit_usd.to_string()) {
            Ok(profit) => profit,
            Err(e) => {
                error!("Failed to convert profit: {}", e);
                return vec![];
            }
        };

        vec![Action::SubmitTx(SubmitTxToMempool {
            tx,
            gas_bid_info: Some(GasBidInfo {
                bid_percentage: self.bid_percentage,
                total_profit,
            }),
        })]
    }

    // for all known borrowers, return a sorted set of those with health factor < 1
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256)>> {
        info!("Getting underwater borrowers");
        let pool = Pool::<M>::new(self.config.pool_address, self.write_client.clone());

        let mut underwater_borrowers = Vec::new();

        // call pool.getUserAccountData(user) for each borrower
        info!("Getting multicall");
        let mut multicall = Multicall::new(
            self.write_client.clone(),
            Some(self.config.multicall3_address.into()), 
        )
        .await?;
        info!("Getting borrowers");
        let borrowers: Vec<&Borrower> = self
            .borrowers
            .values()
            .filter(|b| b.debt.len() > 0)
            .collect();

        for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
            multicall.clear_calls();

            for borrower in chunk {
                multicall.add_call(pool.get_user_account_data(borrower.address), false);
            }

            let result: Vec<(U256, U256, U256, U256, U256, U256)> = multicall.call_array().await?;
            for (borrower, (_, _, _, _, _, health_factor)) in zip(chunk, result) {
                // info!("Checking borrower {:?}", borrower.address);
                if health_factor.lt(&U256::from_dec_str("1000000000000000000").unwrap()) {
                    info!(
                        "Found underwater borrower {:?} -  healthFactor: {}",
                        borrower, health_factor
                    );
                    underwater_borrowers.push((borrower.address, health_factor));
                } else {
                    // info!("Borrower {:?} is not underwater; healthFactor: {}", borrower.address, health_factor);
                }
            }
        }

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(self.config.state_cache_file.clone()) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.creation_block;
            }
        };

        Ok(())
    }

    fn write_intermediate_cache(&self, block_number: u64) {
        
        let cache = StateCache {
            last_block_number: block_number,
            borrowers: self.borrowers.clone(),
        };
        if let Err(e) = File::create(self.config.state_cache_file.clone())
            .and_then(|mut file| file.write_all(serde_json::to_string(&cache)?.as_bytes()))
        {
            error!("Failed to write intermediate cache: {}", e);
        }
    }

    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.archive_client.get_block_number().await?;
        info!(
            "Updating state from block {} to {}",
            self.last_block_number, latest_block
        );

        let mut borrower_count = 0;
        const BLOCK_CHUNK_SIZE: u64 = 1_000_000;

        // Process blocks in 1M chunks
        for chunk_start in (self.last_block_number..latest_block.as_u64()).step_by(BLOCK_CHUNK_SIZE as usize) {
            let chunk_end = std::cmp::min(chunk_start + BLOCK_CHUNK_SIZE, latest_block.as_u64());
            // info!(
            //     "Processing borrow logs for blocks {} to {}",
            //     chunk_start, chunk_end
            // );

            self.get_borrow_logs(chunk_start.into(), chunk_end.into())
                .await?
                .into_iter()
                .for_each(|log| {
                    let user = log.on_behalf_of;
                    if self.borrowers.contains_key(&user) {
                        let borrower = self.borrowers.get_mut(&user).unwrap();
                        borrower.debt.insert(log.reserve);

                        self.write_intermediate_cache(chunk_end);
                    } else {
                        self.borrowers.insert(
                            user,
                            Borrower {
                                address: user,
                                collateral: HashSet::new(),
                                debt: HashSet::from([log.reserve]),
                            },
                        );
                        borrower_count += 1;

                        self.write_intermediate_cache(chunk_end);
                    }
                });
        }

        info!(
            "Got borrow logs from {} to {}",
            self.last_block_number, latest_block
        );

        info!("Getting supply logs");

        for chunk_start in (self.last_block_number..latest_block.as_u64()).step_by(BLOCK_CHUNK_SIZE as usize) {
            let chunk_end = std::cmp::min(chunk_start + BLOCK_CHUNK_SIZE, latest_block.as_u64());
            info!("Getting supply logs from {} to {}", chunk_start, chunk_end);
            self.get_supply_logs(chunk_start.into(), chunk_end.into())
                .await?
                .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                if self.borrowers.contains_key(&user) {
                    // info!("Found borrower with collateral {:?}", log.reserve);
                    self.write_intermediate_cache(chunk_end);
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                        borrower.collateral.insert(log.reserve);
                    }
                });
        }

        info!("Got supply logs");

        self.last_block_number = latest_block.as_u64();

        info!("Writing cache");
        let cache = StateCache {
            last_block_number: self.last_block_number,
            borrowers: self.borrowers.clone(),
        };
        File::create(self.config.state_cache_file.clone())?.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());

        let mut res = Vec::new();
        for (i, start_block) in (from_block.as_u64()..to_block.as_u64())
            .step_by(LOG_BLOCK_RANGE as usize)
            .enumerate()
        {
            if i % 100 == 0 {
                info!("Getting borrow logs from block {} to {}", start_block, start_block + LOG_BLOCK_RANGE - 1);
            }
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.pool_address))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.supply_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.pool_address))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }


    async fn approve_tokens(&mut self) -> Result<()> {
        let liquidator = Liquidator::new(self.liquidator, self.write_client.clone());

        let mut nonce = self
            .write_client
            .get_transaction_count(
                self.write_client
                    .default_sender()
                    .ok_or(anyhow!("No connected sender"))?,
                None,
            )
            .await?;
        for token_address in self.tokens.keys() {
            let token = IERC20::new(token_address.clone(), self.write_client.clone());
            let allowance = token
                .allowance(self.liquidator, self.config.pool_address)
                .call()
                .await?;
            info!("approve token: {:?}", token_address);
            info!("allowance: {:?}", allowance);
            if allowance == U256::zero() {
                // TODO remove unwrap once we figure out whats broken
                liquidator
                    .approve_pool(*token_address)
                    .nonce(nonce)
                    .send()
                    .await
                    .map_err(|e| {
                        error!("approve failed: {:?}", e);
                        e
                    })?;
                nonce = nonce + 1;
            }
        }

        Ok(())
    }

    async fn update_token_configs(&mut self) -> Result<()> {
        info!("Updating token configs");
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.archive_client.clone());
        info!("pool_data: {:?}", pool_data);
        let all_tokens = pool_data.get_all_reserves_tokens().await?;
        let all_a_tokens = pool_data.get_all_a_tokens().await?;
        info!("all_tokens: {:?}", all_tokens);
        for (token, a_token) in zip(all_tokens, all_a_tokens) {
            let (decimals, ltv, threshold, bonus, reserve, _, _, _, _, _) = pool_data
                .get_reserve_configuration_data(token.token_address)
                .await?;
            let protocol_fee = pool_data
                .get_liquidation_protocol_fee(token.token_address)
                .await?;
            self.tokens.insert(
                token.token_address,
                TokenConfig {
                    address: token.token_address,
                    a_address: a_token.token_address,
                    decimals: decimals.low_u64(),
                    ltv: ltv.low_u64(),
                    liquidation_threshold: threshold.low_u64(),
                    liquidation_bonus: bonus.low_u64(),
                    reserve_factor: reserve.low_u64(),
                    protocol_fee: protocol_fee.low_u64(),
                },
            );
        }

        Ok(())
    }

    async fn get_best_liquidation_op(&mut self) -> Result<Option<LiquidationOpportunity>> {
        let underwater = self.get_underwater_borrowers().await?;

        if underwater.len() == 0 {
            return Err(anyhow!("No underwater borrowers found"));
        }

        info!("Found {} underwater borrowers", underwater.len());
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.write_client.clone());

        info!("pool_state init");

        let mut best_bonus: I256 = I256::MIN;
        let mut best_op: Option<LiquidationOpportunity> = None;
        let pool_state = self.get_pool_state().await?;

        info!("pool_state check");

        for (borrower, health_factor) in underwater {
            let borrower_details = self
                .borrowers
                .get(&borrower)
                .ok_or(anyhow!("Borrower not found"))?;

            for collateral_address in &borrower_details.collateral {
                for debt_address in &borrower_details.debt {
                    // TODO: handle case where collateral and debt are same asset
                    if collateral_address.ne(debt_address) {
                        info!("borrower: {:?}, collateral: {:?}, debt: {:?}", borrower, collateral_address, debt_address);
                        if let Some(op) = self
                            .get_liquidation_opportunity(
                                &borrower,
                                collateral_address,
                                debt_address,
                                &pool_data,
                                &health_factor,
                                &pool_state,
                            )
                            .await
                            .map_err(|e| info!("Liquidation op failed {}", e))
                            .ok()
                        {
                            if op.profit_usd > best_bonus {
                                best_bonus = op.profit_usd;
                                best_op = Some(op);
                            }
                        }
                    }
                }
            }
        }

        Ok(best_op)
    }

    fn get_liq_path(&self, collateral: &Address, debt: &Address) -> Result<(Bytes, String)> {
        // Try to use the configured path first
        if let Ok(liq_config) = LiqPathConfig::load_from_file(&self.config.liq_paths_config_file) {
            if let Some(path) = liq_config.build_liq_path(collateral, debt) {
                return Ok(path);
            }
        }

        info!("no liq config found for {:?} and {:?}", collateral, debt);

        // Fall back to default logic if no config file or no path found
        let usdxl_address = self.config.usdxl_address;
        let whype_address = self.config.whype_address;

        let mut path: Vec<Token> = Vec::new();
        
        // Determine the venue and if it's an exact output swap
        let liq_path = if debt.eq(&usdxl_address) {
            "usdxlFlashMinter".to_string()
        } else {
            self.config.default_liq_path.clone()
        };

        let is_kittenswap = liq_path == "kittenswap";
        let exact_out = liq_path == "kittenswap" || liq_path == "hyperswap";

        let (start_token, end_token) = if exact_out {
            (debt, collateral)
        } else {
            (collateral, debt)
        };

        // Build path
        path.push(Token::Address(*start_token));

        // If neither token is WETH, route through it
        if collateral.ne(&whype_address) && debt.ne(&whype_address) {
            if !is_kittenswap {
                path.push(Token::FixedBytes(3000u32.to_be_bytes()[1..].to_vec()));
            }
            path.push(Token::Bool(Self::is_stable(
                &self.pool_state,
                if exact_out { &whype_address } else { start_token },
                if exact_out { start_token } else { &whype_address }
            )?));
            path.push(Token::Address(whype_address));
        }

        // Add final token
        if !is_kittenswap {
            path.push(Token::FixedBytes(3000u32.to_be_bytes()[1..].to_vec()));
        }
        path.push(Token::Bool(Self::is_stable(
            &self.pool_state,
            if path.len() > 1 { &whype_address } else { start_token },
            end_token
        )?));
        path.push(Token::Address(*end_token));

        debug!("get_liq_path {:?}", path);

        let encoded_swap_path = encode_packed(&path)?;
        Ok((Bytes::from(encoded_swap_path), liq_path))
    }

    async fn get_pool_state(&self) -> Result<PoolState> {
        info!("multicall start");
        let mut multicall = Multicall::<M>::new(
            self.write_client.clone(),
            Some(self.config.multicall3_address.into()),
        )
        .await?;
        info!("multicall init");
        let mut prices = HashMap::new();
        let price_oracle = IAaveOracle::<M>::new(self.config.oracle_address, self.write_client.clone());
        info!("price_oracle init");
        for token_address in self.tokens.keys() {
            multicall.add_call(price_oracle.get_asset_price(*token_address), false);
        }

        let result: Vec<U256> = multicall.call_array().await?;
        for (token_address, price) in zip(self.tokens.keys(), result) {
            prices.insert(*token_address, price);
        }
        multicall.clear_calls();

        Ok(PoolState { prices })
    }

    async fn get_liquidation_opportunity(
        &self,
        borrower_address: &Address,
        collateral_address: &Address,
        debt_address: &Address,
        pool_data: &IPoolDataProvider<M>,
        health_factor: &U256,
        pool_state: &PoolState,
    ) -> Result<LiquidationOpportunity> {
        info!("getting liquidation opportunity for {:?}", borrower_address);
        let collateral_asset_price = pool_state
            .prices
            .get(collateral_address)
            .ok_or(anyhow!("No collateral price"))?;
        let debt_asset_price = pool_state
            .prices
            .get(debt_address)
            .ok_or(anyhow!("No debt price"))?;
        let collateral_config = self
            .tokens
            .get(collateral_address)
            .ok_or(anyhow!("Failed to get collateral address"))?;
        let debt_config = self
            .tokens
            .get(debt_address)
            .ok_or(anyhow!("Failed to get debt address"))?;
        let collateral_unit = U256::from(10).pow(collateral_config.decimals.into());
        let debt_unit = U256::from(10).pow(debt_config.decimals.into());
        let liquidation_bonus = collateral_config.liquidation_bonus;
        let a_token = IERC20::new(collateral_config.a_address.clone(), self.write_client.clone());

        let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
            .get_user_reserve_data(*debt_address, *borrower_address)
            .await?;
        let close_factor = if health_factor.gt(&U256::from(LIQUIDATION_CLOSE_FACTOR_THRESHOLD)) {
            U256::from(DEFAULT_LIQUIDATION_CLOSE_FACTOR)
        } else {
            U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)
        };

        let mut debt_to_cover =
            (stable_debt + variable_debt) * close_factor / MAX_LIQUIDATION_CLOSE_FACTOR;
        let base_collateral = (debt_asset_price * debt_to_cover * debt_unit)
            / (collateral_asset_price * collateral_unit);
        let mut collateral_to_liquidate = percent_mul(base_collateral, liquidation_bonus);
        let user_collateral_balance = a_token.balance_of(*borrower_address).await?;

        if collateral_to_liquidate > user_collateral_balance {
            collateral_to_liquidate = user_collateral_balance;
            debt_to_cover = (collateral_asset_price * collateral_to_liquidate * debt_unit)
                / percent_div(debt_asset_price * collateral_unit, liquidation_bonus);
        }

        let mut op = LiquidationOpportunity {
            borrower: borrower_address.clone(),
            collateral: collateral_address.clone(),
            collateral_to_liquidate,
            debt: debt_address.clone(),
            debt_to_cover,
            profit_usd: I256::from(0),
        };

        let (final_token, gain) = self.build_liquidation_call(&op).await?.call().await?;

        let final_token_price = pool_state
            .prices
            .get(&final_token)
            .ok_or(anyhow!("No price found for final token"))?;

        op.profit_usd = gain * I256::from_dec_str(&final_token_price.to_string())? / I256::from(PRICE_ONE);

        info!(
            "Found opportunity - borrower: {:?}, collateral: {:?}, debt: {:?}, profit_usd: {:?}",
            borrower_address, collateral_address, debt_address, op.profit_usd
        );

        Ok(op)
    }

    async fn build_liquidation_call(
        &self,
        op: &LiquidationOpportunity,
    ) -> Result<ContractCall<M, (Address, I256)>> {
        info!(
            "Build - borrower: {:?}, collateral: {:?}, debt: {:?}, debt_to_cover: {:?}, profit_usd: {:?}",
            op.borrower, op.collateral, op.debt, op.debt_to_cover, op.profit_usd
        );

        let liquidator = Liquidator::new(self.liquidator, self.write_client.clone());

        let (swap_path, liq_path) = self.get_liq_path(&op.collateral, &op.debt)?;

        info!("swap path: {:?}", swap_path);
        info!("liq path: {:?}", liq_path);

        info!("collateral to liquidate: {:?}", op.collateral_to_liquidate);

        info!("liquidator contract args:
COLLATERAL=0x{}
DEBT=0x{}
BORROWER=0x{}
DEBT_TO_COVER={}
SWAP_PATH=0x{}
LIQ_PATH={}",
            hex::encode(op.collateral.clone().as_ref()),
            hex::encode(op.debt.clone().as_ref()),
            hex::encode(op.borrower.clone().as_ref()),
            op.debt_to_cover,
            hex::encode(swap_path.clone().as_ref()),
            liq_path);

        let contract_call = liquidator.liquidate(
            op.collateral,
            op.debt,
            op.borrower,
            op.debt_to_cover,
            Bytes::from(swap_path),
            liq_path,
        );

        debug!("Liquidation op contract call: {:?}", contract_call);

        Ok(contract_call)
    }

    async fn build_liquidation(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let mut call = self.build_liquidation_call(op).await?;
        Ok(call.tx.set_chain_id(self.chain_id).clone())
        // let _call = self.build_liquidation_call(op).await?;
        // Ok(TypedTransaction::Eip1559(Eip1559TransactionRequest::new()
        //     .chain_id(self.chain_id).clone()))
    }

    /// Determines if a trading pair should be considered stable based on pool state prices
    /// Returns true if the prices are within 10% of each other
    fn is_stable(pool_state: &PoolState, token_a: &Address, token_b: &Address) -> Result<bool> {
        // Get prices from pool state
        let price_a = pool_state
            .prices
            .get(token_a)
            .ok_or_else(|| anyhow!("No price found for token {}", token_a))?;
        
        let price_b = pool_state
            .prices
            .get(token_b)
            .ok_or_else(|| anyhow!("No price found for token {}", token_b))?;

        // Ensure we don't divide by zero
        if price_b.is_zero() {
            return Ok(false);
        }

        // Convert U256 to f64 via u128
        let ratio = (price_a.as_u128() as f64) / (price_b.as_u128() as f64);

        debug!(
            "Stability check for {:?} and {:?}: price_a={}, price_b={}, ratio={}",
            token_a, token_b, price_a, price_b, ratio
        );

        // Check if prices are within 10% of each other
        // ratio should be between 0.9 and 1.1
        Ok(ratio >= 0.9 && ratio <= 1.1)
    }
}

fn percent_mul(a: U256, bps: u64) -> U256 {
    (U256::from(5000) + (a * bps)) / U256::from(10000)
}

fn percent_div(a: U256, bps: u64) -> U256 {
    let half_bps = bps / 2;
    (U256::from(half_bps) + (a * 10000)) / bps
}
