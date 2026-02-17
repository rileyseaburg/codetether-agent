# v3.2.0

## What's New

- **OpenAI-Compatible API**: Added a comprehensive OpenAI-compatible REST API to the server, enabling seamless integration with tools and clients that expect OpenAI endpoints
- **OAuth/JWT Provider Authentication**: Refactored all providers to support OAuth and JWT-based authentication, improving security and enabling cloud provider integrations
- **New Providers**: Added support for OpenAI Codex and Google Vertex GLM models
- **Token Display**: New TUI component for real-time token usage visualization during conversations

## Changes

- Refactored provider module architecture for better OAuth/JWT handling
- Enhanced worker server authentication flows
- Improved Ralph autonomous loop with better error handling
- Updated README with new API documentation and provider configuration examples
- Streamlined A2A worker implementation
