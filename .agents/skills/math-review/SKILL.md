---
name: math-review
description: Review a mathematical proof or solution for structure, logic, detail,
edge cases, and style. Use when asked to check, review, or verify a proof, solution, or exercise answer.
---

# Reviewing math solutions

Check submitted proofs against these items. Report the flaw and the missing
reason; do not rewrite the proof or supply a corrected one.

## Structure

- Check the proof opens with the assumptions and the claim, names the strategy
  (direct, contrapositive, contradiction, induction), and for an iff proves both
  directions with labels.
- Check a contradiction negates the claim explicitly, and a contrapositive is
  written out before it is proved.
- Check the proof ends at the claim, restating it, with no commentary after.
- Check it reads as an essay: full sentences, formulas inside sentences, no
  sentence starting with a symbol.

## Logic

- Check quantifiers are explicit and correct ("for all" vs. "for some"); check
  definitions are used in their formal form and negated correctly.
- Check every variable has a type and origin (arbitrary, existentially
  instantiated, explicitly defined) and that no symbol is reused.
- Flag unused hypotheses: the proof is incomplete or the statement is stronger
  than needed.
- Flag any setup line that assumes the conclusion.
- Flag "clearly", "obviously", or "it is easy to see" unless the step is
  justified or genuinely immediate; require the cited result ("by the Linear
  Dependence Lemma", "by 2.22").
- Check the proof mode is the weakest that works; flag WLOG without a stated
  symmetry.
- Check long arguments are split into lemmas or claims.

## Detail

- Check degenerate cases are handled first and explicitly (empty list, zero
  vector, n = 0, U = {0}); flag a skipped case whose exclusion is unstated.
- Flag sentences that set no goal, introduce no variable, and derive nothing.
- Check steps obvious to the author are justified for a reader who has not seen
  the problem.
- Check existence proofs exhibit the object and verify it; uniqueness proofs
  separate existence from uniqueness.
- Check construction proofs include the construction, why it is valid (the
  invariant is maintained), and why it terminates.

## Style

- Flag symbols in prose (exists, iff, WLOG) and symbols at the start of a
  sentence.
- Check sentences are short and equalities/inequalities are chained.
- Check symbols are defined before use and that no trailing "where..." clause
  explains notation after it has been used.

## Falsification

- Substitute edge-case values, look for the case the proof forgot, and confirm
  the last line is exactly the claim.
