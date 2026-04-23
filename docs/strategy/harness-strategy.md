# 하네스 전략 문서 v04

## 1. 문서 목적

이 문서는 하네스의 전략 층위를 정의한다.

이 문서는 다음을 다룬다.

- 하네스가 해결해야 하는 문제
- 하네스가 유지해야 하는 설계 입장
- 핵심 불변식
- 사용자 작업 모델
- 상태 축의 의미
- Task, Change Unit, Evidence, Artifact의 전략적 의미
- approval, assurance, acceptance의 분리 원칙
- 사용자 표면, 에이전트 표면, 운영 표면의 관계
- agent surface 통합의 전략적 역할

이 문서는 다음을 직접 확정하지 않는다.

- 데이터베이스 스키마
- 디렉터리 구조와 파일 경로
- CLI 인자와 명령 체계
- 외부 제품별 설정 파일 위치
- Markdown front matter의 세부 규격
- validator 구현 알고리즘
- 특정 제품 버전에 종속되는 기능 판단

## 2. 하네스가 해결해야 하는 문제

AI와 함께 개발할 때 기본값으로 자주 발생하는 문제는 다음과 같다.

- 현재 작업 상태가 사용자에게 충분히 읽히지 않는다.
- 작업 범위와 리스크가 대화 흐름에 따라 흔들린다.
- 중요한 선택의 이유와 trade-off가 대화 속에만 남고 누적되지 않는다.
- 작은 수정과 구조적 작업이 같은 절차로 처리된다.
- 구현자가 스스로 검토한 결과가 완료 판정으로 과도하게 사용된다.
- handoff가 늘어날수록 맥락이 압축되고 누락된다.
- 재개와 복구가 채팅 기록, 모델 기억, 사람의 기억에 의존한다.
- 사용자가 approval, assurance, acceptance를 구분하기 어렵다.
- 사용자가 내부 명령과 상태 개념을 직접 조합해야 해서 일상 사용이 복잡해진다.
- 특정 IDE, 모델, CLI의 기능을 하네스의 코어 원칙처럼 다루는 위험이 생긴다.

하네스는 더 많은 역할을 만들기 위한 구조가 아니다. 하네스는 AI 작업을 사용자가 읽고, 제한하고, 검증하고, 다시 이어받을 수 있는 형태로 유지하기 위한 운영 구조다.

## 3. 설계 입장

### 3.1 사용자가 프로젝트 방향과 최종 판단을 가진다

사용자는 목표, 우선순위, 승인, 수용 판단을 가진다. 하네스는 실행을 돕지만 프로젝트 소유권을 대체하지 않는다.

에이전트는 선택지, 근거, 리스크, 현재 상태를 드러내고, 사용자는 그 정보를 바탕으로 최종 결정을 내린다.

### 3.2 기본 사용 경험은 대화 중심이다

사용자는 일상 작업에서 하네스 명령을 조합하지 않는다.

기본 흐름은 다음이다.

```text
사용자 자연어 요청
→ 에이전트가 하네스 상태 확인
→ 하네스가 Task, mode, next action 판단
→ 에이전트가 필요한 Skill과 MCP tool 사용
→ 하네스가 승인, 범위, 증거, 검증을 집행
→ 에이전트가 사용자에게 상태와 판단 지점을 보고
```

CLI는 설치, 진단, 복구, export, CI, 운영자용 디버그 표면으로 남는다.

### 3.3 가장 작은 유효 워크플로를 기본값으로 둔다

하네스는 모든 작업을 같은 무게로 처리하지 않는다.

- 설명, 비교, 리뷰가 목적이면 `advisor`
- 작고 저위험이며 결과 확인이 분명하면 `direct`
- 구조화, 구현, 독립 검증이 필요하면 `work`

기본값은 절차와 역할을 늘리는 것이 아니라, 작업에 필요한 경계만 남기는 것이다.

### 3.4 하나의 Task는 하나의 연속 맥락을 가진다

하나의 Task는 처음부터 끝까지 이어지는 연속 맥락을 가진다.

이 연속성은 특정 에이전트, 특정 모델, 특정 IDE, 특정 채팅 세션이 아니라 Task 자체에 속한다.

하네스는 불필요한 handoff를 기본 구조로 삼지 않는다.

### 3.5 가장 중요한 분리 경계는 실행과 검증 사이에 둔다

shaping과 implementation은 하나의 주 실행 맥락에서 이어질 수 있다.

verification은 가능한 한 분리된 검증 맥락에서 수행한다.

같은 긴 대화 세션에서 실행자가 스스로 검토한 결과는 detached verification이 아니다.

### 3.6 대화보다 상태, 문서, 증거를 신뢰한다

재개, 판정, 추적의 기준은 대화 이력이 아니라 상태 저장소, 문서, diff, logs, checkpoint, bundle 같은 durable artifact다.

대화는 사용자의 조작 표면이자 설명 표면일 수 있지만, authoritative source는 아니다.

### 3.7 사용자 표면은 행동과 근거를 먼저 보여준다

사용자에게 먼저 필요한 정보는 다음이다.

- 다음 행동
- 남아 있는 사용자 판단
- 현재 리스크
- 증거가 충분한지 여부
- 최신 실행 또는 검증 결과
- approval 또는 acceptance 대기 여부

하네스는 사용자 표면을 action-first, evidence-first로 설계한다.

### 3.8 Skill, MCP, hook, adapter의 책임을 분리한다

하네스 통합은 네 가지 층위를 분리한다.

- 항상 읽히는 rule/context: 짧은 기본 원칙
- Skill 또는 playbook: 언제 하네스를 쓰고 어떤 절차를 따를지 알려주는 안내
- MCP: 상태, approval, artifact, validator, verification을 조작하는 도구 계층
- hook/adapter/sidecar/validator: 허용 범위, approval, evidence capture, same-session verify guard를 집행하는 계층

규칙 파일과 Skill은 에이전트에게 절차를 알려줄 수 있지만, 정책을 강제하지는 못한다.

강제 집행은 상태 전이, MCP tool, validator, hook, adapter, sidecar, 격리 실행 환경이 맡는다.

### 3.9 전문화는 제한된 보조 수단이다

specialist나 subagent는 기본 워크플로가 아니다.

허용되는 사용은 다음처럼 목적이 좁고 경계가 분명한 경우다.

- 병렬 읽기 전용 탐색
- 특정 도구 호출 격리
- 독립 교차검토
- 제한된 범위의 분석 보조

specialist는 Task ownership, approval close, final verdict, completion close를 대신하지 않는다.

### 3.10 자동화는 가시성과 통제 이후에 확장한다

하네스는 사용자가 모르는 사이에 작업을 진행하고 완료하는 자동화를 기본값으로 삼지 않는다.

자동화는 다음이 안정적으로 보장된 뒤 확장한다.

- 현재 상태가 읽힌다.
- 다음 행동이 드러난다.
- 남은 사용자 판단이 표시된다.
- 승인 범위가 확인된다.
- 검증 근거가 남는다.
- 결과 수용 여부가 분리된다.
- 재개와 복구가 대화 없이 가능하다.

### 3.11 코어 계약과 agent surface를 분리한다

하네스의 코어는 특정 IDE, 특정 CLI, 특정 모델, 특정 제품 기능에 종속되지 않는 운영 계약이다.

Codex, Claude Code, Gemini, GitHub Copilot, Cursor 같은 도구는 하네스를 사용하는 agent surface다.

각 표면의 capability는 검증된 profile로 선언하고, 부족한 집행 능력은 sidecar, validator, fresh evaluator, 격리 실행으로 보완한다.

## 4. 핵심 불변식

### 4.1 사용자는 자연어로 작업을 시작할 수 있어야 한다

사용자는 다음과 같은 말로 작업을 시작하거나 이어갈 수 있어야 한다.

```text
이 작업 하네스 기준으로 진행해.
상태 보여줘.
이 작업 이어서 해.
승인해. 범위는 방금 설명한 내용까지만.
detached verify 시작해.
수용해. 작업 닫아.
```

하네스 내부 mode, phase, Change Unit, run, evaluator 선택은 에이전트와 하네스가 처리한다.

### 4.2 제품 파일 쓰기 전에는 scope와 approval을 확인한다

제품 파일 쓰기, 상태 변경 명령, network write, secret access, destructive operation은 하네스의 scope와 approval 검사를 통과해야 한다.

instruction만으로 이 불변식을 만족했다고 보지 않는다.

### 4.3 변경 후에는 evidence를 남긴다

변경 파일, diff, 실행한 명령, logs, checkpoint, baseline, 관련 approval, 관련 decision은 나중에 검증과 재개가 가능하도록 durable evidence로 남긴다.

증거가 부족하면 작업 상태는 완료가 아니라 partial, blocked, 또는 verify pending으로 남는다.

### 4.4 work는 실행자의 자기 보고만으로 닫지 않는다

`work` 작업은 detached verification 또는 그에 준하는 독립 검증 경계를 거치지 않고 완료로 닫지 않는다.

검증 경계의 독립성은 별도로 기록한다.

### 4.5 approval, assurance, acceptance는 서로 다른 질문이다

하네스는 다음 세 질문을 분리한다.

- approval: 진행해도 되는가
- assurance: 기술적으로 어느 수준까지 확인되었는가
- acceptance: 사용자가 결과와 남은 trade-off를 받아들이는가

이 셋은 서로를 대체하지 않는다.

### 4.6 재개와 복구는 대화 없이 가능해야 한다

현재 상태, 마지막 판정, 다음 행동, 관련 승인, 최신 증거, 재개 포인트는 상태 저장소와 문서, artifact만으로 다시 구성할 수 있어야 한다.

### 4.7 하나의 사실에는 하나의 authoritative source가 있다

중복 표시는 가능하지만 기준 원본은 하나여야 한다.

projection은 허용되지만 source-of-truth와 projection은 분리해서 관리한다.

### 4.8 사람이 읽는 문서는 현재 요약과 판단 가능성에 집중한다

저장소 문서는 모든 raw trace와 모든 과거 대화를 담는 공간이 아니다.

문서는 현재 상태, 다음 행동, 남은 판단, 핵심 근거, 관련 artifact 참조를 읽게 해야 한다.

오래된 상세 이력은 별도 report, decision, approval, evidence artifact로 보관한다.

### 4.9 사람이 문서를 수정할 수 있지만 운영 상태는 명시적으로 reconcile한다

사람이 `TASK` 같은 projection 문서에 메모나 제안을 남길 수 있다.

그러나 managed projection을 직접 수정한 내용은 자동으로 운영 상태가 되지 않는다.

하네스는 사람 수정 사항을 proposal 또는 reconcile 대상 이벤트로 감지하고, 명시적 상태 변경을 통해 반영한다.

### 4.10 민감 범주는 명시적 approval 없이 진행하지 않는다

민감 범주는 작업을 진행하기 전에 approval이 필요하다.

대표 범주는 다음이다.

- 인증, 권한, 보안 정책 변경
- schema, migration, persistence 변경
- dependency 추가, 제거, 업그레이드
- public API 또는 외부 contract 변경
- destructive write
- network write 또는 외부 서비스 write
- secret access
- production config, deployment, CI/CD, infra 변경
- privacy, PII, data export, telemetry 변경
- license, compliance, billing 또는 비용 영향 변경
- policy override

approval은 범주만이 아니라 허용 경로, 도구, network, secret scope, baseline, 만료 조건을 함께 가진다.

### 4.11 agent surface capability는 선언되고 검증되어야 한다

표면이 하네스 규칙을 읽을 수 있는지, MCP를 호출할 수 있는지, hook을 제공하는지, 사전 guard를 집행할 수 있는지, fresh evaluator를 실행할 수 있는지는 surface profile로 기록한다.

하네스는 제품명만으로 capability를 가정하지 않는다.

## 5. 사용자 작업 모델

### 5.1 advisor

`advisor`는 읽기, 비교, 설명, 리뷰, 의사결정 정리에 사용하는 모드다.

적합한 작업은 다음이다.

- 구조 설명
- 코드 리뷰
- 설계 선택지 비교
- 테스트 전략 검토
- 결정 초안 작성

`advisor`는 제품 코드 변경을 전제하지 않는다.

### 5.2 direct

`direct`는 작고 저위험이며 결과 확인이 명확한 변경을 빠르게 처리하는 모드다.

적합한 작업은 다음이다.

- 오타 수정
- 명백한 import 경로 보정
- 문서 링크 정리
- 범위가 좁은 테스트 보정
- 국소 버그 수정

`direct`는 가능한 한 한 번의 실행으로 닫지만, 범위가 커지거나 민감 범주가 드러나면 같은 Task를 `work`로 전환한다.

### 5.3 work

`work`는 구조화, 구현, 독립 검증이 필요한 일반 개발 모드다.

적합한 작업은 다음이다.

- 기능 추가
- 구조 변경
- 비국소 버그 수정
- 리팩터링
- 테스트 전략 강화
- 단계적 구현

`work`는 shaping, implementation, verification을 가지며, 최종 판정은 분리된 검증 경계를 통해 닫는다.

## 6. 상태 축의 의미

하네스는 단일 문자열 상태 대신 여러 축을 사용한다.

### 6.1 mode

사용자 관점의 작업 유형이다. `advisor`, `direct`, `work`를 구분한다.

### 6.2 phase

현재 진행 단계다. intake, shaping, ready, executing, verifying, waiting_user, blocked, completed, cancelled 같은 흐름을 표현한다.

### 6.3 result

현재까지 나온 종료 성격 또는 판정이다. advice성 종료인지, 구현이 통과했는지, 실패했는지, 취소되었는지를 구분한다.

### 6.4 assurance

기술적 검증을 통해 결과가 어느 수준까지 확인되었는지를 나타낸다.

assurance는 approval이나 acceptance와 다르다.

### 6.5 verification independence

검증이 얼마나 독립적인지를 나타내는 내부 qualifier다.

예시는 다음이다.

- same session self-check
- read-only subcontext review
- fresh session review
- fresh worktree review
- sandboxed review

사용자 표면은 단순한 assurance 값을 보여주되, 고위험 작업에서는 independence qualifier를 함께 보여준다.

### 6.6 approval

민감한 범주의 변경을 진행해도 되는지에 대한 사전 허가 상태다.

### 6.7 acceptance

검증 이후 사용자가 결과와 남은 trade-off를 받아들였는지에 대한 사후 수용 상태다.

### 6.8 risk

현재 작업의 운영 리스크 수준이다.

### 6.9 evidence

독립 검증과 재개에 필요한 증거가 충분한지, 오래되어 신뢰하기 어려운지를 나타낸다.

사용자 표면은 이 축들을 조합해 간단한 display state를 보여줄 수 있다. display state는 파생 표현이며, 기준 의미는 각 상태 축에 있다.

## 7. Task, Change Unit, Evidence, Artifact

### 7.1 Task

Task는 사용자 가치 단위다.

하나의 Task는 다음을 명확히 가진다.

- 목적
- 기대 결과
- 범위와 비범위
- acceptance 기준
- 현재 요약
- 다음 행동
- 남은 사용자 판단
- 관련 증거와 결정

### 7.2 Task Spine

Task의 continuity anchor는 세 층을 가진다.

- Current Summary: 사용자가 지금 바로 알아야 할 상태와 다음 행동
- Rolling Spine: 현재 유효한 사실, 가정, 결정, rejected option, watchpoint, resume note
- Snapshot References: run summary, eval, approval, decision, evidence artifact 참조

`TASK` 문서는 현재를 이해하기 위한 중심 문서다. 모든 과거 상세 이력을 계속 쌓는 문서가 아니다.

### 7.3 Change Unit

Change Unit은 실제 구현 단위다.

하나의 Change Unit은 제한된 목적, 허용 경로, 허용 도구, 검증 조건, 리스크 flag를 가진다.

하네스는 큰 Task를 Change Unit으로 나누되, 사용자가 이해하기 어려운 미세 분할을 기본값으로 두지 않는다.

### 7.4 Evidence Manifest

Evidence Manifest는 acceptance criteria와 evidence의 대응 관계를 보여준다.

검증자는 “무엇을 검토했는가”만이 아니라, “어떤 기준이 어떤 증거로 뒷받침되는가”를 확인해야 한다.

### 7.5 Artifact

Artifact는 나중에 재개, 판정, 추적, 감사를 위해 남기는 durable output이다.

예시는 다음이다.

- `TASK`
- `DEC`
- `APR`
- `RUN-SUMMARY`
- `EVAL`
- `DIRECT-RESULT`
- evidence manifest
- bundle
- diff
- logs
- checkpoint

## 8. source-of-truth 원칙

하네스는 다음 원칙을 따른다.

- 용어와 기본 의미의 기준은 전략 문서다.
- 운영 상태의 기준은 상태 저장소다.
- Task의 현재 의도와 연속 맥락의 기준은 `TASK` 문서와 상태 저장소의 결합이다.
- 결정 근거의 기준은 `DEC` 문서다.
- 승인 판단의 사람용 요약 기준은 `APR` 문서다.
- raw evidence의 기준은 artifact 저장소다.
- 문서 front matter와 display state는 projection이다.
- 에이전트 대화는 사용 표면이지만 source-of-truth가 아니다.
- generated rule, Skill, connector 파일은 connector manifest가 관리하는 projection이다.

핵심은 같은 사실을 여러 곳에 흩뿌리는 것이 아니라, 기준 원본과 표시용 projection을 구분하는 것이다.

## 9. approval, verification, acceptance 운영 원칙

### 9.1 approval

approval은 작업을 진행해도 되는지에 대한 사용자 결정이다.

민감한 범주의 변경은 명시적 approval 없이 진행하지 않는다.

approval은 scope-bound contract다.

승인 범위가 바뀌면 기존 approval을 조용히 재사용하지 않는다.

### 9.2 verification

verification은 결과가 acceptance 기준을 충족하는지에 대한 판정이다.

verification은 가능한 한 fresh run, 구조화된 evidence, 재실행 가능한 check를 사용한다.

same-session self-review는 detached verification으로 승격하지 않는다.

### 9.3 acceptance

acceptance는 사용자가 결과와 남은 trade-off를 받아들이는지에 대한 최종 판단이다.

모든 작업이 explicit acceptance를 요구하지는 않는다.

고위험 작업, 사용자 선택이 남아 있는 작업, trade-off가 남은 작업은 acceptance를 별도 상태로 남긴다.

## 10. 사용자 표면, 에이전트 표면, 운영 표면

### 10.1 사용자 표면

사용자 표면은 사용자가 에이전트와 대화하는 곳이다.

사용자 표면은 다음 정보를 우선 보여준다.

- next action
- pending decision
- risk
- evidence
- 최신 report
- approval 또는 acceptance 대기 여부

### 10.2 에이전트 표면

에이전트 표면은 LLM 개발 도구다.

에이전트 표면은 하네스의 사용자 경험을 제공하지만, 하네스의 source-of-truth가 아니다.

에이전트 표면은 다음을 수행한다.

- 사용자의 자연어 요청 해석
- Harness Skill 또는 rule 로드
- Harness MCP 호출
- 작업 수행
- evidence 전달
- 사용자에게 상태와 판단 지점 보고

### 10.3 운영 표면

운영 표면은 다음을 다룬다.

- 상태 저장
- 상태 전이
- validator 실행
- projection 갱신
- approval 집행
- artifact 등록
- recovery와 reconcile
- adapter와 security policy
- hook/sidecar 기반 강제 집행

운영 표면의 복잡성을 사용자 표면으로 그대로 밀어 넣지 않는다.

## 11. 비목표

현재 하네스 전략의 비목표는 다음이다.

- 인간 조직 직함을 기본 아키텍처로 삼는 것
- multi-agent 구조를 기본값으로 강제하는 것
- 사용자가 내부 CLI를 조합해야만 작업할 수 있게 만드는 것
- 채팅 기록만으로 재개와 판정을 수행하는 것
- 규칙 파일이나 prompt만으로 보안과 검증을 집행하려는 것
- 전체 흐름이 읽히지 않는 상태에서 완전 자동화를 우선하는 것
- 특정 IDE, 특정 CLI, 특정 모델에 코어 계약을 고정하는 것
- 에이전트 표면의 memory를 하네스 상태 저장소로 사용하는 것
- 모든 agent surface connector를 한 번에 완성하는 것을 core 품질보다 우선하는 것

## 12. 요약

하네스의 중심은 역할 수나 명령 수가 아니라 다음 네 가지다.

- 연속 맥락
- 명시적 경계
- 독립 검증
- durable evidence

사용자는 에이전트와 대화하고, 에이전트는 Harness Skill과 Harness MCP를 통해 하네스를 조작한다.

하네스는 AI를 더 많이 조직화하기 위한 프레임워크가 아니라, AI 작업을 사용자가 읽고 통제할 수 있는 구조 안에 두기 위한 운영 계약이다.
