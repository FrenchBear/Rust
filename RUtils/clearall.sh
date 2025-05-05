#!/bin/bash
find . -type d -name target -print -exec rm -rf {} \; 2>/dev/null
