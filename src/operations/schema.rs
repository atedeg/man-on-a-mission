use super::bot_operations::help;
use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    types::Update,
};

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    Update::filter_message().endpoint(help)
    // let message_handler = Update::filter_message()
    // .branch(command_handler);

    // dialogue::enter::<Update, InMemStorage<()>, (), _>()
    //     .branch(command_handler)
}
