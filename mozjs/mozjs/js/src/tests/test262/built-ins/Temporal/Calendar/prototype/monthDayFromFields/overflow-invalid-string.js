// |reftest| skip -- Temporal is not supported
// Copyright (C) 2021 Igalia, S.L. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-temporal.calendar.prototype.monthdayfromfields
description: RangeError thrown when overflow option not one of the allowed string values
info: |
    sec-getoption step 10:
      10. If _values_ is not *undefined* and _values_ does not contain an element equal to _value_, throw a *RangeError* exception.
    sec-temporal-totemporaloverflow step 1:
      1. Return ? GetOption(_normalizedOptions_, *"overflow"*, « String », « *"constrain"*, *"reject"* », *"constrain"*).
    sec-temporal-isomonthdayfromfields step 2:
      2. Let _overflow_ be ? ToTemporalOverflow(_options_).
    sec-temporal.calendar.prototype.monthdayfromfields step 6:
      6. Let _result_ be ? ISOMonthDayFromFields(_fields_, _options_).
features: [Temporal]
---*/

const calendar = new Temporal.Calendar("iso8601");
const badOverflows = ["", "CONSTRAIN", "balance", "other string", "constra\u0131n", "reject\0"];
for (const overflow of badOverflows) {
  assert.throws(
    RangeError,
    () => calendar.monthDayFromFields({ year: 2000, month: 5, day: 2 }, { overflow }),
    `invalid overflow ("${overflow}")`
  );
}

reportCompare(0, 0);