// @ts-check
export const MAX_BYTES = 512 * 1024;
/** @param {number} width @param {number} height @param {number} [edge] */
export function dimensions(width, height, edge = 1600) {
  if (width <= 0 || height <= 0) throw new Error('Preview is not ready');
  const scale = Math.min(1, edge / Math.max(width, height));
  return {width:Math.max(1, Math.floor(width * scale)), height:Math.max(1, Math.floor(height * scale))};
}
/** Encode only selected video pixels. Discard canvas pixels after use.
 * @param {HTMLVideoElement} video @param {AbortSignal} signal */
export async function capture(video, signal) {
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d', {alpha:false});
  if (!context) throw new Error('Canvas unavailable');
  const size = dimensions(video.videoWidth, video.videoHeight);
  try {
    for (let scale = 1; scale >= 0.1; scale *= 0.7) {
      canvas.width = Math.max(1, Math.floor(size.width * scale));
      canvas.height = Math.max(1, Math.floor(size.height * scale));
      context.drawImage(video, 0, 0, canvas.width, canvas.height);
      for (const quality of [0.85,0.7,0.55,0.4]) {
        signal.throwIfAborted();
        const blob = await new Promise(resolve => canvas.toBlob(resolve, 'image/jpeg', quality));
        signal.throwIfAborted();
        if (blob instanceof Blob && blob.type === 'image/jpeg' && blob.size <= MAX_BYTES) {
          const bytes = new Uint8Array(await blob.arrayBuffer());
          signal.throwIfAborted();
          let binary = '';
          for (const byte of bytes) binary += String.fromCharCode(byte);
          return btoa(binary);
        }
      }
    }
    throw new Error('Screenshot exceeds the safe upload limit');
  } finally { canvas.width = 0; canvas.height = 0; }
}