package main

//@[Domain/Accumulator] Collects stuff
type Accumulator struct {
    ID string
    Value int32
}

func (a *Accumulator) Collect(x int32) error {
    /*@[Domain/Accumulator/Invariants]{alias:Domain rules}
    The accumulated value is always increasing when collecting new values.
    */
    // Normal comment
    if x <= 0 {
        return errors.New("Negative value")
    }

    a.Value += x

    return nil
}

func (a *Accumulator) Reset() {
    /* @[Domain/Accumulator/Invariants]
    Accumulator can be reset to a starting point (0) explicitly.
    */
    a.Value = 0
}

func main() {
    acc := Accumulator{ID: "abc", Value: 0}
    _ = acc.Collect(5)
    println(acc.Value)
}