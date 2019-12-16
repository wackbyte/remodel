mod remodel;

use rlua::Context;

pub use remodel::Remodel;

pub struct RemodelApi;

impl RemodelApi {
    pub fn inject(context: Context<'_>) -> rlua::Result<()> {
        context.globals().set("remodel", Remodel)?;

        Ok(())
    }
}
