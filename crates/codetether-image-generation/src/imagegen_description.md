Generate images from detailed descriptions or edit existing images from specific instructions.
Use this tool for requested diagrams, portraits, comics, memes, illustrations, and other raster visuals.
It can add or remove elements, change colors, improve quality, or transform an image's style.

Omit both image selectors to generate a new image. For edits, use `referenced_image_paths` when
the targets are local PNG, JPEG, or WebP files. Use `num_last_images_to_include` only to edit
images produced by recent `image_gen` calls in this process. Select the smallest sufficient count,
up to five. Never provide both selectors. Generated PNGs are returned to the conversation and
saved under the CodeTether data directory.
