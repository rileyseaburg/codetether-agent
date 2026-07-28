"""Generate a Vertex AI custom training job specification.

Secrets are passed as environment variables at submission time and are never
written into the generated YAML that lands on disk.
"""

import argparse
import json
import os

from pathlib import Path

from .vertex_env import PASSTHROUGH_VARS, SECRET_VARS
from .vertex_machine import gpu_count
from .vertex_spec import JobRequest, build


def main() -> None:
    """Write a job specification for `gcloud ai custom-jobs create`."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--image', required=True)
    parser.add_argument('--machine', required=True)
    parser.add_argument('--accelerator', required=True)
    parser.add_argument('--bucket', required=True)
    parser.add_argument('--hf-repo', required=True)
    parser.add_argument('--epochs', default='1')
    parser.add_argument('--preemptible', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    environment = {name: os.environ[name] for name in SECRET_VARS}
    environment['CODETETHER_HF_REPO'] = values.hf_repo
    environment['CODETETHER_GCS_BUCKET'] = values.bucket
    environment['CODETETHER_EPOCHS'] = values.epochs
    for name in PASSTHROUGH_VARS:
        if os.environ.get(name):
            environment[name] = os.environ[name]
    spec = build(
        JobRequest(
            image=values.image,
            machine=values.machine,
            accelerator=values.accelerator,
            environment=environment,
            preemptible=values.preemptible,
            count=gpu_count(),
        )
    )
    values.output.write_text(json.dumps(spec, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'config': str(values.output), 'machine': values.machine}))


if __name__ == '__main__':
    main()
