use anyhow::{anyhow, Result};
use ethers::types::Address;
use ethers::abi::{encode_packed, Token};
use ethers::types::Bytes;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use log::{error};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStep {
    pub swap_venue: String,
    pub pair: String,
    pub token_in: Address,
    pub token_out: Address,
    #[serde(default)]
    pub stable: bool,
    #[serde(default)]
    pub fee: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiqPath {
    pub liq_path: String,
    pub swap_path: Vec<SwapStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathConfig {
    pub collateral: Address,
    pub debt: Address,
    pub pair: String,
    pub liq_paths: Vec<LiqPath>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiqPathConfig(Vec<PathConfig>);

impl LiqPathConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        match File::open(path) {
            Ok(file) => {
                info!("opening liq path config file");
                let result = serde_json::from_reader(file);
                match result {
                    Ok(config) => {
                        info!("read liq path config from file");
                        Ok(config)
                    },
                    Err(e) => {
                        error!("failed to parse liq path config: {}", e);
                        Err(e.into())
                    }
                }
            }
            Err(_) => {
                info!("no liq path config file found");
                Err(anyhow!("no liq path config file found"))
            }
        }
    }

    pub fn find_path(&self, token_in: &Address, token_out: &Address, venue: &str) -> Option<&LiqPath> {
        // Find the path config that matches these tokens
        let path_config = self.0.iter().find(|config| {
            (config.collateral == *token_in && config.debt == *token_out) ||
            (config.collateral == *token_out && config.debt == *token_in)
        })?;

        // Find and return the entire liq path with the given venue
        path_config.liq_paths.iter()
            .find(|p| p.liq_path == venue)
    }

    pub fn build_liq_path(&self, collateral: &Address, debt: &Address) -> Option<(Bytes, String)> {
        info!("building liq path for {:?} and {:?}", collateral, debt);
        
        // Find the path config that matches these tokens
        let path_config = self.0.iter().find(|config| {
            (config.collateral == *collateral && config.debt == *debt) ||
            (config.collateral == *debt && config.debt == *collateral)
        })?;

        // Use the highest priority path (first in the list)
        let path = path_config.liq_paths.first()?;

        info!("using path: {:?}", path);

        // Build the encoded path from the swap steps
        let mut encoded_path: Vec<Token> = Vec::new();
        
        // Need to reverse the path for exact output venues (kittenswap/hyperswap)
        let is_exact_out = path.liq_path == "kittenswap" || path.liq_path == "hyperswap";
        
        let tokens_reversed = path_config.collateral != *collateral;
        let should_reverse = is_exact_out != tokens_reversed; // XOR - reverse if exactly one is true

        let mut steps = path.swap_path.iter().collect::<Vec<_>>();
        if should_reverse {
            steps.reverse();
        }

        for (index, step) in steps.iter().enumerate() {
            // Also need to swap token_in/token_out if path is reversed
            let (token_in, token_out) = if should_reverse {
                (step.token_out, step.token_in)
            } else {
                (step.token_in, step.token_out)
            };

            // Add token_in only for the first step
            if index == 0 {
                encoded_path.push(Token::Address(token_in));
            }
            
            if step.swap_venue == "kittenswap" {
                encoded_path.push(Token::Bool(step.stable));
            } else if step.swap_venue == "hyperswap" {
                 // fee is a uint24 which is 3 bytes
                encoded_path.push(Token::FixedBytes(step.fee.to_be_bytes()[1..4].to_vec()));
            }
            
            encoded_path.push(Token::Address(token_out));
        }

        let encoded_swap_path = encode_packed(&encoded_path).ok()?;
        Some((Bytes::from(encoded_swap_path), path.liq_path.clone()))
    }
} 