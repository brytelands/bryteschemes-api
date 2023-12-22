use lazy_static::lazy_static;
use warp::{http::StatusCode, reply::json, Reply};

use bryte_descriptor_client::{get_account_data, get_account_schema, get_discriminator, get_discriminator_offline};
use crate::config::Config;

use crate::Result;

lazy_static! {
    static ref CONFIG: Config = Config::build().unwrap();
}

#[utoipa::path(
get,
path = "/account/{account_key}/{program_id}/{rpc_env}",
tag = "",
responses(
    (status = 200, description = "Account data retrieval successful"),
    (status = 404, description = "Account not found."),
),
params(
    ("account_key" = String, Path, description = "The public key of the account for which you want to retrieve data"),
    ("program_id" = String, Path, description = "The program ID to which the account was initialized."),
    ("rpc_env" = String, Path, description = "The Solana environment where you want to retrieve the data. Available values are: Dev, Test, Main, Local")
)
)]
pub async fn account_handler(account_key: String, program_id: String, rpc_env: String) -> Result<impl Reply> {
    let rpc_url = CONFIG.rpc_url(&rpc_env).unwrap();
    let result = get_account_data(account_key, program_id, rpc_url.to_string()).await.unwrap();
    println!("result {:?}", result);
    Ok(json(&result))
}

#[utoipa::path(
get,
path = "/account-schema/{account_key}/{program_id}/{rpc_env}",
tag = "",
responses(
(status = 200, description = "Account schema retrieval successful"),
(status = 404, description = "Account schema not found."),
),
params(
("account_key" = String, Path, description = "The public key of the account for which you want to retrieve the schema"),
("program_id" = String, Path, description = "The program ID to which the account was initialized."),
("rpc_env" = String, Path, description = "The Solana environment where you want to retrieve the schema. Available values are: Dev, Test, Main, Local")
)
)]
pub async fn account_schema_handler(account_key: String, program_id: String, rpc_env: String) -> Result<impl Reply> {
    let rpc_url = CONFIG.rpc_url(&rpc_env).unwrap();
    //TODO pass the start index of the data instead of bool
    let result = get_account_schema(account_key, program_id, true, rpc_url.to_string()).await.unwrap();
    println!("result {:?}", &result);
    Ok(result)
}

#[utoipa::path(
get,
path = "/discriminator-offline/{account_name}/{account_type}",
responses(
(status = 200, description = "Discriminator data retrieval successful"),
(status = 404, description = "Discriminator not found."),
),
params(
("account_name" = String, Path, description = "The name of the account struct for which you want to get the discriminator. This will not call the RPC, only the BryteSchemes API."),
("account_type" = String, Path, description = "The type of data for the struct for which you want to retrieve the discriminator. Most common values are account or event (Anchor and BryteDescriptor use these values as prefixes when generating the disciminator)")
)
)]
pub async fn discriminator_offline(account_name: String, account_type: String) -> Result<impl Reply> {
    let disc = get_discriminator_offline(account_name, account_type).await.unwrap();
    Ok(disc)
}

#[utoipa::path(
get,
path = "/discriminator/{account_key}/{rpc_env}",
responses(
(status = 200, description = "Discriminator retrieval successful"),
(status = 404, description = "Discriminator not found."),
),
params(
("account_key" = String, Path, description = "The public key of the account for which you want to retrieve data"),
("rpc_env" = String, Path, description = "The Solana environment where you want to retrieve the data. Available values are: Dev, Test, Main, Local")
)
)]
pub async fn discriminator(account_key: String, rpc_env: String) -> Result<impl Reply> {
    let rpc_url = CONFIG.rpc_url(&rpc_env).unwrap();
    let disc = get_discriminator(account_key, rpc_url.to_string()).await.unwrap();

    Ok(disc)
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
