import re

with open("src/provider/stepfun.rs", "r") as f:
    code = f.read()

before = """                    if chunks.is_empty() {
                        StreamChunk::Text(String::new())
                    } else if chunks.len() == 1 {
                        chunks.pop().unwrap()
                    } else {
                        // Return first chunk, others are lost (simplified)
                        chunks.remove(0)
                    }
                }"""

after = """                    let mut process = || -> anyhow::Result<StreamChunk> {
                        if chunks.is_empty() {
                            Ok(StreamChunk::Text(String::new()))
                        } else if chunks.len() == 1 {
                            Ok(chunks.pop().ok_or_else(|| anyhow::anyhow!("no chunks"))?)
                        } else {
                            // Return first chunk, others are lost (simplified)
                            Ok(chunks.remove(0))
                        }
                    };
                    
                    match process() {
                        Ok(c) => c,
                        Err(e) => StreamChunk::Error(e.to_string())
                    }
                }"""

code = code.replace(before, after)

with open("src/provider/stepfun.rs", "w") as f:
    f.write(code)

