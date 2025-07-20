use async_trait::async_trait;
use graph_flow::{
    Context, ExecutionStatus, FlowRunner, GraphBuilder, GraphStorage, InMemoryGraphStorage,
    InMemorySessionStorage, NextAction, Session, SessionStorage, Task, TaskResult,
};
use rig::{
    agent::Agent,
    client::{self, CompletionClient, ProviderClient},
    completion::CompletionModel,
};

const GOAL_CREATOR_PROMPT: &str = r#"
Analyze the user's input and generate clear, actionable goals.

# Requirements:
1. Goals **MUST** be specific, clear, and detailed to an actionable level.
2. The only actions you can take are:
 - Use the Internet to conduct research to achieve the goal.
 - Generate a report for the user.
3. **NEVER** take any action other than 2.

# User input: {query}
"#;

fn get_llm_agent(model: &str, preamble: &str) -> anyhow::Result<Agent<impl CompletionModel>> {
    let client = rig::providers::openai::Client::from_env();
    let agent = client.agent(model).preamble(preamble).build();
    Ok(agent)
}

struct PassiveGoalCreator<M>
where
    M: CompletionModel,
{
    provider: rig::agent::Agent<M>,
}

impl<M> PassiveGoalCreator<M>
where
    M: CompletionModel,
{
    fn new(model: String, preamble: String) -> PassiveGoalCreator {
        PassiveGoalCreator {
            provider: get_llm_agent(model, preamble).unwrap(),
        }
    }
}

#[async_trait]
impl Task for PassiveGoalCreator {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let name: String = context.get_sync("name").unwrap();
        let greeting = format!("Hello, {}", name);
        context.set("greeting", greeting.clone()).await;
        Ok(TaskResult::new(Some(greeting), NextAction::Continue))
    }
}

#[tokio::main]
async fn main() {
    println!("hello");
}
