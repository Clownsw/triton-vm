# Operational Stack Table

The [operational stack](data-structures.md#operational-stack)[^abbrev] is where the program stores simple elementary operations, function arguments, and pointers to important objects.
There are 16 registers (`st0` through `st15`) that the program can access directly.
These registers correspond to the top of the stack.
They are recorded in the [Processor Table](processor-table.md).

The rest of the op stack is stored in a dedicated memory object called “operational stack underflow memory”.
It is initially empty.
The evolution of the underflow memory is recorded in the Op Stack Table.
The sole task of the Op Stack Table is to keep underflow memory immutable.
To achieve this, any read or write accesses to the underflow memory are recorded in the Op Stack Table.
Read and write accesses to op stack underflow memory are a side effect of shrinking or growing the op stack.

## Base Columns

The Op Stack Table consists of 4 columns:
1. the cycle counter `clk`
1. the shrink stack indicator `shrink_stack`, corresponding to the processor's instruction bit 1 `ib1`,
1. the op stack pointer `stack_pointer`, and
1. the first underflow element `first_underflow_element`.

| Clock | Shrink Stack Indicator | Stack Pointer | First Underflow Element |
|:------|:-----------------------|:--------------|:------------------------|
| -     | -                      | -             | -                       |

Column `clk` records the processor's execution cycle during which the read or write access happens.
The `shrink_stack` indicator signals whether the underflow memory access is a read or a write:
a read corresponds to a shrinking stack is indicated by a 1, a write corresponds to a growing stack and is indicated by a 0.
The same column doubles up as a [padding indicator](#padding), in which case `shrink_stack` is set to 2.
The `stack_pointer` is the address at which the to-be-read-or-written element resides.
Finally, the `first_underflow_element` is the stack element being transferred from stack register `st15` into underflow memory on a write, or the other way around on a read.

A [subset Permutation Argument](permutation-argument.md) with the [Processor Table](processor-table.md) establishes that the rows recorded in the Op Stack Table are consistent with the processor's part of the op stack.

In order to guarantee [memory consistency](memory-consistency.md), the rows of the operational stack table are sorted by `stack_pointer` first, cycle count `clk` second.
The mechanics are best illustrated by an example.
Observe how the malicious manipulation of the Op Stack Underflow Memory, the change of “42” into “99” in cycle 8, is detected:
The transition constraints of the Op Stack Table stipulate that if `stack_pointer` does not change, then the `first_underflow_element` can only change if the next instruction grows the stack.
Consequently, row `[4, 0, 8, 42]` being followed by row `[10, 1, 8, 99]` violates the constraints.
The shrink stack indicator being correct is guaranteed by the Permutation Argument between Op Stack Table and the [Processor Table](processor-table.md).

For illustrative purposes only, we use four stack registers `st0` through `st3` in the example.
Triton VM has 16 stack registers, `st0` through `st15`.
Furthermore, implied next instructions usually recorded in register “next instruction or argument” `nia` are omitted for reasons of readability.

Processor's execution trace:

| `clk` | `ci` | `nia` | `st0` | `st1` | `st2` | `st3` | Op Stack Underflow Memory | `op_stack_pointer` |
|------:|:-----|------:|------:|------:|------:|------:|:--------------------------|-------------------:|
|     0 | push |    42 |     0 |     0 |     0 |     0 | []                        |                  4 |
|     1 | push |    43 |    42 |     0 |     0 |     0 | [ 0]                      |                  5 |
|     2 | push |    44 |    43 |    42 |     0 |     0 | [ 0,  0]                  |                  6 |
|     3 | push |    45 |    44 |    43 |    42 |     0 | [ 0,  0,  0]              |                  7 |
|     4 | push |    46 |    45 |    44 |    43 |    42 | [ 0,  0,  0,  0]          |                  8 |
|     5 | push |    47 |    46 |    45 |    44 |    43 | [42,  0,  0,  0,  0]      |                  9 |
|     6 | push |    48 |    47 |    46 |    45 |    44 | [43, 42,  0,  0,  0,  0]  |                 10 |
|     7 | nop  |       |    48 |    47 |    46 |    45 | [44, 43, 42,  0,  0,  0]  |                 11 |
|     8 | pop  |       |    48 |    47 |    46 |    45 | [44, 43, 99,  0,  0,  0]  |                 11 |
|     9 | pop  |       |    47 |    46 |    45 |    44 | [43, 99,  0,  0,  0,  0]  |                 10 |
|    10 | pop  |       |    46 |    45 |    44 |    43 | [99,  0,  0,  0,  0]      |                  9 |
|    11 | pop  |       |    45 |    44 |    43 |    99 | [ 0,  0,  0,  0]          |                  8 |
|    12 | push |    77 |    44 |    43 |    99 |     0 | [ 0,  0,  0]              |                  7 |
|    13 | swap |     3 |    77 |    44 |    43 |    99 | [ 0,  0,  0,  0]          |                  8 |
|    14 | push |    78 |    99 |    44 |    43 |    77 | [ 0,  0,  0,  0]          |                  8 |
|    15 | swap |     3 |    78 |    99 |    44 |    43 | [77,  0,  0,  0,  0]      |                  9 |
|    16 | push |    79 |    43 |    99 |    44 |    78 | [77,  0,  0,  0,  0]      |                  9 |
|    17 | pop  |       |    79 |    43 |    99 |    44 | [78, 77,  0,  0,  0,  0]  |                 10 |
|    18 | pop  |       |    43 |    99 |    44 |    78 | [77,  0,  0,  0,  0]      |                  9 |
|    19 | pop  |       |    99 |    44 |    78 |    77 | [ 0,  0,  0,  0]          |                  8 |
|    20 | pop  |       |    44 |    78 |    77 |     0 | [ 0,  0,  0]              |                  7 |
|    21 | pop  |       |    78 |    77 |     0 |     0 | [ 0,  0]                  |                  6 |
|    22 | pop  |       |    77 |     0 |     0 |     0 | [ 0]                      |                  5 |
|    23 | halt |       |     0 |     0 |     0 |     0 | []                        |                  4 |


Operational Stack Table:

| `clk` | `shrink_stack` | `stack_pointer` | `first_underflow_element` |
|------:|---------------:|----------------:|--------------------------:|
|     0 |              0 |               4 |                         0 |
|    22 |              1 |               4 |                         0 |
|     1 |              0 |               5 |                         0 |
|    21 |              1 |               5 |                         0 |
|     2 |              0 |               6 |                         0 |
|    20 |              1 |               6 |                         0 |
|     3 |              0 |               7 |                         0 |
|    11 |              1 |               7 |                         0 |
|    12 |              0 |               7 |                         0 |
|    19 |              1 |               7 |                         0 |
|     4 |              0 |               8 |                        42 |
|    10 |              1 |               8 |                        99 |
|    14 |              0 |               8 |                        77 |
|    18 |              1 |               8 |                        77 |
|     5 |              0 |               9 |                        43 |
|     9 |              1 |               9 |                        43 |
|    16 |              0 |               9 |                        78 |
|    17 |              1 |               9 |                        78 |
|     6 |              0 |              10 |                        44 |
|     8 |              1 |              10 |                        44 |


## Extension Columns

The Op Stack Table has 2 extension columns, `rppa` and `ClockJumpDifferenceLookupClientLogDerivative`.

1. A Permutation Argument establishes that the rows of the Op Stack Table correspond to the rows of the [Processor Table](processor-table.md).
  The running product for this argument is contained in the `rppa` column.
1. In order to achieve [memory consistency](memory-consistency.md), a [Lookup Argument](lookup-argument.md) shows that all clock jump differences are contained in the `clk` column of the [Processor Table](processor-table.md).
  The logarithmic derivative for this argument is contained in the `ClockJumpDifferenceLookupClientLogDerivative` column.

## Padding

The last row in the Op Stack Table is taken as the padding template row.
Should the Op Stack Table be empty, the row (`clk`, `shrink_stack`, `stack_pointer`, `first_underflow_element`) = (0, 0, 16, 0) is used instead.
In the template row, the `shrink_stack` indicator is set to 2, signifying padding.
The template row is inserted below the last row until the desired padded height is reached.

## Memory-Consistency

Memory-consistency follows from two more primitive properties:

1. Contiguity of regions of constant memory pointer.
  Since the memory pointer for the Op Stack table, `osp` can change by at most one per cycle, it is possible to enforce a full sorting using AIR constraints.
2. Correct inner-sorting within contiguous regions.
  Specifically, the rows within each contiguous region of constant memory pointer should be sorted for clock cycle.
  This property is established by the clock jump difference [Lookup Argument](lookup-argument.md).
  In a nutshell, every difference of consecutive clock cycles that occurs within one contiguous block of constant memory pointer is shown itself to be a valid clock cycle through a separate cross-table argument.

# Arithmetic Intermediate Representation

Let all household items (🪥, 🛁, etc.) be challenges, concretely evaluation points, supplied by the verifier.
Let all fruit & vegetables (🥝, 🥥, etc.) be challenges, concretely weights to compress rows, supplied by the verifier.
Both types of challenges are X-field elements, _i.e._, elements of $\mathbb{F}_{p^3}$.

## Initial Constraints

1. `clk` is 0
1. `osv` is 0.
1. `osp` is the number of available stack registers, _i.e._, 16.
1. The running product for the permutation argument with the Processor Table `rppa` starts off having accumulated the first row with respect to challenges 🍋, 🍊, 🍉, and 🫒 and indeterminate 🪤.
1. The logarithmic derivative for the clock jump difference lookup `ClockJumpDifferenceLookupClientLogDerivative` is 0.

### Initial Constraints as Polynomials

1. `clk`
1. `osv`
1. `osp - 16`
1. `rppa - (🪤 - 🍋·clk - 🍊·ib1 - 🍉·osp - 🫒osv)`
1. `ClockJumpDifferenceLookupClientLogDerivative`

## Consistency Constraints

None.

## Transition Constraints

1.
  - the `osp` increases by 1, *or*
  - the `osp` does not change AND the `osv` does not change, *or*
  - the `osp` does not change AND the shrink stack indicator `shrink_stack` is 1.
1. The running product for the permutation argument with the Processor Table `rppa` absorbs the next row with respect to challenges 🍋, 🍊, 🍉, and 🫒 and indeterminate 🪤.
1. If the op stack pointer `osp` does not change, then the logarithmic derivative for the clock jump difference lookup `ClockJumpDifferenceLookupClientLogDerivative` accumulates a factor `(clk' - clk)` relative to indeterminate 🪞.
  Otherwise, it remains the same.

Written as Disjunctive Normal Form, the same constraints can be expressed as:

1.
  - the `osp` increases by 1 or the `osp` does not change
  - the `osp` increases by 1 or the `osv` does not change or the shrink stack indicator `shrink_stack` is 1
1. `rppa' = rppa·(🪤 - 🍋·clk' - 🍊·ib1' - 🍉·osp' - 🫒osv')`
1. - the `osp` changes or the logarithmic derivative accumulates a summand, and
   - the `osp` does not change or the logarithmic derivative does not change.

### Transition Constraints as Polynomials

1. `(osp' - (osp + 1))·(osp' - osp)`
1. `(osp' - (osp + 1))·(osv' - osv)·(1 - shrink_stack)`
1. `rppa' - rppa·(🪤 - 🍋·clk' - 🍊·ib1' - 🍉·osp' - 🫒osv')`
1. `(osp' - (osp + 1))·((ClockJumpDifferenceLookupClientLogDerivative' - ClockJumpDifferenceLookupClientLogDerivative) · (🪞 - clk' + clk) - 1)`<br />
   `+ (osp' - osp)·(ClockJumpDifferenceLookupClientLogDerivative' - ClockJumpDifferenceLookupClientLogDerivative)`

## Terminal Constraints

None.

---

[^abbrev]: frequently abbreviated as “Op Stack”
