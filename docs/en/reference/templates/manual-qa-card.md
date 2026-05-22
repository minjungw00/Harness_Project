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

## Rendered sections

- Manual QA requirement
- record
- gate
- profile
- target
- checklist
- evidence to record
- result prompt

## Full template

````text
Manual QA is required.

Record: {manual_qa_record_id|none until recorded}
Gate: {qa_gate display: pending|passed|failed|waived|not_required}
Profile: {profile}
Target: {screen_or_flow}
Checklist:
- {checklist_item}

Evidence to record:
- screenshot or walkthrough note
- browser log when relevant

Record the QA result?
````

## Notes

This template is a rendered card shape, not canonical QA state. `qa_gate` remains the close-relevant gate.
