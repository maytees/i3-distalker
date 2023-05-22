#!/bin/bash
 if which i3distalker >/dev/null 2>&1; then
  echo "i3distalker binary found. Running Discord..."
  i3distalker &
else
  echo "i3distalker binary not found. Please install Discord and try again."
fi

# Check if 'discord' binary is available
if which discord >/dev/null 2>&1; then
  echo "Discord binary found. Running Discord..."
  discord &
else
  echo "Discord binary not found. Please install Discord and try again."
fi
