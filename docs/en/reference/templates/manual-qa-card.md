# Manual QA Card Template

## Used when

Use the Manual QA card when Manual QA needs a compact human-inspection prompt showing the record, gate, profile, target, checklist, evidence to record, and waiver/risk visibility.

This is template reference documentation. It does not authorize runtime/server implementation, generated operational files, executable fixtures, or runtime data before the redesigned docs are accepted. The first implementation/proof target remains Kernel Smoke; Agency-Hardened MVP and post-MVP automation stay out of scope unless their owner docs promote and prove them.

## Source records

- Manual QA requirement and `qa_gate`
- Manual QA record, if one exists
- QA profile
- human inspector or role and the human judgment being requested
- target screen or flow
- checklist items
- expected screenshot, walkthrough note, browser log, Browser QA artifact, or manually supplied artifact evidence
- waiver reason, QA waiver Decision Packet refs when required, and Residual Risk refs when QA is waived or deferred
- verification, acceptance, and close-impact summaries

Close context and waiver placeholders are derived display summaries from QA records, `qa_gate`, related gate states, Decision Packet refs, and Residual Risk refs. Waiver paths should render those refs or say that recording is still needed.

## Rendered sections

- Manual QA requirement
- record
- gate
- profile
- target
- checklist
- evidence to record
- close context
- waiver recording
- result prompt

## Full template

````text
Manual QA is required.
Display only: `qa_gate` and QA records remain canonical.
Human inspection only: automated checks, screenshots, browser logs, and Browser QA artifacts can support context, but they are not Manual QA by themselves.
Browser QA Capture: useful when promoted and supported; not final acceptance, not detached verification without an independent Eval, and not a replacement for required human judgment.

Record: {manual_qa_record_id|none until recorded}
Gate: {qa_gate display: not_required|required|pending|passed|failed|waived}
Profile: {profile}
Human judgment requested: {human_inspection_summary}
Target: {screen_or_flow}
Checklist:
- {checklist_item}

Evidence to record:
- screenshot or walkthrough note
- qa_capture artifact when promoted and supported
- browser log when relevant
- manually supplied artifact or human note when browser capture is unsupported
- redaction/omission/block note when evidence cannot be recorded as raw content

Close context:
- automated checks: {check_refs|none; not a Manual QA result}
- Browser QA artifacts: {artifact_refs|none; supporting refs only}
- verification impact: {verification_impact}
- acceptance impact: {acceptance_impact}
- residual risk or follow-up: {residual_risk_or_follow_up|none}

Waiver recording:
- skipped Manual QA surface:
- risk visible before waiver:
- accepted risk:
- follow-up:
- relevant refs:
- close impact:
- waiver source: {manual_qa_record_id and waiver_reason; waiver_decision_packet_ref when user-owned risk is involved}

Record the Manual QA result, record an allowed low-risk QA waiver reason, or request a QA waiver Decision Packet for user-owned risk?
````

## Notes

This template is a rendered card shape, not canonical QA state. `qa_gate` remains the close-relevant gate.

Manual QA is human inspection. Passing tests, browser smoke, screenshot capture, Browser QA Capture artifacts, verification, or user acceptance may support the close context, but they do not become Manual QA unless `record_manual_qa` records a Manual QA result or a valid QA waiver updates `qa_gate=waived` with a waiver reason and, when user-owned risk is involved, a compatible QA waiver Decision Packet. Browser QA Capture remains a v1/post-MVP candidate unless owner docs explicitly promote it, and captured artifacts do not record final acceptance or detached verification unless a separate Eval path satisfies independence. A chat statement alone is not enough when the waiver affects close or accepted risk.

The card may ask for replacement evidence or waiver recording when an artifact is `secret_omitted` or `blocked`, but it must not display omitted values or blocked raw capture content. When browser capture is unsupported for the surface, the card should ask for human Manual QA notes and manually supplied artifacts instead of treating capture absence as a QA result.
