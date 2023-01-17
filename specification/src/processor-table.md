# Processor Table

## Base Columns

The processor consists of all registers defined in the [Instruction Set Architecture](isa.md).
Each register is assigned a column in the processor table.

## Extension Colums

The Instruction Table has 12 extension columns, corresponding to Evaluation Arguments and Permutation Arguments.
Namely:
1. `RunningEvaluationStandardInput` for the Evaluation Argument with the input symbols.
1. `RunningEvaluationStandardOutput` for the Evaluation Argument with the output symbols.
1. `RunningProductInstructionTable` for the Permutation Argument with the [Instruction Table](instruction-table.md).
1. `RunningProductOpStackTable` for the Permutation Argument with the [OpStack Table](operational-stack-table.md).
1. `RunningProductRamTable` for the Permutation Argument with the [RAM Table](random-access-memory-table.md).
1. `RunningProductJumpStackTable` for the Permutation Argument with the [Jump Stack Table](jump-stack-table.md).
1. `RunningEvaluationHashInput` for the Evaluation Argument with the [Hash Table](hash-table.md) for copying the input to the hash function from the Processor to the Hash Coprocessor.
1. `RunningEvaluationHashDigest` for the Evaluation Argument with the [Hash Table](hash-table.md) for copying the hash digest from the Hash Coprocessor to the Processor.
1. `RunningEvaluationSpongeAbsorb` for the Evaluation Argument with the [Hash Table](hash-table.md) for copying the 10 next to-be-absorbed elements from the Processor to the Hash Coprocessor.
1. `RunningEvaluationSpongeSqueeze` for the Evaluation Argument with the [Hash Table](hash-table.md) for copying the 10 next squeezed elements from the Hash Coprocessor to the Processor.
1. `RunningProductU32Table` for the Permutation Argument with the [U32 Table](u32-table.md).
1. `RunningProductAllClockJumpDifferences` for the [Multi-Table Set Equality argument](memory-consistency.md#clock-jump-differences-with-multiplicities-in-the-processor-table) with the [RAM Table](random-access-memory-table.md), the [JumpStack Table](jump-stack-table.md), and the [OpStack Table](operational-stack-table.md).

Lastly, extension columns `RunningEvaluationSelectedClockCycles` and `RunningEvaluationUniqueClockJumpDifferences` help achieving [memory consistency](memory-consistency.md#unique-clock-jump-differences-in-the-processor-table).

## Padding

A padding row is a copy of the Processor Table's last row with the following modifications:
1. column `clk` is increased by 1, and
1. column `IsPadding` is set to 1.

## Memory Consistency: Inner Sorting Argument

In order to satisfy [Memory-Consistency](memory-consistency.md), the rows of memory-like tables (*i.e.*, [RAM Table](random-access-memory-table.md), [JumpStack Table](jump-stack-table.md), [OpStack Table](operational-stack-table.md)), need to be sorted in a special way.
In particular, the regions of constant memory pointer need to be contiguous;
and the rows in each such contiguous region must be sorted for clock cycle. 
The contiguity of regions is trivial for the JumpStack and OpStack Table, and for the RAM Table the [Contiguity Argument](memory-consistency.md#contiguity-for-ram-table) establishes this fact.

The [Inner Sorting Argument via Clock Jump Differences](memory-consistency.md#clock-jump-differences-and-inner-sorting) impacts the Processor Table quite substantially.
Concretely, the following 3 base columns and 3 extension columns only help achieving memory consistency.

- Base column `cjd`, the list of all clock jump differences greater than 1 in all memory-like tables.
- Base column `invm`, the list of inverses of clock jump differences, counting multiplicities. This column helps to select all nonzero `cjd`'s.
- Base column `invu`, the list of inverses of unique clock jump differences, *i.e.*, without counting multiplicities. This column helps to select the unique nonzero `cjd`'s.
- Extension column `rer`, the running evaluation of relevant clock cycles.
- Extension column `reu`, the running evaluation of unique clock cycle differences.
- Extension column `rpm`, the running product of all clock jump differences, with multiplicities.

# Arithmetic Intermediate Representation

Let all household items (🪥, 🛁, etc.) be challenges, concretely evaluation points, supplied by the verifier.
Let all fruit & vegetables (🥝, 🥥, etc.) be challenges, concretely weights to compress rows, supplied by the verifier.
Both types of challenges are X-field elements, _i.e._, elements of $\mathbb{F}_{p^3}$.

Note, that the transition constraint's use of `some_column` vs `some_column_next` might be a little unintuitive.
For example, take the following part of some execution trace.

| Clock Cycle | Current Instruction |  st0 |  …  | st15 | Running Evaluation “To Hash Table”  | Running Evaluation “From Hash Table”    |
|:------------|:--------------------|-----:|:---:|-----:|:------------------------------------|:----------------------------------------|
| $i-1$       | `foo`               |   17 |  …  |   22 | $a$                                 | $b$                                     |
| $i$         | hash                |   17 |  …  |   22 | $🪣·a + \sum_j 🧄_j \cdot st_j$       | $b$                                     |
| $i+1$       | `bar`               | 1337 |  …  |   22 | $🪣·a + \sum_{j=0}^9 🧄_j \cdot st_j$ | $🪟·b + \sum_{j=0}^4 🫑_j \cdot st_{j+5}$ |

In order to verify the correctness of `RunningEvaluationHashInput`, the corresponding transition constraint needs to conditionally “activate” on row-tuple ($i-1$, $i$), where it is conditional on `ci_next` (not `ci`), and verifies absorption of the next row, _i.e._, row $i$.
However, in order to verify the correctness of `RunningEvaluationHashDigest`, the corresponding transition constraint needs to conditionally “activate” on row-tuple ($i$, $i+1$), where it is conditional on `ci` (not `ci_next`), and verifies absorption of the next row, _i.e._, row $i+1$.

## Initial Constraints

1. The cycle counter `clk` is 0.
1. The previous instruction `previous_instruction` is 0.
1. The instruction pointer `ip` is 0.
1. The jump address stack pointer `jsp` is 0.
1. The jump address origin `jso` is 0.
1. The jump address destination `jsd` is 0.
1. The operational stack element `st0` is 0.
1. The operational stack element `st1` is 0.
1. The operational stack element `st2` is 0.
1. The operational stack element `st3` is 0.
1. The operational stack element `st4` is 0.
1. The operational stack element `st5` is 0.
1. The operational stack element `st6` is 0.
1. The operational stack element `st7` is 0.
1. The operational stack element `st8` is 0.
1. The operational stack element `st9` is 0.
1. The operational stack element `st10` is 0.
1. The operational stack element `st11` is 0.
1. The operational stack element `st12` is 0.
1. The operational stack element `st13` is 0.
1. The operational stack element `st14` is 0.
1. The operational stack element `st15` is 0.
1. The operational stack pointer `osp` is 16.
1. The operational stack value `osv` is 0.
1. The RAM pointer `ramp` is 0.
1. The RAM value `ramv` is 0.
1. `RunningEvaluationStandardInput` is 1.
1. `RunningEvaluationStandardOutput` is 1.
1. `RunningProductInstructionTable` has absorbed the first row with respect to challenges 🍓, 🍒, and 🥭 and indeterminate 🛁.
1. `RunningProductOpStackTable` has absorbed the first row with respect to challenges 🍋, 🍊, 🍉, and 🫒 and indeterminate 🪤.
1. `RunningProductRamTable` has absorbed the first row with respect to challenges 🍍, 🍈, 🍎, and 🌽 and indeterminate 🛋.
1. `RunningProductJumpStackTable` has absorbed the first row with respect to challenges 🍇, 🍅, 🍌, 🍏, and 🍐 and indeterminate 🧴.
1. `RunningEvaluationHashInput` has absorbed the first row with respect to challenges 🧄₀ through 🧄₉ and indeterminate 🪣 if the current instruction is `hash`. Otherwise, it is 1.
1. `RunningEvaluationHashDigest` is 1.
1. `RunningEvaluationSpongeAbsorb` is has absorbed the first row with respect to challenges 🧅₀ through 🧅₉ and indeterminate 🧽 if the current instruction is `absorb_init`. Otherwise, it is 1.
1. `RunningEvaluationSpongeSqueeze` is 1.
1. `RunningProductU32Table` is 1.
1. The running evaluation of relevant clock cycles is 1.
1. The running evaluation of unique clock jump differences starts off having applied one evaluation step with the clock jump difference with respect to indeterminate 🛒, if the `cjd` column does not start with zero.
1. The running product of all clock jump differences starts starts off having accumulated the first factor with respect to indeterminate 🚿, but only if the `cjd` column does not start with zero.

(Note that the `cjd` column can start with a zero, but only if all other elements of this column are zero. This event indicates the absence of clock jumps.)

### Initial Constraints as Polynomials

1. `clk`
1. `previous_instruction`
1. `ip`
1. `jsp`
1. `jso`
1. `jsd`
1. `st0`
1. `st1`
1. `st2`
1. `st3`
1. `st4`
1. `st5`
1. `st6`
1. `st7`
1. `st8`
1. `st9`
1. `st10`
1. `st11`
1. `st12`
1. `st13`
1. `st14`
1. `st15`
1. `osp`
1. `osv`
1. `ramp`
1. `ramv`
1. `RunningEvaluationStandardInput - 1`
1. `RunningEvaluationStandardOutput - 1`
1. `RunningProductInstructionTable - (🛁 - 🍓·ip - 🍒·ci - 🥭·nia)`
1. `RunningProductOpStackTable - (🪤 - 🍋·clk - 🍊·ib1 - 🍉·osp - 🫒·osv)`
1. `RunningProductRamTable - (🛋 - 🍍·clk - 🍈·ramp - 🍎·ramv - 🌽·previous_instruction)`
1. `RunningProductJumpStackTable - (🧴 - 🍇·clk - 🍅·ci - 🍌·jsp - 🍏·jso - 🍐·jsd)`
1. `(ci - opcode(hash))·(RunningEvaluationHashInput - 1)`<br />
    `+ hash_deselector·(RunningEvaluationHashInput - 🪣 - 🧄₀·st0 - 🧄₁·st1 - 🧄₂·st2 - 🧄₃·st3 - 🧄₄·st4 - 🧄₅·st5 - 🧄₆·st6 - 🧄₇·st7 - 🧄₈·st8 - 🧄₉·st9)`
1. `RunningEvaluationHashDigest - 1`
1. `(ci - opcode(absorb_init))·(RunningEvaluationSpongeAbsorb - 1)`<br />
    ` + absorb_init_deselector·(RunningEvaluationSpongeAbsorb - 🧽 - 🧅₀·st0 - 🧅₁·st1 - 🧅₂·st2 - 🧅₃·st3 - 🧅₄·st4 - 🧅₅·st5 - 🧅₆·st6 - 🧅₇·st7 - 🧅₈·st8 - 🧅₉·st9)`
1. `RunningEvaluationSpongeSqueeze - 1`
1. `RunningProductU32Table - 1`
1. `rer - 1`
1. `cjd · (reu - 🛒 - cjd)) + (1 - cjd · invm) · (reu - 1)`
1. `cjd · (rpm - (🚿 - cjd)) + (1 - cjd · invm) · (rpm - 1)`

## Consistency Constraints

1. The composition of instruction buckets `ib0` through `ib7` corresponds to the current instruction `ci`.
1. The inverse of clock jump difference with multiplicity `invm` is the inverse-or-zero of the the clock jump difference `cjd`. (Results in 2 polynomials.)
1. The padding indicator `IsPadding` is either 0 or 1.

### Consistency Constraints as Polynomials

1. `ci - (2^7·ib7 + 2^6·ib6 + 2^5·ib5 + 2^4·ib4 + 2^3·ib3 + 2^2·ib2 + 2^1·ib1 + 2^0·ib0)`
1. `invm·(invm·cjd - 1)`
1. `cjd·(invm·cjd - 1)`
1. `IsPadding·(IsPadding - 1)`

## Transition Constraints

Due to their complexity, instruction-specific constraints are defined [in their own section](instruction-specific-transition-constraints.md).

The following constraints apply to every pair of rows.

1. The cycle counter `clk` increases by 1.
1. The padding indicator `IsPadding` is 0 or remains unchanged.
1. The current instruction `ci` in the current row is copied into `previous_instruction` in the next row or the next row is a padding row.
1. The running evaluation for standard input absorbs `st0` of the next row with respect to 🛏 if the current instruction is `read_io`, and remains unchanged otherwise.
1. The running evaluation for standard output absorbs `st0` of the next row with respect to 🧯 if the current instruction in the next row is `write_io`, and remains unchanged otherwise.
1. If the next row is not a padding row, the running product for the Instruction Table absorbs the next row with respect to challenges 🍓, 🍒, and 🥭 and indeterminate 🛁. Otherwise, it remains unchanged.
1. The running product for the OpStack Table absorbs the next row with respect to challenges 🍋, 🍊, 🍉, and 🫒 and indeterminate 🪤.
1. The running product for the RAM Table absorbs the next row with respect to challenges 🍍, 🍈, 🍎, and 🌽 and indeterminate 🛋.
1. The running product for the JumpStack Table absorbs the next row with respect to challenges 🍇, 🍅, 🍌, 🍏, and 🍐 and indeterminate 🧴.
1. If the current instruction in the next row is `hash`, the running evaluation “Hash Input absorbs the next row with respect to challenges 🧄0 through 🧄9 and indeterminate 🪣. Otherwise, it remains unchanged.
1. If the current instruction is `hash`, the running evaluation “Hash Digest” absorbs the next row with respect to challenges 🫑₀ through 🫑₄ and indeterminate 🪟. Otherwise, it remains unchanged.
1. If the current instruction in the next row is `absorb_init` or `absorb`, then the running evaluation “Sponge absorb” absorbs the next row with respect to challenges 🧅₀ through 🧅₉ and indeterminate 🧽. Otherwise, it remains unchanged.
1. If the current instruction is `squeeze`, then the running evaluation “Sponge squeeze” absorbs the next row with respect to challenges 🥔₀ through 🥔₉ and indeterminate 🚪.
1.  1. If the current instruction is `split`, then the running product with the U32 Table absorbs `st0` and `st1` in the next row and `ci` in the current row with respect to challenges 🥜, 🌰, and 🥑, and indeterminate 🧷.
    1. If the current instruction is `lt`, `and`, `xor`, or `pow`, then the running product with the U32 Table absorbs `st0`, `st1`, and `ci` in the current row and `st0` in the next row with respect to challenges 🥜, 🌰, 🥑, and 🥕, and indeterminate 🧷.
    1. If the current instruction is `log2floor`, then the running product with the U32 Table absorbs `st0` and `ci` in the current row and `st0` in the next row with respect to challenges 🥜, 🥑, and 🥕, and indeterminate 🧷.
    1. If the current instruction is `div`, then the running product with the U32 Table absorbs both
        1. `st0` in the next row and `st1` in the current row as well as the constants `opcode(lt)` and `1` with respect to challenges 🥜, 🌰, 🥑, and 🥕, and indeterminate 🧷.
        1. `st0` in the current row and `st1` in the next row as well as `opcode(split)` with respect to challenges 🥜, 🌰, and 🥑, and indeterminate 🧷.
    1. Else, _i.e._, if the current instruction is not a u32 instruction, the running product with the U32 Table remains unchanged.
1. The unique inverse column `invu'` holds the inverse-or-zero of the difference of consecutive `cjd`'s, if `cjd'` is nonzero.
    (Results in 2 constraint polynomials.)
1. The running evaluation `reu` of unique `cjd`'s is updated relative to indeterminate 🛒 whenever the difference of `cjd`'s is nonzero *and* the next `cjd` is nonzero.
1. The running evaluation `rer` or relevant clock cycles is updated relative to indeterminate 🛒 or not at all.
1. The running product `rpm` of `cjd`'s with multiplicities is accumulates a factor `🚿 - cjd'` in every row, provided that `cjd'` is nonzero.

### Transition Constraints as Polynomials

1. `clk' - (clk + 1)`
1. `IsPadding·(IsPadding' - IsPadding)`
1. `(1 - IsPadding')·(previous_instruction' - ci)`
1. `(ci - opcode(read_io))·(RunningEvaluationStandardInput' - RunningEvaluationStandardInput)`<br />
    `+ read_io_deselector·(RunningEvaluationStandardInput' - 🛏·RunningEvaluationStandardInput - st0')`
1. `(ci' - opcode(write_io))·(RunningEvaluationStandardOutput' - RunningEvaluationStandardOutput)`<br />
    `+ write_io_deselector'·(RunningEvaluationStandardOutput' - 🧯·RunningEvaluationStandardOutput - st0')`
1. `(1 - IsPadding')·(RunningProductInstructionTable' - RunningProductInstructionTable(🛁 - 🍓·ip' - 🍒·ci' - 🥭·nia'))`<br />
    `+ IsPadding'·(RunningProductInstructionTable' - RunningProductInstructionTable)`
1. `RunningProductOpStackTable' - RunningProductOpStackTable·(🪤 - 🍋·clk' - 🍊·ib1' - 🍉·osp' - 🫒·osv')`
1. `RunningProductRamTable' - RunningProductRamTable·(🛋 - 🍍·clk' - 🍈·ramp' - 🍎·ramv' - 🌽·previous_instruction')`
1. `RunningProductJumpStackTable' - RunningProductJumpStackTable·(🧴 - 🍇·clk' - 🍅·ci' - 🍌·jsp' - 🍏·jso' - 🍐·jsd')`
1. `(ci' - opcode(hash))·(RunningEvaluationHashInput' - RunningEvaluationHashInput)`<br />
    `+ hash_deselector'·(RunningEvaluationHashInput' - 🪣·RunningEvaluationHashInput - 🧄₀·st0' - 🧄₁·st1' - 🧄₂·st2' - 🧄₃·st3' - 🧄₄·st4' - 🧄₅·st5' - 🧄₆·st6' - 🧄₇·st7' - 🧄₈·st8' - 🧄₉·st9')`
1. `(ci - opcode(hash))·(RunningEvaluationHashDigest' - RunningEvaluationHashDigest)`<br />
    `+ hash_deselector·(RunningEvaluationHashDigest' - 🪟·RunningEvaluationHashDigest - 🫑₀·st5' - 🫑₁·st6' - 🫑₂·st7' - 🫑₃·st8' - 🫑₄·st9')`
1. `(ci' - opcode(absorb_init))·(ci' - opcode(absorb))·(RunningEvaluationHashDigest' - RunningEvaluationHashDigest)`<br />
    `+ absorb_init_deselector·(RunningEvaluationSpongeAbsorb' - 🧽·RunningEvaluationSpongeAbsorb - 🧅₀·st0' - 🧅₁·st1' - 🧅₂·st2' - 🧅₃·st3' - 🧅₄·st4' - 🧅₅·st5' - 🧅₆·st6' - 🧅₇·st7' - 🧅₈·st8' - 🧅₉·st9')`<br />
    `+ absorb_deselector·(RunningEvaluationSpongeAbsorb' - 🧽·RunningEvaluationSpongeAbsorb - 🧅₀·st0' - 🧅₁·st1' - 🧅₂·st2' - 🧅₃·st3' - 🧅₄·st4' - 🧅₅·st5' - 🧅₆·st6' - 🧅₇·st7' - 🧅₈·st8' - 🧅₉·st9')`
1. `(ci - opcode(squeeze))·(RunningEvaluationSpongeSqueeze' - RunningEvaluationSpongeSqueeze)`<br />
    `+ squeeze_deselector·(RunningEvaluationSpongeSqueeze' - 🚪·RunningEvaluationSpongeSqueeze - 🥔₀·st0' - 🥔₁·st1' - 🥔₂·st2' - 🥔₃·st3' - 🥔₄·st4' - 🥔₅·st5' - 🥔₆·st6' - 🥔₇·st7' - 🥔₈·st8' - 🥔₉·st9')`
1.  1. `split_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0' - 🌰·st1' - 🥑·ci))`
    1. `+ lt_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0 - 🌰·st1 - 🥑·ci - 🥕·st0'))`
    1. `+ and_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0 - 🌰·st1 - 🥑·ci - 🥕·st0'))`
    1. `+ xor_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0 - 🌰·st1 - 🥑·ci - 🥕·st0'))`
    1. `+ pow_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0 - 🌰·st1 - 🥑·ci - 🥕·st0'))`
    1. `+ log2floor_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0 - 🥑·ci - 🥕·st0'))`
    1. `+ div_deselector·(RunningProductU32Table' - RunningProductU32Table·(🧷 - 🥜·st0' - 🌰·st1 - 🥑·opcode(lt) - 🥕)·(🧷 - 🥜·st0 - 🌰·st1' - 🥑·opcode(split)))`
    1. `+ (1 - ib2)·(RunningProductU32Table' - RunningProductU32Table)`
1. `invu'·(invu'·(cjd' - cjd) - 1)·cjd'`
1. `(cjd' - cjd)·(invu'·(cjd' - cjd) - 1)·cjd'`
1. `(1 - (cjd' - cjd)·invu)·(reu' - reu) + (1 - cjd'·invm)·(reu' - reu) + cjd'·(cjd' - cjd)·(reu' - 🛒·reu - cjd')`
1. `(rer' - rer·🛒 - clk')·(rer' - rer)`
1. `cjd'·(rpm' - rpm·(🚿 - cjd')) + (cjd'·invm' - 1)·(rpm' - rpm)`

## Terminal Constraints

1. In the last row, register “current instruction” `ci` is 0, corresponding to instruction `halt`.
1. In the last row, the running evaluations `rer` and `reu` are equal.

### Terminal Constraints as Polynomials

1. `ci`
1. `rer - reu`
