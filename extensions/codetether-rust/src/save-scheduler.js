'use strict';

/** Coalesce saves into one check plus at most one rerun. Example: scheduler.schedule(folder). */
function createScheduler({ delay, busy, run, report, timers = { setTimeout, clearTimeout } }) {
  const pending = new Map();
  let disposed = false;
  async function flush(key) {
    const item = pending.get(key);
    if (!item || disposed) return;
    item.timer = undefined;
    if (busy(key)) return;
    pending.delete(key);
    try { await run(item.folder); } catch (error) { report(error); }
  }
  return {
    schedule(folder) {
      if (disposed) return;
      const key = folder.uri.toString();
      const previous = pending.get(key);
      if (previous?.timer) timers.clearTimeout(previous.timer);
      const item = { folder, timer: timers.setTimeout(() => flush(key), delay(folder)) };
      pending.set(key, item);
    },
    idle(key) { if (pending.has(key) && !pending.get(key).timer) void flush(key); },
    cancel(key) {
      const item = pending.get(key);
      if (item?.timer) timers.clearTimeout(item.timer);
      pending.delete(key);
    },
    dispose() {
      disposed = true;
      for (const item of pending.values()) if (item.timer) timers.clearTimeout(item.timer);
      pending.clear();
    }
  };
}
module.exports = { createScheduler };