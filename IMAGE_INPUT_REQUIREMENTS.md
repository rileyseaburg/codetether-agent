# Image Input Support Requirements

Based on analysis of the opencode TypeScript implementation, this document outlines requirements for adding image/multimodal input support to CodeTether Agent.

## Overview

Opencode handles images through a sophisticated multimodal content system that supports:
- Base64-encoded inline images
- External image URLs
- File attachments with MIME type detection
- Provider-specific optimizations

## Current State (CodeTether)

The `ContentPart` enum in `src/provider/mod.rs` already has basic image support:

```rust
pub enum ContentPart {
    Text { text: String },
    Image {
        url: String,
        mime_type: Option<String>,
    },
    File {
        path: String,
        mime_type: Option<String>,
    },
    // ...
}
```

However, this is not fully implemented across the system.

---

## Requirements

### 1. Data Model Enhancements

#### 1.1 Enhanced Image ContentPart
Extend the `Image` variant to support both URL and base64 data:

```rust
pub enum ImageData {
    Url(String),           // External URL (https://...)
    Base64 {              // Inline base64-encoded image
        data: String,
        media_type: String, // e.g., "image/jpeg", "image/png"
    },
    FileId(String),       // Provider-specific file ID (OpenAI file API)
}

pub enum ContentPart {
    Text { text: String },
    Image {
        data: ImageData,
        detail: Option<ImageDetail>, // low, high, auto
        filename: Option<String>,
    },
    // ...
}

pub enum ImageDetail {
    Low,
    High,
    Auto,
}
```

#### 1.2 File Part Enhancement
Support generic file attachments (PDFs, etc.):

```rust
pub enum ContentPart {
    File {
        data: FileData,
        mime_type: String,
        filename: Option<String>,
    },
    // ...
}

pub enum FileData {
    Path(String),         // Local file path
    Url(String),          // External URL
    Base64(String),       // Base64-encoded content
    FileId(String),       // Provider file ID
}
```

### 2. File Input Handling

#### 2.1 MIME Type Detection
Implement automatic MIME type detection:

```rust
pub fn detect_mime_type(path: &Path) -> Option<String> {
    // Use file extension and magic bytes
    // Support: image/*, application/pdf, text/*
}

pub fn is_image_mime(mime: &str) -> bool {
    mime.starts_with("image/")
}

pub fn is_supported_vision_mime(mime: &str) -> bool {
    matches!(mime, 
        "image/jpeg" | "image/png" | "image/gif" | 
        "image/webp" | "image/bmp" | "application/pdf"
    )
}
```

#### 2.2 Binary File Detection
Implement binary file detection (from opencode's `shouldEncode`):

```rust
pub fn should_encode_as_binary(mime_type: &str) -> bool {
    let top_level = mime_type.split('/').next().unwrap_or("");
    let sub_type = mime_type.split('/').nth(1).unwrap_or("");
    
    // Binary top-level types
    if matches!(top_level, "image" | "audio" | "video" | "font" | "model" | "multipart") {
        return true;
    }
    
    // Binary subtypes
    let binary_markers = ["zip", "gzip", "pdf", "msword", "binary", "ogg"];
    binary_markers.iter().any(|m| sub_type.contains(m))
}
```

#### 2.3 Image Processing
- **Base64 encoding**: Convert binary images to base64 strings
- **Size limits**: Enforce maximum file sizes (provider-specific)
- **Format conversion**: Optionally convert to supported formats
- **Detail levels**: Support "low", "high", "auto" detail settings

### 3. CLI Interface

#### 3.1 Run Command Enhancement
Extend `RunArgs` to support image attachments:

```rust
#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Message to send
    pub message: String,
    
    /// Files to attach (images, PDFs, etc.)
    #[arg(short, long)]
    pub file: Vec<PathBuf>,
    
    /// Image detail level
    #[arg(long, value_parser = ["low", "high", "auto"])]
    pub image_detail: Option<String>,
    
    // ... existing fields
}
```

**Usage examples:**
```bash
# Single image
codetether run "Describe this image" --file screenshot.png

# Multiple images
codetether run "Compare these" --file image1.jpg --file image2.png

# With PDF
codetether run "Summarize this document" --file report.pdf

# With detail level
codetether run "Read this code" --file diagram.png --image-detail high
```

#### 3.2 TUI Integration
- Drag-and-drop support for images
- Paste image from clipboard
- Visual indicator for attached files
- Image preview in chat (if terminal supports it)

### 4. Provider Implementations

#### 4.1 OpenAI-Compatible (OpenAI, Copilot)

Convert `ContentPart::Image` to OpenAI's message format:

```rust
// For base64 images
{
    "type": "image_url",
    "image_url": {
        "url": "data:image/jpeg;base64,/9j/4AAQ...",
        "detail": "high"
    }
}

// For external URLs
{
    "type": "image_url", 
    "image_url": {
        "url": "https://example.com/image.jpg",
        "detail": "auto"
    }
}

// For PDFs (OpenAI Responses API)
{
    "type": "input_file",
    "file_url": "https://..."  // or file_id
}
```

Implementation in `src/provider/openai.rs`:

```rust
fn convert_content_part(part: &ContentPart) -> serde_json::Value {
    match part {
        ContentPart::Text { text } => json!({
            "type": "text",
            "text": text
        }),
        ContentPart::Image { data, detail, .. } => {
            let url = match data {
                ImageData::Url(u) => u.clone(),
                ImageData::Base64 { data, media_type } => {
                    format!("data:{};base64,{}, media_type, data)
                }
                ImageData::FileId(id) => {
                    // Use file_id field instead
                    return json!({
                        "type": "image_file",
                        "file_id": id
                    });
                }
            };
            
            let mut obj = json!({
                "type": "image_url",
                "image_url": { "url": url }
            });
            
            if let Some(d) = detail {
                obj["image_url"]["detail"] = json!(d.to_string().to_lowercase());
            }
            
            obj
        }
        // ...
    }
}
```

#### 4.2 Anthropic (Claude)

Anthropic uses a different format:

```rust
// Base64 image
{
    "type": "image",
    "source": {
        "type": "base64",
        "media_type": "image/jpeg",
        "data": "/9j/4AAQ..."
    }
}

// External URL (if supported)
{
    "type": "image",
    "source": {
        "type": "url",
        "url": "https://example.com/image.jpg"
    }
}
```

#### 4.3 Google (Gemini)

Gemini format:

```rust
// Base64 inline
{
    "inlineData": {
        "mimeType": "image/jpeg",
        "data": "/9j/4AAQ..."
    }
}

// File URI
{
    "fileData": {
        "mimeType": "image/jpeg",
        "fileUri": "gs://bucket/image.jpg"
    }
}
```

#### 4.4 Provider Capability Detection

Add vision capability to `ModelInfo`:

```rust
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub supports_vision: bool,
    pub vision_mime_types: Vec<String>, // Supported MIME types
    pub max_image_size: Option<usize>,  // Max bytes per image
    pub max_images_per_request: Option<usize>,
    // ...
}
```

### 5. Session Management

#### 5.1 Persisting Image Data
Images should be stored efficiently in sessions:

```rust
// Option 1: Store base64 in session JSON (simple, but large)
// Option 2: Store images in separate directory, reference by ID

pub struct Session {
    // ... existing fields
    pub attachments: Vec<Attachment>,
}

pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub path: PathBuf,  // Local storage path
    pub created_at: DateTime<Utc>,
}
```

#### 5.2 Session Storage Layout

```
~/.local/share/codetether/
├── sessions/
│   ├── session-uuid.json
│   └── attachments/
│       └── session-uuid/
│           ├── img-1.png
│           └── doc.pdf
```

### 6. TUI Enhancements

#### 6.1 File Attachment UI
- Show attached files in input area
- Support keyboard shortcuts (Ctrl+A to attach)
- Visual indicators for file types

#### 6.2 Image Display
- Show image filenames in chat
- Optional: terminal image preview (using sixel, iTerm2 inline images, or kitty graphics protocol)
- Fallback: show file info (size, dimensions, type)

#### 6.3 Input Handling
```rust
pub enum InputEvent {
    Text(String),
    AttachFile(PathBuf),
    PasteImage(Vec<u8>),  // From clipboard
}
```

### 7. Error Handling

#### 7.1 Validation Errors
- Unsupported file type
- File too large
- Invalid image format
- Provider doesn't support vision

#### 7.2 User Feedback
```
Error: Cannot attach "video.mp4" - unsupported file type
Supported: .jpg, .jpeg, .png, .gif, .webp, .pdf

Error: Image "huge.png" (25MB) exceeds maximum size (20MB)

Warning: Model "gpt-3.5-turbo" doesn't support images. 
Images will be ignored or use a vision-capable model.
```

### 8. Security Considerations

#### 8.1 Path Validation
- Validate file paths are within allowed directories
- Prevent directory traversal attacks
- Sandbox file access

#### 8.2 Content Validation
- Verify file magic bytes match extension
- Scan for malicious content
- Limit total attachment size per request

#### 8.3 URL Validation
- Whitelist allowed URL schemes (https only)
- Validate URL format
- Implement timeouts for fetching

### 9. Implementation Phases

#### Phase 1: Core Infrastructure
1. Enhance `ContentPart` enum with `ImageData` and `FileData`
2. Implement MIME type detection
3. Add base64 encoding utilities
4. Update serialization/deserialization

#### Phase 2: Provider Support
1. Implement OpenAI image format conversion
2. Implement Anthropic image format conversion
3. Implement Google image format conversion
4. Add vision capability detection

#### Phase 3: CLI Integration
1. Update `RunArgs` with file attachment options
2. Implement file reading and validation in `run.rs`
3. Add image detail parameter
4. Error handling and user feedback

#### Phase 4: Session Persistence
1. Design attachment storage system
2. Implement attachment upload/download
3. Update session serialization
4. Cleanup orphaned attachments

#### Phase 5: TUI Integration
1. Add file attachment UI
2. Implement clipboard paste
3. Visual indicators for attachments
4. Optional image preview

### 10. Testing Requirements

#### 10.1 Unit Tests
- MIME type detection
- Base64 encoding/decoding
- ContentPart serialization
- Provider format conversion

#### 10.2 Integration Tests
- End-to-end image upload flow
- Multiple provider support
- Session persistence
- Error scenarios

#### 10.3 Test Images
- Various formats: JPG, PNG, GIF, WebP, BMP
- Various sizes: small (<1KB), medium (<1MB), large (>10MB)
- Edge cases: corrupted, zero-byte, non-image with image extension

### 11. Documentation

#### 11.1 User Documentation
- How to attach images via CLI
- Supported file formats
- Provider-specific limitations
- Image detail levels explained

#### 11.2 Developer Documentation
- ContentPart enum design
- Adding new providers
- Extending MIME type support
- TUI integration guide

---

## Open Questions

1. **Storage**: Should we store base64 in session JSON or use external files?
2. **Caching**: Should we cache fetched external URLs?
3. **Compression**: Should we compress/resize large images automatically?
4. **Privacy**: How to handle sensitive images in logs/telemetry?
5. **Rate Limiting**: How to handle provider image upload limits?

## References

- Opencode implementation: `packages/opencode/src/session/message-v2.ts`
- Opencode file handling: `packages/opencode/src/file/index.ts`
- OpenAI Vision API: https://platform.openai.com/docs/guides/vision
- Anthropic Vision: https://docs.anthropic.com/claude/docs/vision
- Google Gemini Vision: https://ai.google.dev/gemini-api/docs/vision
