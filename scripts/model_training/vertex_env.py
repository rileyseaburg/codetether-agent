"""Environment variables permitted in a Vertex worker specification."""

SECRET_VARS = ('VAULT_ADDR', 'VAULT_TOKEN')

PASSTHROUGH_VARS = (
    'CODETETHER_BASE_MODEL',
    'CODETETHER_GPU_COUNT',
    'CODETETHER_MAX_LENGTH',
    'CODETETHER_MAX_TURNS',
    'CODETETHER_RUN_ID',
)
