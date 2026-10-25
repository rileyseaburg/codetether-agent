Generate images from detailed descriptions or edit existing images from specific instructions.
Use this tool for requested diagrams, portraits, comics, memes, illustrations, and other raster visuals.
It can add or remove elements, change colors, improve quality, or transform an image's style.

The default provider is `openai`. For that provider, omit both image selectors to generate a new image. For edits, use `referenced_image_paths` when
the targets are local PNG, JPEG, or WebP files. Use `num_last_images_to_include` only to edit
images produced by recent `image_gen` calls in this process. Select the smallest sufficient count,
up to five. Never provide both selectors. Generated PNGs are returned to the conversation and
saved under the CodeTether data directory.

Set `provider` to `gemini-web` to use Gemini's private web protocol with a configured Google
session. Cookie renewal and image retrieval are handled by native Rust. For improvements to a
Gemini image, set `refine_from` to the previous result's `refinement_path`, output folder, or
saved image path, and describe the changes in `prompt`. Each version is saved separately.
Gemini Web does not support arbitrary image uploads or the other image selectors in this tool.