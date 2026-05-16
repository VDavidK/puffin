# Sizes

Sizes used by the `vbox` and `hbox` components follow a specific string format. This
format is `Type:Arg[:Arg]` where one type and one argument is necessary, and an optional
second argument (denoted in the square brackets) can be used in certain cases.

Each different type has its own effect on how the component scales. A list of valid types is listed below.

If a size is necessary but is not provided, the default size is usually defined as `Percentage:100`

## Valid sizes

* `Length:X` A literal size which does not scale where X is the amount of columns/rows the component will span.
* `Fill:X` Takes up as much space as possible weighed by the X value. Meaning if other components also use `Fill`, then the X value will serve as a ratio between the other components X values.
* `Min:X` Ensures the component will never shrink below X columns/rows.
* `Max:X` Ensures the component will never grow larger than X columns/rows.
* `Percentage:X` Takes up a percentage of the parent's available space, where X indicates the percentage (i.e. 50 for 50%).
* `Ratio:X:Y` Similar to `Percentage` but is calculated through a ratio instead, where the percentage is X / Y.
