#!/usr/bin/env zsh

clean_text() {
  local merged_text=""

  # combine word continuations
  while read -r line; do
    if [[ $line =~ -$ ]]; then
      merged_text+="${line%-}"
    else
      merged_text+="$line\n"
    fi
  done <<< "$1"

  # strip out anything that isn't a letter, number, or whitespace
  cleaned_text=$(echo -e "$merged_text" | tr -cd '[:alnum:]\n\r[:space:]')
  echo "$cleaned_text"
}

_a=$(cat "$1")
_b=$(cat "$2")
_a=$(clean_text "$_a")
_b=$(clean_text "$_b")

echo "$_a" > "a.tmp"
echo "$_b" > "b.tmp"

wdiff "a.tmp" "b.tmp" \
  --start-delete $'\033[30;41m' --end-delete $'\033[0m' \
  --start-insert $'\033[30;42m' --end-insert $'\033[0m' \
  --ignore-case

rm "a.tmp"
rm "b.tmp"