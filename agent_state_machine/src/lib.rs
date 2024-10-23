//! Agent State Machine is a library for managing Large Language Model (LLM) agents
//! using a state machine pattern. It provides a robust way to handle agent states,
//! transitions, and behaviors.
//! 
//! # Example
//! ```rust,no_run
//! use agent_state_machine::{ChatAgentStateMachine, AgentState};
//! use rig::providers::openai;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let client = openai::Client::from_env();
//!     let agent = client
//!         .agent(openai::GPT_4)
//!         .preamble("You are a helpful AI assistant.")
//!         .build();
//!     
//!     let mut state_machine = ChatAgentStateMachine::new(agent);
//!     
//!     let response = state_machine.process_message("Hello!").await.unwrap();
//!     println!("Response: {}", response);
//! }
//! ```

mod state;
mod machine;

pub use state::AgentState;
pub use machine::ChatAgentStateMachine;