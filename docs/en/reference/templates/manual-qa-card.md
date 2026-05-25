# Manual QA Card Template

## Used when

Use the Manual QA card when required Manual QA needs a compact prompt showing the record, gate, profile, target, checklist, and evidence to record.

## Source records

- Manual QA requirement and `qa_gate`
- Manual QA record, if one exists
- QA profile
- target screen or flow
- checklist items
- expected screenshot, walkthrough note, or browser log evidence
- Decision Packet or required judgment-path refs, plus Residual Risk refs, when QA is waived or deferred
- verification, acceptance, and close-impact summaries

## Rendered sections

- Manual QA requirement
- record
- gate
- profile
- target
- checklist
- evidence to record
- close context
- waiver path
- result prompt

## Full template

````text
Manual QA is required.
Display only: `qa_gate` and QA records remain canonical.

Record: {manual_qa_record_id|none until recorded}
Gate: {qa_gate display: pending|passed|failed|waived|not_required}
Profile: {profile}
Target: {screen_or_flow}
Checklist:
- {checklist_item}

Evidence to record:
- screenshot or walkthrough note
- browser log when relevant
- redaction/omission/block note when evidence cannot be recorded as raw content

Close context:
- automated checks: {check_refs|none; not a Manual QA result}
- verification impact: {verification_impact}
- acceptance impact: {acceptance_impact}
- residual risk or follow-up: {residual_risk_or_follow_up|none}

Waiver path:
- skipped Manual QA surface:
- accepted risk:
- follow-up:
- relevant refs:
- close impact:
- Judgment path: {decision_packet_or_required_judgment_ref|required before waiver is treated as user-owned judgment}

Record the Manual QA result, or route a QA waiver through the required judgment path?
````

## Notes

This template is a rendered card shape, not canonical QA state. `qa_gate` remains the close-relevant gate.

Manual QA is human inspection. Passing tests, browser smoke, screenshot capture, verification, or user acceptance may support the close context, but they do not become Manual QA unless a Manual QA result is recorded or a valid QA waiver is accepted through the required judgment path. A chat statement alone is not enough when the waiver affects close or accepted risk.

The card may ask for replacement evidence or a waiver path when an artifact is `secret_omitted` or `blocked`, but it must not display omitted values or blocked raw capture content.
