package main

func main() {
    /* @[Application]
    Uses hard-coded values for demonstrating @[Domain/Accumulator/Invariants] in action.

    _Primary uses:_
    - Demo Posterity features
    - Testing
      - Including **Markdown** rendering
    */
    acc := Accumulator{ID: "abc", Value: 0} //@[Domain/Accumulator/Invariants]
    _ = acc.Collect(5)
    println(acc.Value)
}