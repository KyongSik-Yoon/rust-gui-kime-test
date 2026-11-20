# rust-gui-kime-test

Linux Wayland(Hyprland) + `kime` 한글 입력기 환경에서 Rust 네이티브 GUI 프레임워크(`iced`, `gpui`)의 한글 조합·커밋·편집 호환성을 직접 검증하기 위한 최소 샘플 모음입니다.

## 구성

- `iced-kime-test/`: `iced 0.14` (`text_input` + `text_editor` + 실시간 IME/Key 이벤트 로그)
- `gpui-kime-test/`: `gpui-pre 0.3.13` (`EntityInputHandler` 기반 입력창 + 실시간 IME/Keystroke 로그)

## 실행 방법

로컬 물리 키보드 환경에서 각각 실행해 한글 입력을 테스트합니다.

### 1) iced 0.14

```bash
cd iced-kime-test
cargo run
```

### 2) gpui (Zed 최신 스냅샷 `gpui-pre 0.3.13`)

```bash
cd gpui-kime-test
cargo run
```

XWayland(`kime-xim`) 모드로 비교하고 싶을 때는 `WAYLAND_DISPLAY`를 비워 실행할 수 있습니다.

```bash
WAYLAND_DISPLAY= cargo run
```

## 테스트 체크리스트

1. 영문 입력 (`hello`) → 한영 전환 → 한글 입력 (`안녕하세요`)
2. 조합 중 백스페이스(초성·중성·종성 단계별 삭제) 및 완료 글자 삭제
3. 한글 입력 중 방향키 이동·마우스 클릭으로 커서 이동 시 마지막 조합 글자 유실/중복 여부
4. Enter 제출(`iced` 한 줄 입력창) 및 여러 줄 줄바꿈(`iced` `text_editor`)
5. 한영 전환 직후 첫 글자 씹힘 여부

## 참고 (프로토콜 로그 확인)

문제가 재현되면 아래 명령으로 `zwp_text_input_v3` 패킷을 확인할 수 있습니다.

```bash
WAYLAND_DEBUG=1 cargo run 2>&1 | rg "zwp_text_input_v3"
```
