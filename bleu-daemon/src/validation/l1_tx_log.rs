use serde_json::Value;

use crate::error::error::ExpectedError;
use crate::libs::opt::opt_to_result;
use crate::libs::serde::{get_type, get_u64};

pub fn verify(params: &Vec<Value>) -> Result<(), ExpectedError> {
    for param_val in params {
        if get_type(param_val) != "object" {
            return Err(ExpectedError::TypeError("param item must be object!".to_string()));
        } else {
            let param_obj = opt_to_result(param_val.as_object())?;
            let _ = get_u64(param_obj, "block_number")?;
            let _ = get_u64(param_obj, "queue_index")?;
        }
    }
    Ok(())
}
