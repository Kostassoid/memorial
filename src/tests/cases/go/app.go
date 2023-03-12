package main

func main() {
    /* @[Application]
    Uses hard-coded values for demonstrating @[Domain/Accumulator/Invariants] in action.
    */
    acc := Accumulator{ID: "abc", Value: 0} //@[Domain/Accumulator/Invariants]
    _ = acc.Collect(5)
    println(acc.Value)
}