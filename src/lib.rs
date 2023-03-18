use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[derive(Deserialize, Serialize)]
pub struct Customer  {
		CustomerID: i32,
		CompanyName: String,
		ContactName: String
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

		let d1 = env.d1("DB")?;
		let statement = d1.prepare( "SELECT * FROM Customers WHERE CompanyName = ?");
		let query = statement.bind(&["Bs Beverages".into()])?;
		let result = query.first::<Customer>(None).await?;
		match result {
				Some(thing) => Response::from_json(&thing),
				None => Response::error("Not found", 404),
		}
}
