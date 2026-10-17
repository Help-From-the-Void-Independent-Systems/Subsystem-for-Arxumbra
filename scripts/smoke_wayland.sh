set -u

APP_PATH="$1"
ASSETS_SRC="$2"
EVIDENCE_DIR="$3"
EVIDENCE_DIR="$(cd "$EVIDENCE_DIR" && pwd)"
STATUS_FILE="$EVIDENCE_DIR/smoke-status.tsv"
LOG_FILE="$EVIDENCE_DIR/smoke.log"
APP_LOG="$EVIDENCE_DIR/app.log"
WESTON_LOG="$EVIDENCE_DIR/weston.log"
WAYLAND_INFO_LOG="$EVIDENCE_DIR/wayland-info.log"
SOCKET_NAME="arxumbra-smoke"
HOLD_SECS="7.0"
APP_PID=""
WESTON_PID=""

note() {
  printf '%s\n' "$1" | tee -a "$LOG_FILE"
}

cleanup() {
  trap - EXIT INT TERM
  if [ -n "$APP_PID" ]; then
    kill "$APP_PID" 2>/dev/null
    wait "$APP_PID" 2>/dev/null
  fi
  if [ -n "$WESTON_PID" ]; then
    kill "$WESTON_PID" 2>/dev/null
    wait "$WESTON_PID" 2>/dev/null
  fi
  [ -n "${SMOKE_ROOT:-}" ] && rm -rf "$SMOKE_ROOT"
}

trap cleanup EXIT INT TERM

: > "$STATUS_FILE"
: > "$LOG_FILE"

command -v weston >/dev/null 2>&1 || { record compositor "FAIL weston not installed"; exit 1; }
record weston-version "$(weston --version 2>&1)"

for dependency in libwayland-client.so libxkbcommon.so; do
  if ldconfig -p | grep -q "$dependency"; then
    printf '%s\t%s\n' "dep-$dependency" present >> "$STATUS_FILE"
  else
    printf '%s\t%s\n' "dep-$dependency" FAIL >> "$STATUS_FILE"
  fi
done

VULKAN_STATUS="unavailable"
if command -v vulkaninfo >/dev/null 2>&1; then
  vulkaninfo --summary >/dev/null 2>&1 && VULKAN_STATUS="present" || VULKAN_STATUS="no-device"
else
  VULKAN_STATUS="vulkaninfo-missing"
fi
record vulkan "$VULKAN_STATUS"

if [ "$VULKAN_STATUS" != "present" ]; then
  ICD_DIR="/usr/share/vulkan/icd.d"
  if [ -d "$ICD_DIR" ]; then
    for icd in "$ICD_DIR"/*lvp*.json "$ICD_DIR"/*sw*.json; do
      if [ -f "$icd" ]; then
        export VK_DRIVER_FILES="$icd"
        export VK_ICD_FILENAMES="$icd"
        record vulkan-icd "$icd"
        break
      fi
    done
  fi
fi

export XDG_RUNTIME_DIR
if command -v systemd-run >/dev/null 2>&1 && [ -n "${RUNNER_TEMP:-}" ]; then
  SMOKE_ROOT="$RUNNER_TEMP/arxumbra-smoke-$$"
else
  SMOKE_ROOT="$(mktemp -d)"
fi
mkdir -p "$SMOKE_ROOT/xdg"
chmod 700 "$SMOKE_ROOT" "$SMOKE_ROOT/xdg"
export XDG_RUNTIME_DIR="$SMOKE_ROOT/xdg"
record runtime-dir "$XDG_RUNTIME_DIR"

WESTON_BACKEND=""
if weston --help 2>&1 | grep -q 'headless-backend.so'; then
  WESTON_BACKEND="headless-backend.so"
elif weston --help 2>&1 | grep -q 'headless'; then
  WESTON_BACKEND="headless"
fi
[ -n "$WESTON_BACKEND" ] || { record compositor "FAIL no headless backend advertised"; exit 1; }
record compositor-backend "$WESTON_BACKEND"

WESTON_FLAGS=(--backend="$WESTON_BACKEND" --socket="$SOCKET_NAME" --width=1280 --height=720 --idle-time=0 --log="$WESTON_LOG")
if weston --help 2>&1 | grep -q -- '--no-xwayland'; then
  WESTON_FLAGS+=(--no-xwayland)
  record xwayland-flag "--no-xwayland supported and used"
else
  record xwayland-flag "unsupported by this weston build"
fi

unset DISPLAY
DISPLAY_WAS_SET="${DISPLAY-}"
record display-unset "$([ -z "$DISPLAY_WAS_SET" ] && echo yes || echo "FAIL was $DISPLAY_WAS_SET")"
export WAYLAND_DISPLAY="$SOCKET_NAME"

weston "${WESTON_FLAGS[@]}" &
WESTON_PID=$!

SOCKET_READY="no"
for attempt in $(seq 1 60); do
  if [ -S "$XDG_RUNTIME_DIR/$SOCKET_NAME" ]; then
    if command -v wayland-info >/dev/null 2>&1; then
      if WAYLAND_DISPLAY="$SOCKET_NAME" XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" wayland-info > "$WAYLAND_INFO_LOG" 2>&1; then
        SOCKET_READY="yes"
        break
      fi
    else
      SOCKET_READY="socket-only"
      break
    fi
  fi
  if ! kill -0 "$WESTON_PID" 2>/dev/null; then
    note "compositor exited during readiness wait"
    break
  fi
  sleep 1
done
record compositor-ready "$SOCKET_READY"

if grep -qi 'xwayland' "$WESTON_LOG" 2>/dev/null; then
  record xwayland-audit "review: $(grep -i 'xwayland' "$WESTON_LOG" | head -2 | tr '\n' ' ')"
else
  record xwayland-audit "no xwayland activity in compositor log"
fi

if [ "$SOCKET_READY" = "no" ]; then
  note "compositor never became ready"
  record compositor "FAIL not ready"
  exit 1
fi
record compositor "PASS"

INSTALL_DIR="$SMOKE_ROOT/install"
mkdir -p "$INSTALL_DIR"
cp "$APP_PATH" "$INSTALL_DIR/$(basename "$APP_PATH")"
chmod 755 "$INSTALL_DIR/$(basename "$APP_PATH")"
cp -r "$ASSETS_SRC" "$INSTALL_DIR/assets"
record installed-to "$INSTALL_DIR"

APP_BIN="$INSTALL_DIR/$(basename "$APP_PATH")"
(
  cd "$INSTALL_DIR" || exit 1
  env -u DISPLAY WAYLAND_DISPLAY="$SOCKET_NAME" XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" RUST_LOG=info "$APP_BIN" > "$APP_LOG" 2>&1 &
  echo $! > "$SMOKE_ROOT/app.pid"
)
APP_PID="$(cat "$SMOKE_ROOT/app.pid")"
record app-pid "$APP_PID"

APP_READY="no"
for attempt in $(seq 1 15); do
  if ! kill -0 "$APP_PID" 2>/dev/null; then
    note "application exited before readiness"
    break
  fi
  sleep 1
done
if kill -0 "$APP_PID" 2>/dev/null; then
  APP_READY="yes"
fi
record app-ready "$APP_READY"

SHOOTER="none"
if command -v weston-screenshooter >/dev/null 2>&1; then
  SHOOTER="weston-screenshooter"
fi
record screenshot-tool "$SHOOTER"

capture() {
  if [ "$SHOOTER" = "none" ]; then
    record "shot-$1" "NOT RUN no capture tool"
    return 1
  fi
  if WAYLAND_DISPLAY="$SOCKET_NAME" XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" weston-screenshooter "$EVIDENCE_DIR/$1.png" >/dev/null 2>&1 && head -c 8 "$EVIDENCE_DIR/$1.png" | od -An -tx1 | tr -d ' \n' | grep -q '89504e470d0a1a0a'; then
    record "shot-$1" "PASS"
    return 0
  fi
  record "shot-$1" "FAIL capture rejected"
  return 1
}

WTYPE="none"
if command -v wtype >/dev/null 2>&1; then
  WTYPE="wtype"
fi
if [ "$WTYPE" = "none" ]; then
  record keyboard-tool "NOT RUN wtype unavailable"
else
  if grep -q 'zwp_virtual_keyboard_manager_v1' "$WAYLAND_INFO_LOG" 2>/dev/null; then
    record keyboard-tool "wtype plus virtual-keyboard global"
  else
    record keyboard-tool "wtype present; virtual-keyboard global unverified"
  fi
fi

tap() {
  if [ "$WTYPE" = "none" ]; then
    record "key-$1" "NOT RUN"
    return 1
  fi
  if WAYLAND_DISPLAY="$SOCKET_NAME" XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" wtype -k "$1" >> "$LOG_FILE" 2>&1; then
    record "key-$1" "delivered"
    return 0
  fi
  record "key-$1" "FAIL rejected"
  return 1
}

capture home
tap Tab
sleep 1
capture focus-home
tap Tab
sleep 1
capture focus-registry
tap Return
sleep 1
capture registry
tap Tab
sleep 1
capture focus-after-open
tap Return
sleep 1
capture home-again
tap F1
sleep 1
capture hud-hidden
tap F1
sleep 1
capture hud-restored
tap Escape
sleep 1
if kill -0 "$APP_PID" 2>/dev/null; then
  record escape-tap-no-exit "PASS held exit threshold not reached"
else
  record escape-tap-no-exit "FAIL application exited on short tap"
fi

if kill -0 "$APP_PID" 2>/dev/null; then
  note "sending SIGTERM after interaction phases; escape-hold exit path is not exercisable without a hold-capable injection tool and stays NOT RUN"
  kill -TERM "$APP_PID" 2>/dev/null
  for attempt in $(seq 1 10); do
    kill -0 "$APP_PID" 2>/dev/null || break
    sleep 1
  done
fi

if kill -0 "$APP_PID" 2>/dev/null; then
  record termination "FAIL application unresponsive to SIGTERM"
  record smoke "FAIL"
  exit 1
fi
record termination "process ended after interactions"
record app-log-size "$(wc -c < "$APP_LOG")"

if [ "$APP_READY" != "yes" ]; then
  record smoke "FAIL application not ready"
  exit 1
fi

record smoke "PASS"
exit 0
