// Chat media is neutral; the legacy directory remains readable for old messages.
import { join } from 'node:path';
/** @param {string} home @returns {string[]} */
export function mediaRoots(home) {
  return [
    join(home, '.local/share/codetether-mobile/uploads'),
    join(home, '.local/share/codetether-chat/.codetether-agent/generated_images'),
    join(home, 'spotlessbinco/.codetether-agent/generated_images'),
  ];
}