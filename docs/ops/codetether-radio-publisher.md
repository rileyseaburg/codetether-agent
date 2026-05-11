# CodeTether Radio Temporary Publisher

The public podcast site/API at `voice.quantum-forge.io` is down with Cloudflare
`530 / 1033`, so episode publishing is currently handled by a TetherScript-backed
filesystem upload path plus a lightweight LAN publisher.

## Published feed

- Feed: `http://192.168.50.101:8025/published/ef302249/feed.xml`
- Episode MP3: `http://192.168.50.101:8025/published/ef302249/episodes/recruitswarm.mp3`

## TetherScript upload path

Plugin:

```text
examples/tetherscript/podcast_upload_path.tether
```

The plugin writes episode JSON, updates `podcast.json`, and regenerates RSS
without requiring the HTTP podcast API to be healthy.

## Runtime service

A user systemd service keeps the temporary publisher alive:

```bash
systemctl --user status codetether-radio-publisher.service
systemctl --user restart codetether-radio-publisher.service
journalctl --user -u codetether-radio-publisher.service -f
```

Service file:

```text
~/.config/systemd/user/codetether-radio-publisher.service
```

Published files live under:

```text
/home/riley/qwen-tts-api/outputs/podcast_publish/ef302249/
```

## Validation

```bash
curl http://192.168.50.101:8025/published/ef302249/feed.xml
curl -r 0-15 http://192.168.50.101:8025/published/ef302249/episodes/recruitswarm.mp3 | xxd
```

The MP3 should begin with an `ID3` header.


## Episode art

Generated art is served alongside the temporary feed:

- RSS square cover: `http://192.168.50.101:8025/published/ef302249/art/recruit-the-swarm-rss-cover.png`
- YouTube thumbnail: `http://192.168.50.101:8025/published/ef302249/art/recruit-the-swarm-youtube-thumbnail.png`

The RSS feed includes channel-level `<itunes:image>` / `<image>` tags and
item-level `<itunes:image>` plus `<media:thumbnail>` for the YouTube artwork.
