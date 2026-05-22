# Template 참조

## 사용 시점

MVP-required projection template이 render하는 Markdown 형태를 확인할 때 이 파일들을 사용합니다. Projection rule과 권한 경계는 [문서 Projection 참조](../document-projection.md)가 정의합니다. Freshness behavior도 같은 문서가 담당합니다.

## Template tiering

Projection template은 API `ProjectionKind` tier와 일치합니다.

| Tier | Templates | Rule |
|---|---|---|
| MVP-required | `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT` | MVP projector는 이를 render해야 합니다. |
| MVP-optional | `MANUAL-QA`, `TDD-TRACE`, `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT` | Policy가 적용되거나, record가 있거나, user/operator가 projection을 켰을 때 render합니다. |
| Extension / appendix | `DEC`, `DESIGN`, `EXPORT`, `JOURNEY-CARD` | 해당 extension 또는 appendix projection이 켜져 있을 때만 render합니다. |

Template은 render된 형태일 뿐 canonical state가 아닙니다. Kernel field, MCP schema, SQLite DDL, gate behavior, artifact integrity rule을 재정의하면 안 됩니다.

## MVP-required templates

- [TASK](task.md)
- [APR](approval.md)
- [RUN-SUMMARY](run-summary.md)
- [EVIDENCE-MANIFEST](evidence-manifest.md)
- [EVAL](eval.md)
- [DIRECT-RESULT](direct-result.md)

## Notes

분리된 MVP-required template body를 이 디렉터리에서 찾습니다. Optional 및 extension template body의 현재 위치는 이동 전까지 legacy consolidated [Appendix A](../../appendix/A-template-library.md)입니다.
