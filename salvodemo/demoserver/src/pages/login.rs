use serde::{Serialize};
use salvo::prelude::*;

#[derive(Serialize, Debug)]
struct LoginResult {
    code: i64,
    message: String,
}

#[async_trait]
impl Writer for LoginResult {
    async fn write(
        self,
        _req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response
    ) {
        res.render(Json(self));
    }
}

#[handler]
pub async fn login() -> Result<LoginResult,String> {
    Ok(LoginResult { code: 0, message: String::from("成功") })
}

#[handler]
pub async fn logout(req: &mut Request, res: &mut Response) -> Result<String, String> {
    let id = req.param::<i64>("id");
    match id {
        Some(lid) => Ok(format!("logout {}", lid)),
        None => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            Err(String::from("不是有效的ID"))
        },
    }
}