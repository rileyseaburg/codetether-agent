# Low-Latency TTS Models for RTX 2080

Research summary of the best text-to-speech models that can run locally on an NVIDIA RTX 2080 (8GB VRAM).

---

## 🏆 Top Recommendations

### 1. Kokoro-82M — Fastest Option ⚡

- **Latency**: Sub-0.3 seconds across all text lengths
- **Size**: Only 82M parameters (very lightweight)
- **VRAM**: ~500MB-1GB
- **Quality**: Good natural speech
- **Best for**: Real-time applications, conversational AI
- **Link**: [hexgrad/kokoro](https://github.com/hexgrad/kokoro)

### 2. MeloTTS — CPU-Friendly Alternative

- **Latency**: Real-time on CPU, even faster on GPU
- **Size**: Based on VITS/VITS2
- **Languages**: Multilingual (English, Spanish, Chinese, etc.)
- **VRAM**: Very low (~1-2GB)
- **Best for**: Low-resource environments
- **Link**: [myshell-ai/MeloTTS](https://github.com/myshell-ai/MeloTTS)

### 3. Piper TTS — Ultra-Fast, Runs on CPU

- **Latency**: ~50-100ms on modern CPU
- **Size**: ~20-60MB per voice
- **VRAM**: None (CPU-based)
- **Quality**: Decent, slightly robotic
- **Best for**: Home automation, voice assistants
- **Link**: [rhasspy/piper](https://github.com/rhasspy/piper)

### 4. F5-TTS — Quality + Speed Balance

- **Latency**: ~0.15 RTF (real-time factor)
- **Size**: Available in smaller variants
- **VRAM**: ~4-6GB on RTX 2080
- **Features**: Zero-shot voice cloning, multilingual
- **Best for**: When you need quality and speed
- **Link**: [SWivid/F5-TTS](https://github.com/SWivid/F5-TTS)

### 5. Qwen3-TTS-0.6B — New & Powerful

- **Latency**: As low as **97ms** streaming (first audio packet)
- **Size**: 0.6B or 1.7B variants
- **VRAM**: 0.6B fits on RTX 2080 (~4-5GB)
- **Features**: 
  - Voice cloning (3-second reference)
  - Voice design via natural language
  - 10 languages (Chinese, English, Japanese, Korean, German, French, Russian, Portuguese, Spanish, Italian)
  - Streaming generation
- **Best for**: High-quality multilingual synthesis
- **Link**: [QwenLM/Qwen3-TTS](https://github.com/QwenLM/Qwen3-TTS)

### 6. Spark-TTS-0.5B

- **Latency**: Good streaming performance
- **Size**: 0.5B parameters
- **VRAM**: ~3-4GB
- **Features**: Emotion-aware, multilingual
- **Link**: [SparkAudio/Spark-TTS](https://github.com/SparkAudio/Spark-TTS)

---

## 📊 Quick Comparison

| Model | Latency | VRAM | Quality | Voice Clone | Languages |
|-------|---------|------|---------|-------------|-----------|
| Kokoro-82M | <0.3s | ~1GB | Good | ❌ | EN |
| MeloTTS | Real-time | ~2GB | Good | ❌ | Multi |
| Piper | ~50-100ms | CPU | Decent | ❌ | Multi |
| F5-TTS | 0.15 RTF | ~5GB | Excellent | ✅ | Multi |
| Qwen3-TTS-0.6B | 97ms | ~5GB | Excellent | ✅ | 10 langs |
| Spark-TTS | Good | ~4GB | Very Good | ✅ | Multi |

---

## 🚀 Recommendations by Use Case

### If speed is critical:
- **Kokoro-82M** — Sub-0.3s latency, minimal VRAM
- **Piper** — ~50-100ms, runs on CPU

### If you need voice cloning:
- **F5-TTS** — Excellent quality + cloning
- **Qwen3-TTS-0.6B** — 97ms streaming + cloning + multilingual

### Best overall balance for RTX 2080:
- **Qwen3-TTS-0.6B**
  - Streaming with 97ms first-audio latency
  - Voice cloning from 3-second reference
  - Voice design via natural language descriptions
  - Excellent quality while fitting on 8GB VRAM
  - Multilingual support

---

## 🔧 Quick Start Commands

### Kokoro-82M
```bash
pip install kokoro
```

### MeloTTS
```bash
pip install melo-tts
melo-tts --text "Hello world" --output output.wav
```

### Piper
```bash
pip install piper-tts
piper --model en_US-lessac-medium --output_file output.wav < text.txt
```

### F5-TTS
```bash
git clone https://github.com/SWivid/F5-TTS.git
cd F5-TTS
pip install -e .
```

### Qwen3-TTS
```bash
pip install -U qwen-tts
pip install -U flash-attn --no-build-isolation
```

```python
import torch
from qwen_tts import Qwen3TTSModel

model = Qwen3TTSModel.from_pretrained(
    "Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice",
    device_map="cuda:0",
    dtype=torch.bfloat16,
    attn_implementation="flash_attention_2",
)
```

---

## 📝 Notes

- RTX 2080 has 8GB VRAM — avoid models larger than 1.7B parameters
- FlashAttention 2 recommended for faster inference and lower memory
- Consider CPU fallback for extremely long texts to save VRAM
- Streaming models (Qwen3-TTS, F5-TTS) are better for real-time applications

---

## 📚 References

- [12 Best Open-Source TTS Models Compared (2025)](https://www.inferless.com/learn/comparing-different-text-to-speech---tts--models-part-2)
- [Best Self-Hosted TTS Models in 2025 - A2E](https://a2e.ai/best-self-hosted-tts-models-2025/)
- [Qwen3-TTS GitHub](https://github.com/QwenLM/Qwen3-TTS)
- [Qwen3-TTS Paper](https://arxiv.org/abs/2601.15621)

---

*Research conducted: January 2025*
