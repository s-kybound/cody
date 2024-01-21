#!/bin/zsh

if [ $# -eq 0 ]; then
  echo "Usage: $0 <argument>"
  exit 1
fi

argument="$1"

# Command 1
echo "Running Cody compiler with file: $argument"
# Replace the following line with your actual command
# Example: command1 "$argument"
cargo run -- -i "$argument" -o "$argument".cody

# Command 2
echo "Compiling with llc..."
# Replace the following line with your actual command
# Example: command2 "$argument"
llc "$argument".cody
lli "$argument".cody

echo $?
