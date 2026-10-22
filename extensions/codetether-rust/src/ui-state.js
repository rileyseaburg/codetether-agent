'use strict';

/** Track only owned task executions, independently for every workspace. */
function create() {
  const active = new Map(), last = new Map();
  return {
    start(execution) {
      const definition = execution.task.definition;
      if (definition.type === 'codetether-rust') active.set(execution, definition);
    },
    end(execution, exitCode) {
      const definition = active.get(execution);
      if (!definition) return;
      active.delete(execution);
      last.set(definition.workspace, { action: definition.action, exitCode });
    },
    get(key) {
      return { running: [...active.values()].filter(item => item.workspace === key).map(item => item.action), last: last.get(key) };
    }
  };
}
module.exports = { create };