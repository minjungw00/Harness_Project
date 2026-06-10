# 런타임 경계 참조

이 문서는 Product Repository, Harness Server, Harness Runtime Home 사이의 경계 모델을 담당합니다. 문서 원천 자료일 뿐이며 이 저장소에는 Harness Server/runtime 구현, Runtime Home, 생성된 상태 보기 시스템, 적합성 실행기, 런타임 데이터가 없습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- Product Repository / Harness Server / Harness Runtime Home 분리
- 제품 파일, 하네스 기록, 렌더링된 표시 사이의 변경 권한 경계
- 비격리와 OS 샌드박싱 비보장
- 생성된 표시와 Product Repository 텍스트가 하네스 권한을 만들지 않는다는 규칙

이 문서는 담당하지 않습니다.

- 저장소 기록, 효과, 아티팩트, 버전 관리, 잠금, 마이그레이션: [참조 색인](README.md)의 저장소 담당 문서
- 공개 API 스키마나 메서드 동작: [참조 색인](README.md)의 API 담당 문서
- 보안 보장 의미: [보안](security.md)
- 상태 보기 권한: [Projection과 템플릿](projection-and-templates.md)

## Product Repository

Product Repository는 사용자의 프로젝트 작업 공간입니다. 제품 파일은 향후 하네스 확인의 입력이 될 수 있지만, Product Repository 내용은 하네스 상태도 아니고 Runtime Home도 아니며 하네스 권한 증거도 아닙니다.

## Harness Server

향후 Harness Server는 하네스 기록과 API 동작을 중재할 수 있습니다. 이 저장소에는 아직 구현되어 있지 않습니다. 문서 편집은 서버 코드를 만들지 않고 제품/런타임 쓰기를 승인하지 않습니다.

## Harness Runtime Home

Harness Runtime Home은 향후 사용자별 또는 설치별 운영 데이터 공간입니다. 정확한 저장소 기록, 경로, 아티팩트, 잠금, 마이그레이션, 버전 관리는 분리된 저장소 담당 문서가 맡습니다. 이 문서 저장소는 Runtime Home이 아닙니다.

## 변경 권한 경계

Core가 소유한 하네스 기록은 담당 문서가 정한 경로로만 바뀔 수 있습니다. Product Repository 편집, 생성된 Markdown, 렌더링된 상태 보기, 대화 텍스트, 커넥터 설명, 에이전트 기억은 하네스 기록을 직접 바꾸지 않습니다.

아티팩트 본문 읽기와 아티팩트 승격에는 API, 저장소, 로컬 접점, 보안 담당 문서가 정한 경계가 모두 맞아야 합니다. 복사된 `surface_id`, 표시된 `ArtifactRef`, 렌더링된 상태 보기는 그 자체로 권한이 아닙니다.

## 비격리 경계

현재 문서는 OS 수준 권한 제어, 임의 도구 샌드박스, 변조 방지 저장소, 기본 도구 실행 전 차단, 보안 격리, 권한 격리를 주장하지 않습니다. 그런 주장은 승격된 담당 문서가 문서화한 메커니즘과 증명 경로가 있어야 합니다.
