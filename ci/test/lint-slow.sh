#!/usr/bin/env bash

# Copyright 2019 Materialize, Inc. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.
#
# lint-slow.sh — slow linters that require building code.

set -euo pipefail

. misc/shlib/shlib.bash

ci_init

ci_try cargo test --locked --doc
# Intentionally run check last, since otherwise it won't use the cache.
# https://github.com/rust-lang/rust-clippy/issues/3840
ci_try bin/check

ci_status_report
