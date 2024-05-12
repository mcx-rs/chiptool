use regex::{self, Regex};
use serde::{Deserialize, Serialize};

use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteNumSuffix {}

impl DeleteNumSuffix {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut renames = vec![];
        for (name, _) in &ir.blocks {
            if name.ends_with("0") {
                if !ir.blocks.iter().any(|(n, _)| {
                    if name == n {
                        return false;
                    }
                    let re = Regex::new(&format!("^{}(\\d+)$", name.strip_suffix("0").unwrap()))
                        .unwrap();
                    re.is_match(&n)
                }) {
                    renames.push(name.clone());
                }
            }
        }
        for name in renames {
            // rename hashmap key
            let b = ir.blocks.remove(&name).unwrap();
            ir.blocks
                .insert(name.strip_suffix("0").unwrap().to_string(), b);
        }

        Ok(())
    }
}
