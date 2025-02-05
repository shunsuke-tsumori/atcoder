#!/bin/zsh

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <contest_id>"
  exit 1
fi

CONTEST_ID="$1"

sed '/^TEMPLATE$/{
  r template.rs
  d
}' compete_template.toml > compete.toml

touch -r compete_template.toml compete.toml

cargo compete new "$CONTEST_ID"

TARGET_DIR="./${CONTEST_ID}"
if [ -d "$TARGET_DIR" ]; then
  mkdir -p "$TARGET_DIR/.cargo"
  cp -p .cargo/config.toml "$TARGET_DIR/.cargo/config.toml"
  echo "Success: Copied .cargo/config.toml to ${TARGET_DIR} ."
  cp -p test.sh "$TARGET_DIR"
  cp -p submit.sh "$TARGET_DIR"
  cp -p open.sh "$TARGET_DIR"
  echo "Success: Copied open.sh, test.sh and submit.sh to ${TARGET_DIR} ."
else
  echo "Error: No ${TARGET_DIR} dir."
  exit 1
fi
