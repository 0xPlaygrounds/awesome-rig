# Awesome Rig

A curated list of projects, libraries, examples, integrations, talks, and articles
for building LLM-powered Rust applications with [Rig](https://github.com/0xPlaygrounds/rig).

Rig is a modular Rust framework for agents, tool calling, completions,
embeddings, retrieval-augmented generation, vector stores, and multi-provider AI
applications.

## Contents

- [Official Resources](#official-resources)
- [Find More Projects](#find-more-projects)
- [Libraries and Frameworks](#libraries-and-frameworks)
- [Applications](#applications)
- [Production Users](#production-users)
- [Articles and Talks](#articles-and-talks)
- [Contributing](#contributing)
- [License](#license)

## Official Resources

- [Rig](https://github.com/0xPlaygrounds/rig) - The main Rig repository.
- [Rig website](https://rig.rs/) - Project website.
- [Rig docs](https://docs.rig.rs/) - Guides and documentation.
- [API docs](https://docs.rs/rig/latest/rig/) - Rust API reference for the root `rig` crate.
- [rig on crates.io](https://crates.io/crates/rig) - Feature-gated facade crate.
- [rig-core on crates.io](https://crates.io/crates/rig-core) - Core provider, agent, completion, embedding, and vector-store abstractions.
- [Rig ecosystem](https://github.com/0xPlaygrounds/rig/blob/main/ECOSYSTEM.md) - Upstream ecosystem showcase.
- [Rig Onchain Kit](https://github.com/0xPlaygrounds/rig-onchain-kit) - Helpers for connecting Rig agents with Solana and EVM workflows.

## Find More Projects

These sources are useful for finding additional projects that depend on Rig:

- [GitHub dependents for Rig](https://github.com/0xPlaygrounds/rig/network/dependents)
- [Reverse dependencies for `rig`](https://crates.io/crates/rig/reverse_dependencies)
- [Reverse dependencies for `rig-core`](https://crates.io/crates/rig-core/reverse_dependencies)
- [GitHub search for `rig-core`](https://github.com/search?q=%22rig-core%22+language%3ARust&type=code)
- [GitHub search for `rig::providers`](https://github.com/search?q=%22rig%3A%3Aproviders%22+language%3ARust&type=code)

## Libraries and Frameworks

- [Agent Governance Toolkit](https://github.com/microsoft/agent-governance-toolkit) - Microsoft's policy enforcement and governance toolkit for agents, with a dedicated Rust Rig integration for guarded Rig tools.
- [graph-flow](https://github.com/a-agmon/rs-graph-llm) - Type-safe graph execution framework for multi-agent workflows, similar in spirit to LangGraph.
- [coral-rs](https://github.com/Coral-Protocol/coral-rs) - Rig and RMCP helper library for building Rust-powered Coral agents.
- [rig-openapi-tools](https://github.com/skharchikov/rig-openapi-tools) - Generate Rig-callable tools from OpenAPI specifications.
- [llm-coding-tools](https://github.com/Sewer56/llm-coding-tools) - Lightweight Rig tool implementations for coding agents and developer workflows.
- [yart](https://github.com/pupplecat/yart) - Proc-macro utilities, including a `#[rig_tool]` macro for Rig tools.
- [riglr](https://github.com/riglr/riglr) - Rig-compatible tools for Solana, web search, DexScreener, Twitter, and related agent workflows.
- [rig-llama-cpp](https://github.com/camperking/rig-llama-cpp) - Rig completion provider for local GGUF models through llama.cpp, including streaming, tool calling, reasoning, and multimodal support.
- [rig-memvid](https://github.com/ForeverAngry/rig-memvid) - Memvid-backed persistent memory and lexical store for Rig agents.
- [rig-tap](https://github.com/ForeverAngry/rig-tap) - Backend-agnostic observability events and lifecycle taps for Rig agents.
- [rig-retrieval-evals](https://github.com/ForeverAngry/rig-retrieval-evals) - Evaluation harness for Rig retrieval and knowledge-base workflows.
- [dspy-rs](https://github.com/krypticmouse/DSRs) - Rust rewrite of DSPy that uses Rig in its LLM plumbing.
- [rigs](https://github.com/M4n5ter/rigs) - Orchestration framework built around Rig.
- [metalcraft](https://github.com/rust4ai/metalcraft) - LangGraph-style stateful graph orchestrator for AI agents in Rust.
- [reasonkit-core](https://github.com/reasonkit/reasonkit-core) - Rust-native reasoning engine for auditable production AI systems.
- [solagent](https://github.com/zTgx/solagent.rs) - Framework for connecting AI agents to Solana protocols in Rust.
- [skill](https://github.com/kubiyabot/skill) - Runtime, CLI, and MCP server for AI agent skills, with Rig-powered execution components.
- [unifai-sdk-rs](https://github.com/unifai-network/unifai-sdk-rs) - Rust SDK for dynamic tools and agent-to-agent communication on Unifai.
- [awpak-ai](https://github.com/afuentesan/awpak-tui/tree/main/awpak-ai) - Agent, command, and URL orchestration library using execution graphs.
- [nika](https://github.com/supernovae-st/nika) - Semantic YAML workflow engine for AI tasks with DAG execution, MCP, and multi-provider LLM support.
- [flow-like](https://github.com/Rheosoph/flow-like) - SDK for building Flow-Like WASM nodes and AI workflow components.
- [weavegraph](https://github.com/Idleness76/weavegraph) - Graph-driven concurrent agent workflow framework with versioned state and deterministic merges.
- [rig-dyn](https://github.com/GustavoWidman/rig-dyn) - Dynamic client-provider abstraction layer on top of `rig-core`.
- [rig-extra](https://github.com/launcher-rs/rig-extra-project) - Lightweight extensions built on top of `rig-core`.
- [rig-redis-vectorstore](https://github.com/daric93/rig-redis-vectorstore) - Redis (RediSearch) vector store integration for Rig with KNN similarity search, metadata filtering, and configurable distance metrics ([crates.io](https://crates.io/crates/rig-redis-vectorstore)).

## Applications

### Coding Agents and Developer Tools

- [VT Code](https://github.com/vinhnx/vtcode) - Rust terminal coding agent with semantic code intelligence, Tree-sitter, ast-grep, and Rig-powered model selection.
- [Dirge](https://github.com/dirge-code/dirge) - Minimal Rust coding agent optimized for memory footprint and performance.
- [Zerostack](https://github.com/gi-dellav/zerostack) - Minimal Rust coding agent powered by the root `rig` crate.
- [Metalcraft Agent](https://github.com/rust4ai/metalcraft-agent) - Interactive AI coding agent with personas, skills, and tool approval.
- [git-iris](https://github.com/hyperb1iss/git-iris) - AI Git assistant for commits, reviews, changelogs, and release notes.
- [rv](https://github.com/gi-dellav/rv) - Non-invasive AI code review tool for existing workflows.
- [nitpicker](https://github.com/arsenyinfo/nitpicker) - Multi-reviewer code review CLI using parallel Rig agents and debate mode.
- [probe](https://github.com/buger/probe) - Local semantic code search tool for large codebases.
- [committor](https://github.com/simonhdickson/committor) - Conventional commit message generator based on Git diffs.
- [gmsg](https://github.com/olorikendrick/gmsg) - AI-powered commit message generator with a TUI editor.
- [gitbuddy](https://github.com/1mw1zard/gitbuddy) - AI tool for simplifying Git commit workflows.
- [bgit](https://github.com/rootCircle/bgit) - Beginner-friendly Git wrapper with safeguards for common mistakes.

### Agents, Assistants, and RAG

- [ChatShell](https://github.com/chatshellapp/chatshell-desktop) - Open-source agentic desktop AI client built on `rig-core` and Tauri.
- [Con](https://github.com/nowledge-co/con-terminal) - GPU-accelerated terminal emulator with an integrated AI agent harness.
- [Ironclaw](https://github.com/nearai/ironclaw) - Secure personal AI assistant.
- [clipbud](https://github.com/evilsocket/clipbud) - Cross-platform AI clipboard assistant.
- [deepwiki-rs](https://github.com/sopaco/deepwiki-rs) - AI documentation engine that generates technical documentation from codebases.
- [Cortex Memory](https://github.com/sopaco/cortex-mem) - Memory system for intelligent agents, including extraction, vector search, optimization, MCP, REST, CLI, and dashboards.
- [STEVE](https://github.com/dev-ben-nisien/Steve) - Search Technical Evidence Very Easily.
- [Ghost AI](https://github.com/yazaldefilimone/ghost.ai) - Desktop second-brain assistant.
- [Amico](https://github.com/AIMOverse/amico) - AI agent framework with SDK and plugins.
- [termai](https://github.com/JazzyMcJazz/termai) - AI assistant for the terminal.
- [squid](https://github.com/DenysVuika/squid) - AI-powered command-line code review and suggestion tool.
- [taquba-research](https://github.com/micllam/taquba-research) - Reference implementation of a durable Rig agent: a research CLI that plans, searches the web, reads pages, and synthesizes cited reports, with multi-step runs persisted to object storage so they resume after a crash.

### Domain-Specific Projects

- [Listen](https://github.com/piotrostr/listen) - Framework for AI portfolio-management agents.
- [dkn-compute-node](https://github.com/firstbatchxyz/dkn-compute-node) - Compute node for Dria's decentralized AI network.
- [nine](https://github.com/NethermindEth/nine) - Nethermind's Neural Interconnected Nodes Engine.
- [appdotbuild-agent](https://github.com/neondatabase/appdotbuild-agent) - Rust agent powering Neon's app.build V2 reboot.
- [syncable-cli](https://github.com/syncable-dev/syncable-cli) - CLI that analyzes repositories and generates Infrastructure as Code.
- [tsql](https://github.com/fcoury/tsql) - Modern keyboard-first PostgreSQL CLI.
- [kumo](https://github.com/wihlarkop/kumo) - Async web crawling framework for Rust.
- [markitdown-rs](https://github.com/uhobnil/markitdown-rs) - Rust library for converting document formats into Markdown text.
- [noctisroll](https://github.com/noctisynth/noctisroll) - Modular TRPG dice rolling system.

## Production Users

These companies and teams publicly mention Rig usage in the upstream README or ecosystem docs:

- [St. Jude](https://www.stjude.org/) - Uses Rig in a chatbot utility for [proteinpaint](https://github.com/stjude/proteinpaint), a genomics visualization tool.
- [Coral Protocol](https://www.coralprotocol.org/) - Uses Rig internally and in [coral-rs](https://github.com/Coral-Protocol/coral-rs).
- [Dria](https://dria.co/) - Uses Rig in [dkn-compute-node](https://github.com/firstbatchxyz/dkn-compute-node).
- [Nethermind](https://www.nethermind.io/) - Uses Rig in [nine](https://github.com/NethermindEth/nine).
- [Neon](https://neon.com/) - Uses Rig in [appdotbuild-agent](https://github.com/neondatabase/appdotbuild-agent).
- [Cairnify](https://cairnify.com/) - Uses Rig for agentic AI search workflows.
- [Ryzome](https://ryzome.ai/) - Visual AI workspace for connected canvases of research, thoughts, and agents.
- [ilert](https://www.ilert.com/) - Uses Rig as a multi-provider abstraction in its agentic LLM proxy.
- [Syncable](https://syncable.dev/) - Uses Rig in its CLI.
- [Refresh Agent](https://refreshagent.com/) - Uses Rig for SEO and marketing-analysis agents.
- [nitpik](https://nitpik.dev/) - Uses Rig under the hood for AI code reviews.
- [Archestra](https://github.com/archestra-ai/archestra) - MCP-native secure AI platform. Uses Rig in its agentic benchmark.

## Articles and Talks

- [Rust and Rig: Building for Stability in a Rapidly Changing AI Landscape](https://www.youtube.com/watch?v=nLlY2nNgBgM) - Talk by Joshua Mo.
- [Is Rig better than LangChain?](https://www.youtube.com/watch?v=nLlY2nNgBgM) - Video about Rig and Rust versus Python.
- [Building the Rig AI Framework with Rust](https://www.youtube.com/watch?v=nLlY2nNgBgM) - Podcast episode from Coding Chats.
- [Rust Crate of the Year, IMO](https://www.youtube.com/watch?v=9L9oOmqD6zc) - Code to the Moon introduction to Rig.
- [Building AI Agents in Rust](https://refreshagent.com/engineering/building-ai-agents-in-rust) - Refresh Agent engineering article.
- [Rig guides](https://docs.rig.rs/guides) - Official Rig guides and blog-style walkthroughs.

## Contributing

Contributions are welcome. Good entries should be:

- Publicly accessible.
- Clearly related to Rig, `rig-core`, or a Rig companion crate.
- Useful to Rust developers building LLM, agent, tool-calling, embedding, or RAG applications.
- Described with a short factual sentence, not marketing copy.

Please keep categories alphabetized where practical and avoid adding abandoned forks,
empty demos, or projects with no clear Rig usage.

## License

This project is licensed under the [MIT License](LICENSE).
