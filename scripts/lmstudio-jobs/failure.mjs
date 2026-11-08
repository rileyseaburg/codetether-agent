// Persist only fixed error categories, never raw HTTP bodies, headers or exception text.
/** @param {Error} error */
export function failure(error) {
  if (/^Chat API HTTP [1-5][0-9]{2}$/.test(error.message || '')) return error.message;
  if (error.message === 'Invalid text-only completion') return 'invalid_completion';
  if (error.message === 'Completion has no answer text') return 'empty_answer';
  if (error.message === 'Completion model mismatch') return 'model_mismatch';
  if (error.message === 'Job deadline exceeded') return 'deadline';
  if (error.name === 'AbortError' || error.name === 'TimeoutError') return 'aborted_or_timed_out';
  if (error.message === 'CODETETHER_AUTH_TOKEN is required') return 'missing_authentication';
  return 'transport_or_storage_error';
}
