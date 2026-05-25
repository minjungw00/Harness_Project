# Verification Result Card Template

## 사용 시점

Eval 결과의 verdict, assurance 영향, 검토한 evidence, 남은 작업, 사용자 후속 조치를 간결하게 보여줄 때 Verification Result Card를 사용합니다.

## 기준 기록

- Eval 기록
- assurance 영향과 verification independence 상태
- Manual QA와 acceptance 영향
- 검토된 task, run, Evidence Manifest, TDD trace, diff, log, approval, design 참조
- blocker 또는 rework
- 사용자 후속 조치

## 렌더링 섹션

- verification completion
- Eval identity
- verdict
- assurance
- verification independence
- Manual QA
- acceptance
- 검토한 evidence
- 남은 작업
- 사용자 후속 조치

## 전체 템플릿

````text
Verification이 완료되었습니다.

{eval_id}
Verdict: {verdict}
Assurance: {assurance_impact}
Verification independence: {verification_independence}
Manual QA: {manual_qa_impact}
Acceptance: {acceptance_impact}

검토한 evidence:
- task summary: {task_summary_ref}
- run summary: {run_summary_ref}
- evidence manifest: {evidence_manifest_ref}
- TDD trace: {tdd_trace_ref}
- diff: {diff_ref}
- logs: {logs_ref}
- approvals: {approval_refs}
- design refs: {design_refs}
- redaction or blocked input: {redaction_availability_summary|none}

남은 작업:
{blockers_or_rework}

User follow-up:
{user_followup}
````

## 메모

이 template은 렌더링 결과인 카드 형태일 뿐 verification 권한 자체가 아닙니다. Eval 기록과 gate 상태가 기준입니다.

이 card는 omitted 또는 blocked raw bytes를 검토한 것처럼 암시하면 안 됩니다. `secret_omitted`는 visible nonsecret claim만 뒷받침할 수 있고, `blocked`는 documented resolution이 없는 한 unavailable input입니다.
