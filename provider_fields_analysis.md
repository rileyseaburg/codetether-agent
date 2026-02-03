# Provider Struct Fields Analysis

## Summary of Provider-Related Structs

### 1. `ProviderConfig` (src/config/mod.rs:52)
```rust
pub struct ProviderConfig {
    pub api_key: Option<String>,           // USED - env/config loading, provider initialization
    pub base_url: Option<String>,          // USED - OpenAI-compatible providers
    pub headers: HashMap<String, String>,  // UNUSED - defined but never accessed
    pub organization: Option<String>,      // UNUSED - defined but never accessed
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `api_key` | ✅ USED | Loaded from env vars (OPENAI_API_KEY, ANTHROPIC_API_KEY, GOOGLE_API_KEY), passed to provider constructors |
| `base_url` | ✅ USED | Used for OpenAI-compatible providers (Moonshot, OpenRouter, custom endpoints) |
| `headers` | ❌ UNUSED | Intended for custom HTTP headers in requests - NOT IMPLEMENTED |
| `organization` | ❌ UNUSED | Intended for OpenAI organization ID - NOT IMPLEMENTED |

---

### 2. `ProviderSecrets` (src/secrets/mod.rs:198)
```rust
pub struct ProviderSecrets {
    pub api_key: Option<String>,                    // USED - Vault secrets retrieval
    pub base_url: Option<String>,                   // USED - OpenAI-compatible providers
    pub organization: Option<String>,               // UNUSED - Vault-loaded, never used
    pub headers: Option<HashMap<String, String>>,   // UNUSED - Vault-loaded, never used
    pub extra: HashMap<String, serde_json::Value>,  // UNUSED - catch-all, never accessed
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `api_key` | ✅ USED | Retrieved from Vault, used to initialize providers |
| `base_url` | ✅ USED | Used for OpenAI-compatible providers via Vault |
| `organization` | ❌ UNUSED | Loaded from Vault but never passed to providers |
| `headers` | ❌ UNUSED | Loaded from Vault but never used in HTTP requests |
| `extra` | ❌ UNUSED | #[serde(flatten)] catch-all for future fields, never accessed |

---

### 3. `ProviderInfo` (src/provider/models.rs:76)
```rust
pub struct ProviderInfo {
    pub id: String,                                 // USED - provider lookup
    pub env: Vec<String>,                           // UNUSED - never accessed
    pub npm: Option<String>,                        // UNUSED - never accessed
    pub api: Option<String>,                        // USED - base URL for API calls
    pub name: String,                               // USED - display/UI
    pub doc: Option<String>,                        // UNUSED - never accessed
    pub models: HashMap<String, ApiModelInfo>,      // USED - model catalog
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `id` | ✅ USED | Provider identification, lookup key |
| `env` | ❌ UNUSED | Intended for environment variable hints - NOT USED |
| `npm` | ❌ UNUSED | Intended for npm package info - NOT USED |
| `api` | ✅ USED | API base URL for provider initialization |
| `name` | ✅ USED | Display name for UI/logging |
| `doc` | ❌ UNUSED | Documentation URL - NOT USED |
| `models` | ✅ USED | Model catalog lookup |

---

### 4. `ProviderRegistry` (src/provider/mod.rs:139)
```rust
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,  // USED - all methods
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `providers` | ✅ USED | All registry operations (register, get, list, from_config, from_vault) |

---

### 5. Provider Implementation Structs

#### `OpenAIProvider` (src/provider/openai.rs:22)
```rust
pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,   // USED - API calls
    provider_name: String,          // USED - name() method
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `client` | ✅ USED | All API calls (complete, complete_stream) |
| `provider_name` | ✅ USED | Provider trait name() implementation |

#### `AnthropicProvider` (src/provider/anthropic.rs:9)
```rust
pub struct AnthropicProvider {
    api_key: String,  // ❌ UNUSED - stored but never used (stub implementation)
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `api_key` | ❌ UNUSED | Stored in constructor but implementation is stub - NOT USED |

#### `GoogleProvider` (src/provider/google.rs:9)
```rust
pub struct GoogleProvider {
    api_key: String,  // ❌ UNUSED - stored but never used (stub implementation)
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `api_key` | ❌ UNUSED | Stored in constructor but implementation is stub - NOT USED |

#### `MoonshotProvider` (src/provider/moonshot.rs:15)
```rust
pub struct MoonshotProvider {
    client: Client,         // USED - HTTP requests
    api_key: String,        // USED - Authorization header
    base_url: String,       // USED - API endpoint
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `client` | ✅ USED | HTTP requests |
| `api_key` | ✅ USED | Bearer token in Authorization header |
| `base_url` | ✅ USED | API endpoint construction |

#### `OpenRouterProvider` (src/provider/openrouter.rs:16)
```rust
pub struct OpenRouterProvider {
    client: Client,         // USED - HTTP requests
    api_key: String,        // USED - Authorization header
    base_url: String,       // USED - API endpoint
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `client` | ✅ USED | HTTP requests |
| `api_key` | ✅ USED | Bearer token in Authorization header |
| `base_url` | ✅ USED | API endpoint construction |

#### `StepFunProvider` (src/provider/stepfun.rs:16)
```rust
pub struct StepFunProvider {
    api_key: String,        // USED - Authorization header
    client: reqwest::Client, // USED - HTTP requests
}
```

**Field Usage Mapping:**
| Field | Status | Intended Usage |
|-------|--------|----------------|
| `api_key` | ✅ USED | Bearer token in Authorization header |
| `client` | ✅ USED | HTTP requests |

---

## Unused Fields Catalog

### Critical Unused Fields (should be implemented or removed):

| Struct | Field | Current Status | Recommendation |
|--------|-------|----------------|----------------|
| `ProviderConfig` | `headers` | Never accessed | Implement in HTTP providers OR remove |
| `ProviderConfig` | `organization` | Never accessed | Pass to OpenAI provider OR remove |
| `ProviderSecrets` | `organization` | Loaded from Vault, never used | Pass to OpenAI provider OR remove |
| `ProviderSecrets` | `headers` | Loaded from Vault, never used | Implement custom header support OR remove |
| `ProviderSecrets` | `extra` | Never accessed | Document as reserved for future use |
| `ProviderInfo` | `env` | Never accessed | Document as metadata OR remove |
| `ProviderInfo` | `npm` | Never accessed | Document as metadata OR remove |
| `ProviderInfo` | `doc` | Never accessed | Add to provider info display |
| `AnthropicProvider` | `api_key` | Stub implementation | Complete implementation |
| `GoogleProvider` | `api_key` | Stub implementation | Complete implementation |

---

## Recommended Usage Patterns

### For `api_key` fields:
- **Debug impl**: Use `#[derive(Debug)]` with `debug_struct().field("api_key", &"***")` to redact
- **Display impl**: Never display directly
- **Logging**: Always redact in logs (use `tracing::debug!(api_key = "***", ...)`)
- **Validation**: Check non-empty before use
- **Serialization**: Use `#[serde(skip_serializing)]` or custom serializer

### For `id` fields:
- **Debug impl**: Standard derive
- **Display impl**: Use as primary identifier
- **Logging**: Safe to log fully
- **Validation**: Ensure uniqueness in registries
- **Serialization**: Standard serialization

### For `model` fields:
- **Debug impl**: Standard derive
- **Display impl**: Include in provider/model format
- **Logging**: Safe to log fully
- **Validation**: Validate against provider's available models
- **Serialization**: Standard serialization

### For `role` fields (in Message):
- **Debug impl**: Standard derive
- **Display impl**: Use lowercase string representation
- **Logging**: Safe to log fully
- **Validation**: Validate against enum variants
- **Serialization**: Use `#[serde(rename_all = "lowercase")]`

### For `base_url` fields:
- **Debug impl**: Standard derive
- **Display impl**: Display full URL
- **Logging**: Safe to log (may contain sensitive path info)
- **Validation**: Validate as valid URL
- **Serialization**: Standard serialization

### For `organization` fields (currently unused):
- **Debug impl**: Redact or omit
- **Display impl**: Omit
- **Logging**: Redact
- **Validation**: Validate format if OpenAI-specific
- **Serialization**: Standard serialization

### For `headers` fields (currently unused):
- **Debug impl**: Redact values (may contain secrets)
- **Display impl**: Omit values
- **Logging**: Redact values
- **Validation**: Validate header names are valid HTTP headers
- **Serialization**: Standard serialization

---

## Implementation Priority

1. **HIGH**: Complete `AnthropicProvider` and `GoogleProvider` implementations to use `api_key`
2. **MEDIUM**: Implement `organization` support in `OpenAIProvider` (pass to OpenAIConfig)
3. **MEDIUM**: Implement `headers` support in HTTP-based providers
4. **LOW**: Add `doc` field to provider info display
5. **LOW**: Document or remove unused metadata fields (`env`, `npm`, `extra`)
