# Charta Small-Step Operational Semantics

This document defines the formal operational semantics for the core Charta language.

## Core Ingredients

### Sets

- **Sig** = {s₁, s₂, …} - Countable set of signal names
- **Coil** = {c₁, c₂, …} - Countable set of coil names

### Store

A **store** (or state) maps each signal/coil to a boolean:

```
σ : (Sig ∪ Coil) → {0, 1}
```

- `σ(s) = 1` means signal `s` is high (true) this cycle
- `σ(c) = 1` means coil `c` is energised (true) this cycle

## Guards and Rungs

### Guard Syntax

A **guard** is a propositional formula over signals/coils:

```
g ::= true | false | x | ¬g | (g ∧ g) | (g ∨ g)
```

where `x ∈ Sig ∪ Coil`.

Contacts are defined as:
- `NO x` → `x`
- `NC x` → `¬x`

### Rung Syntax

A **rung** is:

```
r ::= g ⇒ c
```

where `g` is a guard and `c ∈ Coil`.

Intuition: "If guard `g` is true under the current store, energise coil `c` in this cycle."

### Program

A **program** is a finite, ordered list of rungs:

```
P = [r₁, r₂, …, rₙ]
```

Order matters if later rungs override or combine with earlier ones.

## Guard Evaluation

Define evaluation of a guard under a store `σ`:

```
⟦true⟧σ = 1
⟦false⟧σ = 0
⟦x⟧σ = σ(x)  for x ∈ Sig ∪ Coil
⟦¬g⟧σ = 1 if ⟦g⟧σ = 0, and 0 otherwise
⟦g₁ ∧ g₂⟧σ = min(⟦g₁⟧σ, ⟦g₂⟧σ)
⟦g₁ ∨ g₂⟧σ = max(⟦g₁⟧σ, ⟦g₂⟧σ)
```

## Single-Rung Update

Each rung `r = (g ⇒ c)` determines what it wants to do to a coil in this scan.

Define a **proposed coil update** function for a rung:

```
upd_r(σ)(c') = {
  1  if c' = c and ⟦g⟧σ = 1
  ⊥  (no opinion) otherwise
}
```

Here `⊥` means "this rung does not set that coil".

## Combining Rungs into a Cycle

### Environment Signals

The environment provides a signal valuation `ι : Sig → {0,1}` at the start of each cycle.

Define the **cycle input merge**:

```
(σ ⊕ ι)(x) = {
  ι(x)  if x ∈ Sig
  σ(x)  if x ∈ Coil
}
```

This means:
- Signals are refreshed from environment each cycle
- Coils start with their previous values until rungs update them

### Rung Combination

For the whole program `P = [r₁, …, rₙ]`, define a **rung-update aggregator**:

```
Upd_P(σ)(c) = combine( upd_{r₁}(σ)(c), …, upd_{rₙ}(σ)(c) )
```

`combine` resolves multiple rung opinions about the same coil. Using OR-combine:

```
combine(v₁, …, vₙ) = max({ vᵢ | vᵢ ∈ {0,1} }) if any vᵢ is defined, else ⊥
```

So:
- `Upd_P(σ)(c) = 1` if there exists `i` such that `upd_{rᵢ}(σ)(c) = 1`
- `Upd_P(σ)(c) = ⊥` otherwise

### Coil Store Update

Define how coils get their next value:

```
σ'(x) = {
  ι(x)  if x ∈ Sig
  1     if x ∈ Coil and Upd_P(σ ⊕ ι)(x) = 1
  0     if x ∈ Coil and Upd_P(σ ⊕ ι)(x) = ⊥ and coils default to off
  σ(x)  if x ∈ Coil and Upd_P(σ ⊕ ι)(x) = ⊥ and latching coil
}
```

Different variants:
- **Non-latching coil**: `⊥` ⇒ `0` (coil drops if no rung energises it)
- **Latching coil**: `⊥` ⇒ keep previous `σ(x)`

## Small-Step Transition Relation

### Configuration

A **configuration** is:

```
<σ, ι>
```

where:
- `σ` is the current store
- `ι` is the current input snapshot

### Cycle Step

The **cycle step** is:

```
<σ, ι> → <σ', _>
```

where `σ'` is computed as above. The next cycle will have a fresh `ι'` from the environment.

### Formal Rule

Let `σ₀ = σ ⊕ ι`

For all `x ∈ Sig`:
- `σ'(x) = ι(x)`

For all `c ∈ Coil`:
- If `Upd_P(σ₀)(c) = 1` then `σ'(c) = 1`
- Else:
  - Non-latching: `σ'(c) = 0`
  - Latching: `σ'(c) = σ(c)`

So the **small-step rule** for one program `P` is:

```
Given σ, ι and program P, compute σ' as above
─────────────────────────────────────────────────
<σ, ι> →_P <σ', _>
```

## Adding Blocks

Blocks do not change the control semantics; they provide derived signals/coils.

A **block** `B` with inputs and outputs is:

```
B : (Sig ∪ Coil)* × State_B → (Sig ∪ Coil)* × State_B
```

The Charta VM evaluates all block outputs at each cycle before or during rung evaluation. Those outputs appear as additional signal values in `ι`, or as derived coil-like values in `σ`.

### Extended Semantics

The small-step semantics expand to:

1. Read environment inputs `env : Sig_env → {0,1}`
2. Run all blocks to compute derived signals
3. Form composite input `ι` (env + block outputs)
4. Apply the rung-based transition as above to produce `σ'`

The core small-step structure remains the same.

## Adding Evidence

Evidence types extend the store to carry structured evidence objects with confidence, source, and metadata.

### Evidence Objects

An **evidence object** is:

```
Evidence[T] = {
  value: T
  confidence: float  # 0.0 to 1.0
  source: label
  evidence_type: label
  disputed: bool
  verifiable: bool
  permitted_use: list[label]
}
```

### Store Extension

The store (σ) is extended to map signals/coils to either boolean values or evidence objects:

```
σ : (Sig ∪ Coil) → {0, 1} ∪ Evidence[T]
```

For evidence-typed signals:
- `σ(s)` where `s : Evidence[T]` yields an `Evidence[T]` object
- Evidence properties are accessible: `σ(s).confidence`, `σ(s).disputed`, etc.

### Evidence Property Evaluation

Guards can reference evidence properties:

```
⟦evidence.confidence < threshold⟧σ = {
  1  if σ(evidence) is Evidence[T] and σ(evidence).confidence < threshold
  0  otherwise
}

⟦evidence.disputed⟧σ = {
  1  if σ(evidence) is Evidence[T] and σ(evidence).disputed = true
  0  otherwise
}
```

### Evidence Normalization Blocks

Evidence normalization blocks transform probabilistic inputs into evidence objects:

```
NormalizeEvidence : Any × label × label → Evidence[T]
```

Normalization happens in the block phase:
1. Agent/LLM blocks produce probabilistic outputs
2. Normalization blocks transform to `Evidence[T]`
3. Evidence objects enter the store via signals
4. Guards reference evidence properties
5. Rungs produce deterministic coil states

### Deterministic Collapse

The evidence layer ensures deterministic outcomes:

- **Input**: Probabilistic data from LLMs/OCR/sensors
- **Normalization**: Structured evidence objects with confidence
- **Guard Evaluation**: Evidence properties evaluated deterministically
- **Coil Updates**: Deterministic based on guard results
- **Output**: Deterministic actions

All stochastic behaviour ends at normalization. Charta itself is deterministic from guard evaluation onward.

### Extended Small-Step with Evidence

The small-step semantics extend to:

1. Read environment inputs `env : Sig_env → {0,1} ∪ Evidence[T]`
2. Run normalization blocks to transform probabilistic inputs → `Evidence[T]`
3. Run all blocks to compute derived signals (may produce evidence)
4. Form composite input `ι` (env + block outputs, including evidence)
5. Evaluate guards (including evidence property access)
6. Apply rung-based transition to produce `σ'`

Evidence objects flow through the system but do not change the fundamental transition structure.

## Adding Time

Extend configuration to include:

- `σ` (signals/coils)
- `ι` (inputs)
- `τ` (timer states, including elapsed times)
- `Δt` (cycle duration)

Transition:

```
<σ, ι, τ> -[Δt,P]-> <σ', ι', τ'>
```

with `ι'` constructed from environment + block outputs.

Timer semantics:
- For each timer block, update its internal state and outputs based on `σ` and `Δt`
- Merge derived signals into `ι`
- Run the rung/coil logic as before
- Timers carry updated state `τ'` to next cycle

## Where Agents and MCP Fit

On this formal core:

- An **agent call** corresponds to a block invocation whose outputs may lag or be stochastic, but from the VM's perspective it is just another block with outputs and internal state
- **Governance constraints** become additional rungs and coils that gate these blocks (e.g., a coil `allow_agent_A` must be energised for the block `AgentA` to be evaluated)
- **MCP tools** act like "I/O modules": they provide and consume signals, just as sensors and actuators do for a PLC

The core language remains a deterministic or at least structurally well-defined control calculus; LLMs and agents are implementations behind specific blocks and signals.

## Why This Is "Fundamental"

This small-step core:

- Defines execution without reference to any host language
- Mirrors ladder/relay semantics: scan inputs, evaluate rungs, update coils
- Admits additional structure (blocks, agents, cost signals, governance signals) without changing the base transition pattern

Other languages can host this calculus, but they do not define it.

