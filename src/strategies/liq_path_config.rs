use serde::{Deserialize, Serialize};
use ethers::types::Address;
use ethers::abi::{encode_packed, Token};
use ethers::types::Bytes;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiqPathConfig {
    #[serde(with = "token_pair_map_serde")]
    pub pairs: HashMap<(Address, Address), Vec<LiquidationPath>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPairConfig {
    collateral: Address,
    debt: Address,
    pair: String,
    liq_paths: Vec<LiquidationPath>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationPath {
    pub liq_path: String,
    pub swap_path: Vec<SwapPairConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapPairConfig {
    pub pair: String,
    pub token_in: Address,
    pub token_out: Address,
    #[serde(flatten)]
    pub venue_config: SwapVenueConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "swap_venue")]
pub enum SwapVenueConfig {
    #[serde(rename = "kittenswap")]
    Kittenswap { stable: bool },
    #[serde(rename = "hyperswap")]
    Hyperswap { fee: u32 },
}

// Custom serialization for the HashMap
mod token_pair_map_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(
        map: &HashMap<(Address, Address), Vec<LiquidationPath>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pairs: Vec<_> = map
            .iter()
            .map(|((collateral, debt), liq_paths)| TokenPairConfig {
                collateral: *collateral,
                debt: *debt,
                pair: format!("{}-{}", "TOKEN_A", "TOKEN_B"), // You might want to improve this
                liq_paths: liq_paths.to_vec(),
            })
            .collect();
        pairs.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(Address, Address), Vec<LiquidationPath>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs: Vec<TokenPairConfig> = Vec::deserialize(deserializer)?;
        Ok(pairs
            .into_iter()
            .map(|pair| ((pair.collateral, pair.debt), pair.liq_paths))
            .collect())
    }
}

impl LiqPathConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let config: LiqPathConfig = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn find_path(&self, token_in: &Address, token_out: &Address, venue: &str) -> Option<&SwapPairConfig> {
        // Sort addresses for lookup
        let (first, second) = if token_in < token_out {
            (token_in, token_out)
        } else {
            (token_out, token_in)
        };

        // Direct lookup in the HashMap
        let paths = self.pairs.get(&(*first, *second))?;

        // Find the path with the given venue name
        let path = paths.iter()
            .find(|p| p.liq_path == venue)?;

        // Return the first pair (assuming one pair per venue for now)
        path.swap_path.first()
    }

    pub fn build_liq_path(&self, collateral: &Address, debt: &Address) -> Option<(Bytes, String)> {
        // Sort addresses for lookup
        let (first, second) = if collateral < debt {
            (collateral, debt)
        } else {
            (debt, collateral)
        };

        // Get the paths for this token pair
        let paths = self.pairs.get(&(*first, *second))?;

        // Use the highest priority path (first in the list)
        let path = paths.first()?;

        // Build the encoded path from the swap pairs
        let mut encoded_path: Vec<Token> = Vec::new();
        
        // Need to reverse the path in two cases:
        // 1. For exact output venues (kittenswap/hyperswap)
        // 2. When the collateral/debt tokens are opposite of our configured case
        let is_exact_out = path.liq_path == "kittenswap" || path.liq_path == "hyperswap";
        
        let tokens_reversed = first != collateral; // true if we had to swap the order for lookup
        let should_reverse = is_exact_out != tokens_reversed; // XOR - reverse if exactly one is true

        let mut pairs = path.swap_path.iter().collect::<Vec<_>>();
        if should_reverse {
            pairs.reverse();
        }

        for pair in pairs {
            // Also need to swap token_in/token_out if path is reversed
            let (token_in, token_out) = if should_reverse {
                (pair.token_out, pair.token_in)
            } else {
                (pair.token_in, pair.token_out)
            };

            encoded_path.push(Token::Address(token_in));
            
            match &pair.venue_config {
                SwapVenueConfig::Kittenswap { stable } => {
                    encoded_path.push(Token::FixedBytes(vec![0; 3])); // Empty fee for kittenswap
                    encoded_path.push(Token::Bool(*stable));
                },
                SwapVenueConfig::Hyperswap { fee } => {
                    encoded_path.push(Token::FixedBytes(fee.to_be_bytes()[1..].to_vec()));
                    encoded_path.push(Token::Bool(false)); // No stable flag for hyperswap
                },
            }
            
            encoded_path.push(Token::Address(token_out));
        }

        let encoded_swap_path = encode_packed(&encoded_path).ok()?;
        Some((Bytes::from(encoded_swap_path), path.liq_path.clone()))
    }
} 