use tera::Context;
use crate::app::middlewares::auth::AuthState;

pub async fn prepare_tera_context(mut current_user: AuthState) -> Context {
    let mut context = Context::new();
    if let Some(user) = current_user.get_user().await {
        context.insert("username", &user);
    }
    context
}
