/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow
 * @format
 * @oncall relay
 */

'use strict';

const isPerformanceNowAvailable =
  // $FlowFixMe[cannot-resolve-name]
  typeof window !== 'undefined' &&
  // $FlowFixMe[cannot-resolve-name]
  typeof window?.performance?.now === 'function';

function currentTimestamp(): number {
  if (isPerformanceNowAvailable) {
    // $FlowFixMe[cannot-resolve-name]
    return window.performance.now();
  }
  return Date.now();
}

function withStartAndDuration<T>(cb: () => T): [number, number, T] {
  const startTime = currentTimestamp();
  const result = cb();
  return [startTime, currentTimestamp() - startTime, result];
}

module.exports = withStartAndDuration;
