# Rewrite Manifest

이 패키지는 하네스 문서 세트를 단순화 기준으로 재작성한 결과다.

반영한 주요 방향:

- 현재 상태 직접 서술로 통일
- 버전 비교, 초안 대비, 요청 경위 표현 제거
- 세 공간 실행 모델을 모든 문서에서 같은 의미로 정리
- source-of-truth와 projection 기준 유지
- public MCP tool 표면 축소
- Core 구현을 단일 로컬 제어면과 SQLite runtime 중심으로 단순화
- TASK 템플릿을 compact 기본형과 detailed 확장형으로 분리
- connector 문서의 반복 절차를 공통 계약으로 통합
- MVP와 later 기능을 명확히 분리
- approval, assurance, manual QA, acceptance 분리 유지
- detached verification, vertical slice, TDD trace, domain language, module/interface review 원칙 유지
