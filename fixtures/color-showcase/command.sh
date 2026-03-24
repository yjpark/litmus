#!/usr/bin/env bash
# Simulated CI/deploy dashboard that naturally uses all 16 ANSI colors.
# Exercises: all standard and bright ANSI colors in a realistic context.

# Header
printf '\e[1;97m  Deploy Pipeline\e[0m \e[90m— production release v2.4.1\e[0m\n'
printf '\e[90m  ─────────────────────────────────────────────────────\e[0m\n'
# Build stage
printf '  \e[1;37m▸ Build\e[0m\n'
printf '    \e[32m✓\e[0m Compile \e[90m.........................\e[0m \e[32m12.4s\e[0m\n'
printf '    \e[32m✓\e[0m Lint    \e[90m.........................\e[0m \e[32m 3.1s\e[0m\n'
printf '    \e[33m⚠\e[0m Tests   \e[90m.........................\e[0m \e[33m45.7s\e[0m \e[33m2 flaky\e[0m\n'
# Deploy stages
printf '  \e[1;37m▸ Deploy\e[0m\n'
printf '    \e[32m✓\e[0m \e[36mstaging\e[0m  \e[90m......................\e[0m \e[32mdone\e[0m\n'
printf '    \e[32m✓\e[0m \e[34meu-west\e[0m  \e[90m......................\e[0m \e[32mdone\e[0m\n'
printf '    \e[91m✗\e[0m \e[34mus-east\e[0m  \e[90m......................\e[0m \e[91mfailed\e[0m\n'
printf '    \e[33m◌\e[0m \e[34map-south\e[0m \e[90m......................\e[0m \e[90mskipped\e[0m\n'
# Service health
printf '  \e[1;37m▸ Health Check\e[0m\n'
printf '    \e[42;30m PASS \e[0m \e[37mapi-gateway\e[0m    \e[90mlatency\e[0m \e[92m23ms\e[0m\n'
printf '    \e[42;30m PASS \e[0m \e[37mauth-service\e[0m   \e[90mlatency\e[0m \e[92m45ms\e[0m\n'
printf '    \e[41;97m FAIL \e[0m \e[37mdata-pipeline\e[0m  \e[90mlatency\e[0m \e[91m2340ms\e[0m\n'
printf '    \e[43;30m WARN \e[0m \e[37mcache-layer\e[0m    \e[90mhit rate\e[0m \e[93m67%%\e[0m\n'
# Error details
printf '  \e[1;91m✗ us-east error:\e[0m \e[31mConnection timeout\e[0m to \e[35m10.0.3.42:5432\e[0m\n'
printf '    \e[36mRetry\e[0m \e[93m3/3\e[0m failed — see \e[4;94mhttps://status.internal/4821\e[0m\n'
printf '\e[90m  ─────────────────────────────────────────────────────\e[0m\n'
printf '  \e[1;97mSummary:\e[0m \e[92m6 passed\e[0m  \e[91m1 failed\e[0m  \e[93m1 warning\e[0m  \e[90m1 skipped\e[0m\n'
