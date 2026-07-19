"""Route the external HTTP controller through the atomic author service."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/monitor_api.py")
OLD = '''    if routed_metadata.get('protocol') == 'codetether.forgejo-author.v1':
        from .forgejo_author_task import prepare as prepare_author_task

        try:
            deterministic_id, existing = await prepare_author_task(routed_metadata)
        except ValueError as error:
            raise HTTPException(status_code=422, detail=str(error)) from error
        except LookupError as error:
            raise HTTPException(status_code=409, detail=str(error)) from error
        except RuntimeError as error:
            raise HTTPException(status_code=503, detail=str(error)) from error
        if existing:
            return existing
        await _validate_target_worker_is_available(routed_metadata, strict=True)
        task = await bridge.create_task(
            codebase_id=effective_workspace_id,
            title=task_data.title,
            prompt=task_data.prompt,
            agent_type=task_data.agent_type,
            priority=task_data.priority,
            model=routed_metadata.get('model'),
            metadata=routed_metadata,
            model_ref=routing_decision.model_ref,
            task_id=deterministic_id,
            require_persistence=True,
        )
        if not task:
            raise HTTPException(
                status_code=503,
                detail='Verified author task could not be durably persisted',
            )
        return task
'''
NEW = '''    if routed_metadata.get('protocol') == 'codetether.forgejo-author.v1':
        from .forgejo_author_service import create as create_author_task

        try:
            return await create_author_task(
                bridge,
                task_data,
                routed_metadata,
                routing_decision,
                effective_workspace_id,
                _validate_target_worker_is_available,
            )
        except ValueError as error:
            raise HTTPException(status_code=422, detail=str(error)) from error
        except LookupError as error:
            raise HTTPException(status_code=409, detail=str(error)) from error
        except RuntimeError as error:
            raise HTTPException(status_code=503, detail=str(error)) from error
'''


def main() -> None:
    """Replace exactly one inline author business-logic block."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("author service controller anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()