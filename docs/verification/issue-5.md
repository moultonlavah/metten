# Issue #5: source foundation evidence

## Current evidence index

**Current outcome: step-5 implementation and the owner-requested branding cleanup are present, with the packaged Sources GUI proof blocked.** Start at the [implementation receipt](#step-5-final-implementation-receipt): 237 ordinary Rust tests, 18 interface tests, correctness lint, Clippy, formatting, frontend/Mac bundle builds, process regressions and the finished bundle's isolated native credential check passed. The Sources walkthrough could not access its first WebKit input; guarded exit and owned cleanup passed. This is an implementation checkpoint, not full step-5 packaged acceptance or issue acceptance. All four issue criteria remain unproved as a whole.

The unchanged [step-5 plan, revision 1](../proposals/issue-5-egm/step-5-plan.md) received [independent Sol plan approval](#step-5-independent-sol-plan-review), followed by Connor's explicit implementation approval. That review covers the plan, not this implementation. [Step-4 evidence and its Sol checkpoint](#sol-round-2-approval-and-checkpoint) remain reusable for unchanged boundaries. Calibration, frozen qualification, documentation/owner setup and separate external acceptance remain later gates. Earlier statuses below are historical.


**Historical step-2 status: SP1 is corrected; covering checks pass and Sol approved the current behavior in round1 with no findings.** Start at the [late-verification correction](#sp1-late-verification-display-correction-and-sol-loop). The implementation is present against the [final step-2 plan](../proposals/issue-5-egm/step-2-plan.md), revision 5 (`528b87824818515a4349ee99b658f9bc77ed2deb532e514b26f7d7ffa8b60a76`). Owner authority: after reviewing the corrected scope and plain-language explanation, Connor explicitly said **“Okay I approve. Implement”**. This includes the named atomic evidence admission/paired-invalidity correction and shared registration checks. It does not authorize providers, accounts, credentials, UI, trading, dependencies, commits or publication.

Selected branch `t3/brainstorm-issue-5`, starting HEAD `96d3ff3a591a3231c8ecf600c858292b0b2fe7e9`. Existing docs/README edit and untracked proposals/reviews are preserved. The published PR #44 maintainability standard at `f90316e283115f5c6ab9c0c54f47858b2a67703c` was read in full again before code. Issue #5 is open with four unchecked criteria/no comments; native blockers #3/#4 remain closed (fresh read-only check). The approved 599-line design stays unchanged.

Expected change path: validated requirements → source owner → existing normalized evidence → evaluator admission/application → controlled public tests, with engine export, owning contract and code-map/evidence updates. The eight-file scope is the plan's inventory; source protocol, history/calculation, storage, native commands and UI retain their owners.

## Published guidance baseline

**Historical step-2 delivery baseline** (current step-3 baseline is fa366362 in its continuation receipt): origin/main **890af2eaff4fa525163bc00339ce081dbeeee4bf**; the latest lead/reviewer refresh receipts are in the [startup correction](#startup-signed-time-correction-and-sol-loop). The earlier 02:01 UTC receipt below is historical. Selected code remains96d3ff3a591a3231c8ecf600c858292b0b2fe7e9 plus the approved eight-file uncommitted scope. The a22a9a9 entries below preserve the earlier implementation/review and owner-requested refresh chronology; they are not a competing current baseline.

Connor refreshed this existing session with the published-instruction discovery rule during implementation. The lead queried the selected origin (`https://github.com/moultonlavah/metten.git`), repository default branch (`main`) and live commit API, obtaining **`a22a9a9188b683e07be141702e93295d83a4ade1`** on 2026-11-15. This is fresh remote evidence, not a cached tracking ref. Read immutable root AGENTS.md and its applicable linked maintainability, code-map, verification (including maintainability review), issue-workflow, triage and domain procedures from that commit. The published recursive tree contains root AGENTS.md but no separate instruction-discovery procedure. No extra local/ancestor instructions were found; local task/design/ADR/source contracts remain applicable. Source/index/tracking refs were preserved; no merge or fetch was needed.

The current session explicitly loaded that entry/revision and followed discovery successfully. The user-supplied discovery rule itself is not claimed installed in the published entry, and older Council sessions predate this refresh: no retroactive compliance claim. Any new reviewer must independently discover applicable linked guidance from this pinned root entry and report its loaded revision. Refresh live published guidance again at delivery and repeat only affected verification. The bounded implementation reviewer was independently refreshed to this same published entry/revision and completed source discovery successfully; its review is recorded below.

Published issue workflow asks for routine assignment/Project updates, but the approved bounded task explicitly excludes issue mutations/publication; no tracker writes follow from discovery. Body/native blockers remain satisfied. No consequential behavioral conflict was found. The maintainability standard is the same one used for planning; current verification adds the expected-path/justified-structure review already recorded here. No code check is invalidated solely by this guidance refresh.

## Checks

The earlier candidate checks and subsequent failing review reproductions are historical evidence below. The latest correction section records the fixed source identities and passing checks for R1–R5; the old review applies to its pre-correction snapshot. No issue #5 provider acceptance follows. U1–U7 remain open; controlled leases/history are test inputs only.

## Controlled regression and implemented path

New public pair regression was exercised against an isolated temporary copy of actual HEAD engine sources. It failed with `Rejected(EvidenceError { code: "unregistered_binding" })`; the corrected engine passes it and the later lease/replay regression. The first draft test failed for a missing side selector and was corrected; that failure did not establish the bug. An initial isolated `--locked` attempt refused the temporary package's workspace-pruned lock; the actual reproduction used `--offline` in that disposable package only, preserving the selected lockfile/dependency versions. All candidate checks use --locked. The temporary source copy was removed after the check.

Source requirements derive only from the validated representation; shared pure descriptor/scope checks retain engine catalog/workload admission at its owner. Implicit ask meaning is structured once and exact historical explanation lines are tested. The source owner uses controlled native Rust evidence, no provider formats. Connection/metadata/discovery/update completions are generation fenced. Public clocks, coherent multi-binding updates, initialization/replay/observations, leases/coverage, current/next preparation, rejected admission, Busy/Stop and bounded retention are exercised through the public source/evaluator seam.

The source API adds token reads needed to attach captured generations to pending work; no forwarding layer or new runtime is introduced. The 256-KiB view charge has a real over-cap case using admitted field lengths and escaping-heavy synthetic identities, returning resource_limit rather than a partial view. A capacity-limit fixture initially cloned an empty Vec, normalizing away the excess capacity; it was corrected to pass the actual owned over-cap Vec. This was a test-fixture correction, not a waived production limit. The record/index route is in the code map; main docs-map guide/evidence links remain explicitly deferred to approved step7, preserving the inherited docs/README edit.

## Changed processing and memory argument

The fixed Rust 1.100.0 proof convention and unmodified history arithmetic from issue #4 remain applicable. Changed paths are the shared pure check extraction, per-transaction admission mask, per-current-transaction application mask and invalid-pair finalization. Masks have at most128 registered keys each, with u32 key/u8 value. Validation drains its map between transactions; application builds/drains only the current transaction. No mask per whole batch is retained. Both maps, including retained empty roots, are charged simultaneously by `2 * tree_bytes::<u32,u8>(128)` = **9516 bytes**, replacing the old single u32/unit tree charge of4472. This adds5044 to the existing private metadata reservation, not to its 8-MiB allowance.

Rust's existing 11-key/12-edge, five-key minimum-occupancy derivation still applies: node bound26, each conservatively128 +11*(4+1)=183 bytes, one map4758. Independent heap measurement of two simultaneously filled/drained new typed maps observed **3600 bytes** peak against9516 reserved. This measurement checks the actual changed type, including retained roots; it supplements the general node argument rather than proving arbitrary allocator/RSS behavior.

Work remains sliced under the existing outer64 + per-dispatch64 accounting. Validation map insertion/cleanup remains <=240 tree primitives plus fixed mask control under256. Ordinary application adds one <=240 map operation to the existing <=128 raw lookup (or unchanged bounded market-reset work); it remains under its1024 reservation. Finalization reserves512: one <=240 map pop plus <=128 raw search and fixed scalar/mask clearing. Rejection cleanup pops one entry at a time. Shared pure descriptor checks retain the same bounded three128-byte identity scans,128 catalog visits and association loops, with actual instrumentation; no copied whitelist or per-binding requirement allocation is introduced. Existing unaffected heavy branches remain at their prior reservations. The independent whole-slice probe in the new128-binding/256-member invalid-pair case observed **2907 primitives maximum**, below4096; all existing branch/extrema/continuation/resource probes passed in the final engine test run. This is algorithmic work, not a latency claim.

Source-owned state: max64 allocated requirement slots (<=34 live),128 binding slots,4 market slots (<=2 live),6 streams and3 connections; no raw book, frame queue or historical series. On this arm64 compiler, measured inline sizes are owner272, Bound304, BindingView320, SourceRequirement88, StampEntry32 and Market96 bytes. Each retained binding owns <=320 descriptor-text bytes plus <=256 provenance bytes; requirements own <=100 bytes each; markets own <=384 bytes each; current ID<=128 and retained selected series<=128. Conservative retained owner bound:272 +64*88 +34*100 +128*(304+576) +4*96 +2*384 +128 +128 = **123352 bytes**, below128KiB. Insertion, clone-normalized strings and caps prevent hidden retained input capacities; source-only handoffs reuse the existing allocation and release retired payloads.

One returned typed view owns <=128*(320+676) binding bytes plus bounded current/next/source/stream headers, below160KiB before the stricter construction guard. The guard additionally charges2048 fixed/header bytes,512 per market and1024 per binding for keys/scalars/variants plus TWO bytes per text byte (maximum escaping when controls are excluded), and refuses above256KiB. It is a conservative representation allowance, not a newly published native JSON/IPC schema. A later wire projection must enforce its own approved decimal-string counters/envelope cap.

One owned normalized input plus one effect fits a conservative128KiB each: <=128 update/binding records with bounded text, <=256 EvidenceChange slots (or two128-member revocation vectors), <=128 coverage facts/stamp entries, at most4 transaction allocation slots and at most2 current/next registrations. Scratch uses bounded scans, at most128 borrowed matching references per requirement, and no elapsed-time loop. Owner + one input + one effect + one view + constructor overlap stays below1MiB from these type/collection bounds, separately from unchanged evaluator history/metadata/ingress admission. Caller-retained older clones and allocator arenas/RSS are outside that contract. No performance/capacity qualification is inferred.

## Owner-requested existing-session refresh receipt

After Connor's explicit refresh-only instruction, the lead again independently queried origin/default/main and loaded immutable published AGENTS at **a22a9a9188b683e07be141702e93295d83a4ade1**. Selected code HEAD remains **96d3ff3a591a3231c8ecf600c858292b0b2fe7e9** with the authorized uncommitted eight-file implementation. Read the linked maintainability, verification, code-map, issue-workflow, triage and domain procedures in full; reread local root AGENTS, glossary and ADR0001, with applicable local task documents already loaded. Published recursive discovery found only root AGENTS and no separate discovery procedure; no local ancestor/nested instruction files apply. The first recursive-tree query needed shell quoting; the corrected query succeeded. No missing guidance or new consequential conflict. Tracker writes remain excluded by owner authority. Code/index/tracking refs were preserved.

Native child inventory shows all children completed, including the implementation reviewer (which independently loaded/discovered the same a22a9a9 guidance during its review). No active child requires refresh. Older Council reports remain historical, unrefreshed under the new standing rule, and are not reused as implementation correctness evidence. No new agents/reviews/actions were started for this refresh; original implementation authority and exclusions remain exact.

## Implementation review and corrections

One bounded independent implementation reviewer loaded the published a22a9a9 guidance independently and matched the supplied source hashes before review. It inspected the actual source, contracts, public tests and resource derivations without running tests or changing files. Its three confirmed findings were corrected by the lead within the same owners:

- Reverse-order disjoint coverage could bridge an unproved interval. Merge now requires overlap/adjacency in both directions; disjoint old facts do not replace the newest contiguous summary. The public reverse-coverage regression stays unavailable until the actual bridge is supplied.
- Expired, never-ready markets could occupy both discovery slots indefinitely. Boundary/Advance retire closed source records independent of promotion, preserve underlying bindings and the selected series, release future slots and make old/rebound stamps fail. The skipped-window regression also refuses another series after all old records retire.
- Invalid typed values/conflicting observation identities revoked source generations but returned no evaluator batch. The finite error protocol is now explicit: a six-bit bounded revocation-required mask blocks ordinary apply after either error; the consumer must call invalidate on the affected stream, deliver that effect to the evaluator, and complete it before initialization. The public source/evaluator timeline shows candidate removal on delivered revocation and recovery only after fresh initialization. Delivery ordering remains the controlled consumer's responsibility; no acknowledgment/runtime coordinator was added.

A lead follow-through also binds each captured token to its exact stream, in addition to owner/generation; a same-generation book token cannot verify metadata. The public obsolete-completion test pins this identity boundary. These corrections alter only sources.rs/tests and its owning contract; they neither change evaluator history rules nor widen capability support.

All three finding regressions were run against a disposable copy with their corresponding faulty behaviors deliberately restored. Exactly those three checks failed (false history-ready, retained old stamp/market slots, and initialization accepted before revocation). The corrected final candidate passes all three. This is a synthetic faulty-behavior experiment, not a second independent review or claim that the reviewed source was executed. Disposable source was removed; the selected source/index/lockfile stayed intact. The initial new recovery test lacked an ObjectiveView and therefore could not produce a candidate; its controlled objective was supplied, then a borrowed/owned argument mismatch was corrected. These fixture failures were not counted as proof of production bugs. A shell without cargo on PATH was resolved by the existing absolute tool path, without installation.

The independent review covers its recorded pre-correction hashes; final corrections were inspected and tested by the accountable lead, not silently relabeled as independently re-reviewed. Historical Council scores assess plans only. This issue is not counted as a completed member of PR #44's three-new-issues adoption trial.

## Final candidate checks and limits

Pinned compiler: Rust1.100.0 (0ef1bc386, 2026-11-09), local arm64. All candidate cargo commands used the existing /Users/connormoulton/.cargo/bin/cargo and --locked.

- cargo fmt --all --check: passed.
- cargo test -p metten-engine --locked: **120 passed**, two release-only fixtures excluded here as intended (24 unit,6 session,21 source,59 ordinary strategy,10 strategy-storage).
- cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture: **both full-hour fixtures passed on final source**. Each reported allocated_bytes=65988608; registered-contract fixture7461ms and hour-average fixture8020ms. These are controlled local fixture measurements, not production delivery/latency or provider qualification.
- cargo clippy -p metten-engine --all-targets --locked -- -D warnings: passed. Earlier lint failures were corrected before this final run.
- Affected Markdown paths/anchors in engine contract, code map and this record: checked. Final whitespace/diff and eight-file scope checked; inherited docs/README one-line edit preserved, index empty, no commits or issue writes. The approved design remains599 lines with SHA25621e2926329f67fef05b264dcb8a0dfca0455aaa697c753bfc4f2d95b19fe4bc7; rev5 plan remains528b87824818515a4349ee99b658f9bc77ed2deb532e514b26f7d7ffa8b60a76.

Behavior route:21 public source timelines cover acknowledgment without data; honest unsupported profiles; shared book loss/repeated recovery; genuine observations vs initialization/replay; precision/identity rejection; complete history vs current validity; control-completion fencing; dependency stamps; Stop/Busy/rejected admission; once-only/late/expired market handling; and source/view/capacity limits. Five public strategy regressions cover atomic pairs and member orders, forbidden combinations, lease/replay non-revival, requirements/exact explanation output and unchanged resynchronization/crossing/after-checkpoint meanings. Two independent internal mask probes plus existing resource/continuation checks cover the changed evaluator storage/work branch; one source internal counter/size check covers exhaustion and type inventory.

Final source identities:

- lib.rs:64e426574b46c8d684008988e690d85b061826527ecdc1fe60d06af66ab06d72
- strategy.rs:130d390b40459e11526791bd06a49a8f202a36b3907b21272656cda1fd81035c
- sources.rs:b69d454a6576d4ab74c6e5b034e9ea3db671cea8f86e7d2dea56bb9b09a81059
- tests/strategy.rs:2320ca107b96a33f68affb3e2271df36ff905ec6a4eb460d99531361bf57b364
- tests/sources.rs:1349524d8fb1194f503fb6b3a1d6c06393fa551b248c0561ec324ce9c85cd73f

The final live query caught publication of PR #47: main advanced from a22a9a9 to **890af2eaff4fa525163bc00339ce081dbeeee4bf**. The lead read the new immutable root AGENTS and complete published-instruction discovery procedure, independently traversed/read the maintainability, verification, code-map, issue-workflow, triage and domain documents at that same revision, and inspected the recursive tree (only root AGENTS). The changed files are AGENTS and verification only: they publish the discovery/activation rule already delivered explicitly to this session. No product, source owner, engine validation, resource or required engine-check contract changed. Source/test checks and the earlier independently discovered technical review remain applicable; instruction discovery itself was repeated by the lead for the new procedure. The earlier completed reviewer is not claimed to have read a document published after it finished. No active children required refresh, no new review was started, and no universal host installation/delivery claim is made by this engine task. No unavailable applicable guidance or consequential authority conflict remains. The expected route is the actual eight-file route. sources.rs isolates the current shared invalidation/identity/preparation need; the foundation session/storage remains unrelated, registration semantics share existing pure checks, and evaluator calculations/history remain single-owned. No new dependencies, traits, generic manager, native commands, wire formats or strategy-name branches. Ordinary provider field corrections remain with later adapters; recovery changes stay with this owner; supported thresholds remain validated JSON. The engine contract and map now lead the next agent to the actual owner, test and evidence.

Not checked: providers, access rights, credentials/accounts, real feeds, app/browser/Tauri packaging, workspace/native integration, performance under live delivery, capacity qualification, legal clearance or trading. These paths were not changed or authorized; engine-focused checks are the relevant CONTRIBUTING subset for this step. A staged-tree checkpoint was not run because no staging/commit is authorized. No deployment/publication occurred. U1–U7 and provider-specific observation rules remain unresolved; simulated current/history facts confer no provider support. Production consumer integration must still supply substantiated capabilities, exact dependency sets and enforce revocation delivery/one-batch ordering before use.

## Latest four-reviewer code review

This section supersedes the earlier completion assessment; it preserves the earlier successful check results within their actual coverage. **Three newly exercised behaviors fail on the unchanged candidate. No implementation fix was made during this review.**

### Reviewed comparison and instruction baseline

Connor explicitly confirmed review of all uncommitted step-2 changes, including new files, against96d3ff3a591a3231c8ecf600c858292b0b2fe7e9. HEAD is that same commit, so git diff96d3ff3...HEAD and the commit list are empty; they were not misrepresented as the implementation diff. The effective comparison is git diff96d3ff3 -- the five tracked implementation paths, plus git diff --no-index -- /dev/null for sources.rs, tests/sources.rs and this record. The unrelated docs/README edit and inherited proposal/research/review artifacts stay excluded as changes.

The eight-file candidate was frozen in a temporary bundle with original base files, full diff, exact SHA256 manifest, complete immutable published guidance, recursive tree and fresh issue body/comments/native blockers. The source/test hashes match the Final source identities above; this record's reviewed pre-report hash was7310369f2e2f178c169e2d7cccab6c34ad06cf90d2b98e9e56f9eb73ff0b41e7. Appending these findings changes documentation only after the reviews completed. No code, index, refs or inherited work changed.

Lead independently queried public origin/main at02:09UTC, obtaining890af2eaff4fa525163bc00339ce081dbeeee4bf, and read its root AGENTS/discovery procedure and applicable links. The supplied immutable published bundle is a version-pinned alternative to each reviewer doing network retrieval; reviewers independently traversed the entry/links. Lead delivery query at02:43UTC returned the same main commit. Issue5 remains OPEN with no comments, four unchecked items and native blockers3/4CLOSED. No tracker write occurred.

### Reviewer roles and results

At Connor's request, the review expanded from the skill's two defaults to exactly four independent slots: Sol6.1 Standards, GLM5.3Flash Free Standards, a separate Sol6.1 Spec, and DeepSeekV4.1Flash Free Spec. Both Sol reviewers used the existing Codex subscription at high reasoning; free reviewers used the configured OpenCode free provider, high variant, plan agent. No paid fallback, replacement family or Council round was added. The first Sol brief covered both axes and was narrowed to Spec before its final report; a fresh separate Standards Sol completed the fourth slot. Reviewers did not receive other reports before completing first passes.

- **Standards / Sol:** no supported documented-rule violation or concrete smell finding in its bounded pass. Traced requirements, construction/registration, clock/identity/capability checks, effects, masks/finalization, generation/stamps, retirement, Stop, rejection/Busy and resource evidence. Loaded the supplied pinned root and independently followed verification/discovery, maintainability, map, domain/glossary, ADR0001, CONTRIBUTING and SECURITY plus applicable local contracts/product/plan/tests. No mutations/test execution; static assessment only.
- **Standards / GLM:** no hard violation reported; five heuristic suggestions: repeated binding-text capacity checks, repeated scope-to-market lookup, two similar error-revocation branches, provenance fields traveling together, and the short private name Bound. Lead accepts the repeated capacity check as a useful optional cleanup candidate; the other small helper/type/naming changes remain optional judgments without a demonstrated acceptance failure. Refactoring them must not add surface or disturb work accounting merely for style. GLM explicitly did not read CONTRIBUTING/SECURITY and some linked process/domain documents, so its guidance coverage is partial; no full-compliance claim is inferred. It read the supplied root, maintainability, verification, map, ADR and owning contract and matched all candidate hashes.
- **Spec / Sol:** three concrete findings (R1–R3 below), independently grounded in original source/spec. Loaded all eight candidate and plan/design hashes; used supplied immutable890af2e guidance, independently traversed root/verification/discovery/maintainability/map/domain/issue/triage/CONTRIBUTING/SECURITY, applicable local instructions, product/reference/profiles/plan/contract/tests. No material guidance conflict; static traces only until lead reproductions below.
- **Spec / DeepSeek:** awaiting-reason conflation, missing positive implicit-ask parity regression, and a proposed narrower invalidation scope. It used the supplied890af2e bundle and applicable plan/design/source/reference/engine contracts. Its final receipt did not establish traversal of every linked procedure, so full procedural coverage is not claimed. Lead confirms the first two gaps with the narrowed interpretations below. The third is not accepted as a demonstrated contract violation: the approved owner invalidates stream/subscription/connection scopes, and the cited text does not require binding-only revocation for authenticated inconsistent stream evidence. Finer invalidity granularity should not be invented as a compulsory correction from that wording alone.

### Supported Spec findings and lead verification

**R1 — P1: metadata-only invalidation leaves a prepared ask-only market and its old stamps usable.** sources.rs:1021 dispatches invalidation to stream-assigned Bound records. Ask-only requirements have no Metadata Bound record; Market.verified and market_revision are not revoked. The approved plan requires metadata/schedule uncertainty to revoke preparation and relevant metadata/preparation revisions to fence results. The lead reproduced a prepared market → invalidate(Stream::Metadata, MetadataChanged) → old ask stamp still current, Preparation::Ready, zero revocation transactions, and an emitted next-opening boundary. Correct the shared metadata invalidation path to revoke market preparation/stamps even without a reference binding, require re-verification before promotion, and add that timeline.

**R2 — P1: first source readiness at opening can produce a candidate in that same window.** sources.rs:950–955 tests readiness at opening but retains no initial source-readiness time. The ask-only evaluator initializes ready_at=Some(0) for zero-history requirements (strategy.rs:2665 onward); its guard cannot establish source availability before opening. Controlled timeline: connect at0, first initialization/verification at open1, Boundary1 and a valid controlled objective. The lead observed source_opening_transaction_count=1 and Some(Candidate{market_id:m,option_id:buy,side:Yes,remaining_quantity:1}). No order was submitted. This violates the plan's “Initial readiness exactly at opening still waits for a later full market.” Track/gate initial source readiness strictly before the first eligible opening, preserving active-run recovery, and add a source/evaluator boundary regression. The earlier review's generic readiness-guard reasoning missed the zero-history constructor case; this concrete reproduction supersedes that inference without rewriting the historical report.

**R3 — P2: constructor allocates an oversized identity before rejecting it.** sources.rs:347 clones the borrowed first series_id before market-count/identity-length checks. The lead supplied a synthetic2MiB series ID and independently measured a2097152-byte allocation during SourceState::new before rejection. This violates the explicit reject-before-copy admission rule. Validate count and the borrowed initial identity/descriptor before copying; add a focused constructor allocation regression. The retained admitted-record inventory above remains a type bound, but cannot establish the constructor's simultaneous transient bound until this path is corrected.

**R4 — P2: awaiting and expired current data share the same snapshot reason.** sources.rs:1291 uses Freshness both before any verified value and after an initialized lease expires. initialized/capabilities provide extra facts, but the promised distinct snapshot reason is absent. Add an explicit awaiting current-state reason while keeping capability limitations separate, and pin absent/expired/refused distinctions. This is a reporting-contract gap, not proof of a bad live trade.

**R5 — P3: positive implicit-ask source/evaluator parity is not maintained in tests.** tests/sources.rs:860 exercises illegal scopes/implicit side. Other source/evaluator tests already cover valid explicit selected-side asks, so DeepSeek's broader “no positive scope” wording is overstated. No public source constructor test exercises a legal generated implicit ask, however, despite the approved parity requirement. Add accepted explicit/implicit registrations with exact sides to protect that newly shared path. This is missing regression coverage, not a demonstrated constructor failure on valid implicit input.

### New controlled reproduction evidence

The lead ran three additional assertions against a disposable copy of the exact frozen candidate, using public SourceState/Evaluator calls and a test-only allocation observer. They failed as expected, exposing R1–R3:

- review_metadata_loss_revokes_preparation_and_market_stamps_without_reference_inputs: old_stamp_current=true, preparation=Ready, revocation_transactions=0, boundary_transactions=1.
- review_initial_readiness_at_open_must_wait_for_next_full_market: source_opening_transaction_count=1; exact_opening_candidate=Some(…Yes,remaining_quantity:1).
- review_constructor_rejects_oversized_series_before_copy: largest_constructor_allocation=2097152 for rejected synthetic input.

Commands used cargo test --offline --test review with focused name filters (constructor/opening assertions shared a --test-threads=1 run; metadata ran separately), Rust1.100.0 and the selected dependency lock copied into the temporary standalone engine package. Workspace-pruned resolution happens only in that temporary copy; the selected Cargo.lock is unchanged. Temporary source/test packages were removed on exit. These are real failing assertions on candidate behavior, not restored-fault mutations or model scores. The earlier120 ordinary tests/two release fixtures still describe the earlier unchanged suite; no broad rerun was needed to establish these missing cases. No provider, account, application or trading operation was exercised.

### Owner-requested skill update during this review

Changed only the installed local /Users/connormoulton/.codex/skills/code-review/SKILL.md and the matching writing-for-agents/references/model-selection.md pointer. The skill now requires separate Sol6.1+GLM Standards and Sol6.1+DeepSeek Spec reviewers, preserves independent contexts and per-axis reports, allows capacity waves, and reports unavailable slots instead of silently shrinking the roster. The shared policy points back to that authoritative roster. Existing model/provider cost rules and unrelated workflows remain unchanged. No Metten dependency was added.

SKILL.md SHA256591a0dc8c2d1e5e50a5b9e9648f253649c2a458e1c708ec4103f592f66ee6117; shared policy SHA256f6c4b8acc4bbdda2a866cb48f9f31b0c630f1542618d82f5f92315aca3335bcf. The bundled quick_validate.py could not run because installed Python lacks PyYAML; no package was installed. Frontmatter parsed with existing Ruby YAML; four exact role slots, separate Sol instances, missing-spec/unavailable-role behavior, relative links/heading anchors and whitespace were checked. Those checks validate the document, not automatic delivery to every host/session. This active review used the four requested roles; future automatic skill dispatch was not separately forward-tested.

**Disposition:** three reproduced correctness/resource bugs and two additional contract/coverage gaps require correction before step2 is called complete. Standards remains0hard findings with5optional GLM judgments; Spec has5supported findings (worst P1), with the stream-only granularity objection unconfirmed. Recommendation: fix R1–R3, add their maintained public regressions, close R4/R5, then run affected engine checks on the corrected candidate. Source remains unchanged for this read-only review. No commit, PR, issue mutation or publication occurred; U1–U7/provider acceptance and all original operational restrictions remain open.


## Post-review corrections and current evidence

Connor authorized these five bounded corrections with “Ok do it.” This section is the current local step-2 result; it supersedes the outstanding disposition in the prior review while preserving its findings and failed reproductions. No new independent review or Council was run for these fixes. The lead implemented, inspected and tested them; the four reviewer reports remain evidence about the pre-correction snapshot only.

### Guidance, authority and change path

Freshly resolved origin/main to890af2eaff4fa525163bc00339ce081dbeeee4bf, reloaded its immutable root AGENTS and verification/discovery procedure, and reused applicable linked guidance already read completely at that unchanged revision. Local root/task documents and the existing source contract remain applicable; no nested-instruction or consequential conflict appeared. Delivery refresh at2026-11-16 03:28UTC returned the same published commit. Selected code remains96d3ff3a591a3231c8ecf600c858292b0b2fe7e9 with authorized uncommitted work. Issue5 remainsOPEN, comments0, four unchecked criteria and native blockers3/4CLOSED by fresh read-only API checks. No issue mutation, ref/index change, commit or publication occurred.

Expected and actual correction route: source owner → existing normalized Admission/Validity/MarketBoundary evidence → public source/evaluator tests; constructor resource regression reuses the existing internal allocation observer in strategy.rs; owning source contract and this record updated. Existing code-map route still reaches all owners/tests; no entry point moved. No evaluator runtime code changed: stripping the one new constructor allocation test leaves strategy.rs byte-identical to its reviewed candidate. JSON/explanation, math/history, storage, native/UI, dependencies and provider profiles remain unchanged. Optional GLM cleanup suggestions and the unconfirmed finer-invalidation request were kept out of scope.

### R1–R5 resolution

- **R1 closed:** metadata-stream invalidation shares the retained market scope, revokes book/trade generations and all retained metadata verification, and advances the market revision even without a reference input. Old metadata/book completions reject; old market-dependent stamps fail. Existing normalized Admission blocks an active window with unverified metadata, so fresh book data alone cannot restore a candidate. Re-verification plus fresh evidence restores active-run use without repeating a boundary. Public metadata-loss timeline checks stream/source/provider/market/all scopes, stale callbacks, candidate removal/recovery and an unrelated trade-stream invalidation that preserves valid metadata.
- **R2 closed:** each market has a bounded private readiness-since timestamp, checked against readiness at the accepted clock before any renewal changes it. The first emitted eligible boundary requires readiness strictly before opening. First data, first metadata verification or recovery from an expired lease at opening cannot enter that window. A private source flag remembers its first eligible boundary emission, not evaluator completion; display-only promotion does not set it. Active recovery retains existing startup meaning. Readiness is updated before retirement so a skipped old window cannot erase a prepared successor's timestamp when slots compact. Public source/evaluator cases cover all three same-opening failures, a valid pre-open case, active recovery, and skipped/retired-window successor readiness.
- **R3 closed:** market-count admission occurs before source construction; the first borrowed descriptor is fully validated before its series ID is copied. The existing independent thread-local allocation observer now measures the public constructor with a synthetic2MiB invalid identity. Old requested-heap peak was2097613 bytes; corrected single-market rejection peak is461 bytes, while over-count registration rejects with0 measured allocation. These are controlled rejection measurements, not an allocator/RSS guarantee.
- **R4 closed:** never-received data reports the explicit Awaiting/awaiting_data reason. Received-but-expired data retains freshness loss; authenticated Access refusal keeps priority. Capability limitations remain separately visible, so awaiting does not grant support. Public snapshot timeline asserts all three reasons differ.
- **R5 closed:** the shared public registration parity test now accepts legal explicit YES with implicit NO and explicit NO with implicit YES in both SourceState and Evaluator. It retains illegal scope/contract checks and rejects both wrong implicit-side mappings. No provider capability is inferred from constructor acceptance.

### Regression and resource evidence

Before implementation changes, the three new public behavior tests failed on the actual selected source: metadata loss left an old stamp current; first data at opening emitted a boundary; awaiting and expired reasons were identical. The new internal public-constructor allocation check also failed before correction with peak2097613. The functional reason inequality was demonstrated before adding an assertion referencing the new Awaiting variant. The legal-implicit cases passed existing behavior and now provide maintained regression coverage. Corrected tests pass; no production bug was inferred from a fixture-only compilation failure.

Source retained inline sizes: owner272, Bound304, BindingView320, SourceRequirement88, StampEntry32 and Market112 bytes. The per-market timestamp adds16 bytes per allocated market slot (up to4), making the retained-owner bound **123416 bytes** (the prior123352 plus64), still below128KiB. View shape/256KiB construction guard, strings, binding caps and requirement bounds are unchanged. Constructor/input overlap, old64 binding slots during a128-slot growth, bounded validation clones and matching-reference scratch remain within the previously recorded conservative combined1MiB source/input/effect/view argument. Borrowed malformed identities are neither copied nor retained; the count/length checks precede proportional allocation.

Readiness snapshots use fixed two-bool arrays: ordinary events perform at most four preparation passes (two markets before/after mutation), and events delegating to invalidation perform at most six including the outer observation. Each pass examines at most34 requirements and128 bindings; matching-reference allocations are bounded/sequential and no historical cells, compiled strategy or elapsed-time walk are added. Admission examines at most two market descriptors. Metadata revision exhaustion fails closed; the source-counter test now pins that new branch. Existing evaluator4096-slice/8MiB admission, mask accounting9516 and history proofs are unchanged. This is a finite work/storage argument, not a latency/throughput qualification.

### Checks on the corrected candidate

- cargo test -p metten-engine --locked: **125 passed**, two release-only fixtures excluded as intended (25 unit,6 session,25 source,59 ordinary strategy,10 strategy-storage).
- cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture: **2 passed** on the corrected final source, satisfying the approved final-candidate gate. Each reported allocated_bytes65988608; registered-contract fixture3940ms, hour-average fixture4402ms. No performance comparison/production capacity claim follows.
- cargo fmt --all --check: passed.
- cargo clippy -p metten-engine --all-targets --locked -- -D warnings: passed.
- Affected contract/map/evidence paths, final whitespace, reviewed-base versus corrected diff and preservation of unrelated docs/README/index/HEAD: checked. Current source proofs and all existing independent work/allocation/continuation probes are included in the passing engine suite.

Corrected source identities:

- lib.rs:64e426574b46c8d684008988e690d85b061826527ecdc1fe60d06af66ab06d72
- strategy.rs:54e76b54a69526aacd710887ba017b6fe7f442467db005caae5969d006961322
- sources.rs:d59ead3524becd90fee9e285dfeb70e2fb40a1f48d0a7676a5ba52e0213b2f68
- tests/strategy.rs:2320ca107b96a33f68affb3e2271df36ff905ec6a4eb460d99531361bf57b364
- tests/sources.rs:1611633fd47072baa7785fbd99cae96c655c70b346d961a050197a0407585f77

### Existing Python skill-checker resolution

Connor explicitly allowed Python for skill tooling. An existing alternate interpreter /usr/bin/python3 resolves to /Library/Developer/CommandLineTools/usr/bin/python3 and already contains PyYAML6.0.3. Running the installed skill-creator quick_validate.py against code-review with that interpreter returned **Skill is valid!** This supersedes the earlier default-interpreter missing-PyYAML limitation. No Python/package installation, environment mutation, Metten dependency or additional skill edit was needed. The four-reviewer roster and shared-policy pointer remain as approved.

**Local completion:** R1–R5 are corrected with maintained checks; the controlled source component meets this bounded step's completion conditions. The prior four independent reports do not constitute a new review of these corrected hashes. Provider access/rights, live delivery, performance/capacity qualification, packaged/native integration, production dependency/publication coordination and trading remain unverified and outside this authorization. U1–U7 stay open. No account, credential, provider, app/browser or purchase endpoint was accessed; no commit or publication occurred.


## Ponytail review of corrected candidate

Owner requested ponytail-review with subagents. This is a read-only review of the corrected eight-file uncommitted change against96d3ff3a591a3231c8ecf600c858292b0b2fe7e9, including new files. It does not undo R1–R5 corrections; it supersedes the prior local completion conclusion with the new finding below. No implementation file changed.

Lead freshly resolved origin/main890af2eaff4fa525163bc00339ce081dbeeee4bf at2026-11-16 03:38UTC, reloaded root AGENTS and full verification/discovery, and used the unchanged applicable standards already read at that revision. Frozen eight-file diff/base/candidate manifest, complete immutable published guidance, recursive tree and fresh issue body/comments/native blockers were supplied to both new reviewers. Final lead query at03:50UTC still returned890af2e. Selected HEAD/index/refs and inherited docs/README/proposal work were preserved. Issue5 remainsOPEN, comments0 and blockers3/4CLOSED; no tracker writes.

Two separate Sol6.1 high-reasoning reviewers used the native Codex subscription. One covered correctness/safety/recovery; the other covered load/resource/test/lean behavior. Both independently traversed the pinned published root into verification/discovery, maintainability, map, domain/glossary, ADR0001, CONTRIBUTING/SECURITY, issue/triage and applicable local contracts/product/approved design/plan/source profiles/tests. Both matched all eight current hashes and approved plan/design identities. The correctness reviewer disclosed incidental exposure to historical review narrative during a broad evidence read; this is not claimed as a blind review. Neither used another current review as evidence. Neither executed tests or changed files.

Both reviewers reported no supported finding in their assigned lens. The correctness pass traced source intake through identity/units/clocks/capabilities, effects, evaluator admission/masks/completion, metadata revocation, initial/active readiness, gaps/replay/high-water, rejection/Busy/Stop and dependency stamps. Resource/test/lean pass checked cap/growth/retirement, constructor/caller capacities, simultaneous state/input/effect/view bounds, both128-entry masks/9516-byte reservation and sliced cleanup, actual branch reservations, public regression coverage and unnecessary layers/dependencies. No concrete lean deletion was identified. Oversized caller objects rejected before admission remain outside admitted-input inventory, as documented; that was not promoted into a defect.

Assumed workload: one synchronous local owner, at most two retained markets,128 bindings and34 requirements. Existing125ordinary/2release/fmt/clippy results were reused for exact source hashes d59ead3524becd90fee9e285dfeb70e2fb40a1f48d0a7676a5ba52e0213b2f68 (sources),54e76b54a69526aacd710887ba017b6fe7f442467db005caae5969d006961322 (strategy),1611633fd47072baa7785fbd99cae96c655c70b346d961a050197a0407585f77 (source tests) and unchanged other code. These passing cases do not cover the new startup case.

### Finding 1 — Must fix: current window opened before Go is rejected

**What this is:** The source owner retains the current market plus its immediate successor. Clock values are elapsed milliseconds since Go (approved step2 plan:76), while the existing MarketDescriptor uses signed open/close times; a current window may have opened before zero. The design requires current/next records and permits late current-market display without entering that missed window (market-data-design:333–344).

**Problem:** sources.rs:404 adds open_ms<0 rejection beyond the existing shared descriptor validator. A valid current descriptor open=-60000/close=840000 plus successor open=840000/close=1740000 is accepted by Evaluator::new but rejected by SourceState::new as identity_mismatch. This blocks the bounded current+next setup even though it neither imports pre-Go observations nor requests entry into the current window. Both source construction and later RegisterMarket call the same rejecting validate_market path. The source also casts descriptor times to u64 in admission, Boundary, retirement and usability, so removing that guard alone would mishandle these signed times.

**Fix:** Admit valid current-window metadata whose opening precedes Go and use the evaluator's existing signed-time comparison pattern for all source schedule comparisons (current locations568–569,983,987,1027,1218). Preserve first-full-window entry and post-Go-only history. Add a public current+next registration parity test with a negative current opening, late/display-only handling, next-boundary eligibility and expired-window retirement. No new clock framework, configuration or dependency is needed.

**If skipped:** A normal registration during an already-open market fails initialization or loses the required current-market display/preparation path. The current module is not yet wired to provider or financial execution, so this is a demonstrated controlled-boundary bug, not evidence of a live order problem.

Lead reproduced the mismatch in a disposable standalone copy of the exact frozen candidate with existing public helpers: market(m,-60000),market(n,840000), four correct YES/NO bindings, matching series and validated ask rules. Observed evaluator_accepts_current_and_next=true; source_accepts_current_and_next=false with SourceFailure codeidentity_mismatch. The assertion expecting source registration failed. Command: cargo test --offline --test review review_current_window -- --nocapture under Rust1.100.0 with selected lock copied into the temporary package. Only its temporary standalone lock resolution could change; selected Cargo.lock/source/index were untouched. Temporary source/test package was removed on exit. No provider, credentials, accounts, application or order endpoint was used.

**Verdict: fix1 first.** No Should fix/Nice to have or lean-count claim. Finding1 is the lead's independently reproduced result; the two subagent no-finding reports do not override it. No fixes were made in this review. Current evidence requires this correction before local step2 acceptance is restored; R1–R5 remain implemented. Provider/runtime throughput/native packaging/live behavior and U1–U7 remain unverified. No commit, PR or publication occurred.

## Startup signed-time correction and sol-loop

### Resumed baseline and expected path

Fresh remote query of authoritative https://github.com/moultonlavah/metten.git resolved HEAD to main at `890af2eaff4fa525163bc00339ce081dbeeee4bf`. Selected HEAD remains `96d3ff3a591a3231c8ecf600c858292b0b2fe7e9`; comparison base remains `47d0f384669f38aaf76e3c5bc50060836b235f02`. Query/session UTC: 2026-11-16T04:17:48.235346+00:00. Loaded published root AGENTS, verification/discovery, maintainability and code map independently from that immutable tree; its tree contains no nested AGENTS. Local root AGENTS, verification, code map, source contract, product specification, ADR 0001 and approved revision-5 plan apply. Published guidance adds discovery/maintainability to the older local entry; the owner's explicit approval supersedes the plan's historical pause wording. No consequential conflict or missing required guidance identified. This session received Connor's shared standing discovery rule directly; unrelated sessions are not claimed refreshed.

Expected correction path: SourceState shared constructor/RegisterMarket validation → admission, Boundary, retirement and binding usability → public source/evaluator regressions → source contract and this record. All descriptor-time/unsigned-clock comparisons will use i128::from, as the evaluator does. Elapsed clocks, public interfaces, evaluator calculations/history, configuration, fixtures and exports need no change. No added structure or dependencies; no entry point moves, so the existing code map remains valid. Preserve all other step-2 and unrelated work.

Observed failing-before evidence from the completed predecessor run: `/Users/connormoulton/.cargo/bin/cargo test -p metten-engine --locked --test sources before_go -- --nocapture` failed both new tests at SourceState::new with identity_mismatch (25 filtered). Inspected their assertions: current registration parity, unverified active metadata admission, display without boundary/candidate, prepared next exact boundary/candidate, retirement and stale stamps; closed negative-time windows cannot be usable.

### Correction and covering checks

Removed only the source's extra negative-opening guard; shared descriptor identity, duration, duplicate and overlap validation remain. All six schedule comparisons now widen signed descriptors and unsigned clocks with i128::from, covering admission, Boundary ordering/equality, retirement and usability. No storage/state fields, allocations, loops or evaluator code changed; existing bounded resource arguments remain applicable. The two pre-existing new tests needed no behavioral correction.

Checks on this corrected candidate:

- Targeted `cargo test -p metten-engine --locked --test sources before_go -- --nocapture`: 2 passed,25 filtered.
- `cargo test -p metten-engine --locked`: 127 passed (25 unit,6 session,27 source,59 ordinary strategy,10 storage);2 release-only fixtures ignored. The entire source integration suite passes within this run.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy -p metten-engine --all-targets --locked -- -D warnings`: passed.
- `cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture`:2 passed; allocated_bytes65988608 each; registered-contract3894ms, hour-average4381ms. Controlled fixture measurements only.
- Tracked whitespace check passed; approved plan/design hashes still match. Source caller tracing covers constructor and RegisterMarket → validate_market, effect/accepted → admission, Boundary/Advance → retire_expired, snapshot/preparation → usable. No maintained boundary moved.

Fresh read-only issue5 check: OPEN, no comments/assignee, ready-for-agent/product/priority:high, four unchecked criteria; native blockers3/4 closed. No issue or Project writes are authorized. Published domain, glossary, issue/triage, CONTRIBUTING and SECURITY guidance also read; routine tracker mutations remain excluded by owner scope. No provider/library mechanism change requires external API research.

Candidate SHA-256 (evidence file excluded to avoid a self-referential hash):

- `crates/engine/src/sources.rs`: `a85e123a40fcb1089fcb8e2559dc0f4aeb63bac452ea9efddf01ddf4f7467e36`
- `crates/engine/tests/sources.rs`: `07711dc130ae9b5f4b70126e6f67a64500ce7e779d2fdd4228e3419cf6661d3a`
- `crates/engine/src/README.md`: `be837487dd73e998716936d37eb7a7522bb724016073c287b85c39e7bca13c85`

### Sol-loop ledger and delivery

Round1 of3: **APPROVE**, no blocker/should/nit findings. Fixed: confirmed startup Finding1 above. Sol-requested fixes: none. Declined findings: none. No further round required; behavior files stayed frozen during review and their hashes above remain unchanged at delivery. After approval only this evidence record changed to record the outcome. Lead has no remaining in-scope concern.

Reviewer: Codex / gpt-6.1-sol, high reasoningEffort (returned orchestration options and review receipt); token/cost usage not exposed. TaskId: `node:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-startup-sol-round1-20261116-unique1`. [Completed review](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/thread:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-startup-sol-round1-20261116-unique1). Reviewer independently ran all27 source tests and all127 ordinary engine tests successfully. It read the complete untracked source/tests and contract/evidence, traced the original evaluator validation/boundary/history behavior, verified before-snapshot manifest hashes and candidate/approved document identities. Other fmt/clippy/release/failing-before evidence remains the lead's recorded evidence, not attributed to reviewer execution.

Reviewer independently resolved remote/default branch, read pinned root AGENTS at890af2eaff4fa525163bc00339ce081dbeeee4bf, traversed published discovery/verification, maintainability, map, domain/glossary, ADR0001, CONTRIBUTING, issue/triage and SECURITY plus local contracts and relevant approved product/design/plan. Only root AGENTS found in published/local trees; no missing required guidance or consequential conflict. Reviewer delivery query04:21UTC returned the same revision. This establishes activation for this lead and reviewer only. Lead final fresh remote query also returned the same revision at 2026-11-16T04:22:28.303378+00:00; guidance is unchanged, so no affected re-review.

Narrow incremental diff before review → after review: production +14/−14, tests +103/−0, contract +1/−1 unchanged; evidence at review +30/−1, then documentation-only ledger/index updates. This is distinct from the inherited uncommitted step2 implementation. The fixed comparison command remains `git diff 47d0f384669f38aaf76e3c5bc50060836b235f02 -- crates/engine/src/sources.rs crates/engine/tests/sources.rs crates/engine/src/README.md docs/verification/issue-5.md`; untracked source/tests/evidence were explicitly read and compared to the saved before files because ordinary git diff omits them.

No commits, staging, pushes, issue changes, provider/account/credential access, app launch or live tests. Existing step2 implementation, unrelated docs/README edit and all proposal/review artifacts remain. U1–U7, provider delivery/live acceptance, production performance and legal clearance remain unresolved/outside scope. Controlled engine approval does not establish provider support or broader issue5 acceptance.

Final incremental size (including delivery evidence): production +14/−14, tests +103/−0, contract +1/−1, evidence +47/−2: total **+165/−17** across four files. Review added no behavior code.

## Requested two-axis code review

Review baseline 2026-11-16T04:42:10.818458+00:00: origin/main freshly resolved to `fa3663624436e9e4df9d2b000d57f7f21e099786`; selected HEAD `96d3ff3a591a3231c8ecf600c858292b0b2fe7e9`, established base `47d0f384669f38aaf76e3c5bc50060836b235f02`. Lead loaded immutable root AGENTS and verification/discovery. Applicable linked maintainability/map/domain/issue/triage standards are byte-identical to the previously read890af2e revision; CONTRIBUTING's changed security-enforcement status is read and does not alter this engine-only review. No nested published/local AGENTS. Reuse unchanged local instructions/contracts, approved plan/design/product and prior current-source checks; reviewers independently traverse the new pinned entry. No new consequential conflict. Owner requested four independent slots: Standards Sol+GLM, Spec separate Sol+DeepSeek. Read-only review of the same four paths; full source/test untracked contents explicitly included. The three-dot committed diff contains only preceding source-profile documents, so working-tree/untracked comparisons augment it to avoid silently reviewing an empty implementation diff. Other step2 code is context, excluded from new implementation changes. No code edits authorized by this review.

### Four-reviewer result and lead disposition

All four independent slots completed against the frozen four-path candidate: Standards Sol(gpt-6.1-sol,high) and GLM(opencode/glm-5.3-flash-free,high,plan); Spec separate Sol(gpt-6.1-sol,high) and DeepSeek(opencode/deepseek-v4.1-flash-free,high,plan). Free models used configured OpenCode Free Trial; Sol used existing Codex subscription. Usage/cost was not exposed. Candidate manifest hashes matched before this documentation-only aggregation; production/test/contract files were not edited. Reviewer source pointers included historical review narratives in the mandatory evidence file; reviewers were instructed to form independent conclusions and received no other new reviewer's findings before completing.

Standards: both reviewers found no hard documented-standard breach. **ST1,P3 optional Duplicated Code**: both confirmed the owned BindingDescriptor input/scope capacity policy is repeated at sources.rs647–656 and897–909. Lead verified both paths; optional private predicate could centralize a future policy change, but no current acceptance failure warrants a refactor. GLM-only suggestions for a generic counter helper, naming Bound/provider-specific streams, grouping provenance fields, enum indexing and optional-vs-concrete deadline representation were declined as unsupported current maintenance defects. In particular, counter failure paths differ between ordinary pre-mutation refusal and immediate revocation; they are not uniformly the same helper. No lean line-saving claim.

Spec: Sol found no supported defect. **SP1,P2 confirmed — late metadata verification does not update the current-market display without another Boundary event.** DeepSeek identified the original-source path; the lead reproduced it independently. Plan76 requires an on-time opening → ordinary advance → late verification with no boundary timeline; plan126 says “Late verification updates display without backdated entry.” Metadata verified:true at sources.rs961–968 only updates verified/market_revision. current is assigned only in Boundary1003–1008, not Metadata or Advance. Existing tests verify first and then supply a late Boundary; they do not cover opening-before-verification with no repeated Boundary. The prior wording “display without boundary/candidate” meant no emitted evaluator boundary transaction, but did include a source Boundary event; it does not prove this missing sequence.

Lead controlled reproduction used a temporary standalone test package depending on the exact selected engine path, copied source-test helpers and selected lockfile. Only the temporary package's lock resolution could differ; selected Cargo.lock/index/source unchanged. Command: `/Users/connormoulton/.cargo/bin/cargo test --offline --manifest-path /var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/metten-late-metadata-review-h05pluiv/Cargo.toml --test review reviewer_late_metadata -- --nocapture`. Observed **1 failed,27 filtered; Cargo exit101**. Sequence: initialize at0; unverified opening at1 emits no entry; Advance2; verify(m,2) returns no transactions and no evaluator candidate; market verified and Preparation::Ready assertions pass; current_market expectedSome(m), actualNone fails. This is an actual current-source failure, not a simulated fault. No source regression was added by this read-only review.

Recommended bounded correction: when metadata verifies an active [open,close) market, update display identity under existing revision fencing without setting entry flags or emitting MarketBoundary; preserve readiness/preparation and all no-late-entry checks. Add the reproduced public source/evaluator timeline and check both no backdated candidate and later proper full-window entry. Examine Boundary and Metadata display assignment together; do not add a runtime timer workaround or framework. No fix is applied or newly authorized by this report. The signed-time fix remains valid; this defect belongs to the inherited late-display path.

This supported finding supersedes the lead's interim Ponytail “Looks good/Ship” conclusion and restores an outstanding local step2 acceptance item. It is not evidence of unsafe live orders: the reproduced timeline produces no candidate, and no providers/trading are connected. All127 existing ordinary checks/two release fixtures/fmt/clippy remain valid for the covered unchanged behavior; those tests omit SP1.

All four reviewers independently loaded pinned root AGENTS fa3663624436e9e4df9d2b000d57f7f21e099786 and traversed applicable published discovery/verification, maintainability, map, domain/glossary, ADR0001, CONTRIBUTING, issue/triage and SECURITY; no nested AGENTS or unavailable required published standard. Both Sol roles and DeepSeek read applicable product/approved task guidance; GLM reported limited product-depth and sampled surrounding evaluator code rather than a full evaluator diff review. Preserve that Standards-slot coverage limit rather than claiming complete program correctness. No models substituted, no role omitted. Lead delivery refresh 2026-11-16T05:01:16.095938+00:00 returned unchanged origin/main fa3663624436e9e4df9d2b000d57f7f21e099786; no guidance-triggered re-review required.

[Standards GLM report](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/thread:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-cr-standards-glm-20261116-1). [Spec DeepSeek report](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/thread:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-cr-spec-deepseek-20261116-1). Native Sol slots /root/standards_sol and /root/spec_sol completed independently; native tool exposed no thread links. No code changes, commits, pushes or tracker/provider/account/credential/app actions. U1–U7 remain unresolved.

## SP1 late-verification display correction and sol-loop

Owner authorized “Fix it with $sol-loop.” Fresh origin/main query 2026-11-16T05:14:03.103556+00:00 resolved fa3663624436e9e4df9d2b000d57f7f21e099786; selected HEAD96d3ff3a591a3231c8ecf600c858292b0b2fe7e9 and comparison base47d0f384669f38aaf76e3c5bc50060836b235f02 retained. Reloaded immutable root AGENTS and discovery/verification; linked maintainability, map, domain/glossary, ADR0001, CONTRIBUTING, issue/triage/SECURITY are unchanged from this session's completed read. Local instructions, source contract, approved revision5 plan/design/product remain loaded; published/local inventory has only root AGENTS. No missing guidance or consequential conflict; owner exact scope overrides old pause/tracker writes. Reviewer must independently traverse same pinned entry.

Expected path: Metadata and Boundary share the existing current-display identity, while Boundary alone owns entry emission/deduplication. Correct active signed-schedule display on metadata verification without setting entry flags. Trace all current/entered assignments and effect/accepted/stamp callers; preserve pre-open/closed exclusion, postGo history, initial readiness, exact boundary and duplicate suppression. Public source/evaluator regressions plus contract/evidence only; no map/exports/config/fixtures/dependencies or state fields need change. Optional ST1/helper/style work stays excluded. Four scoped before files saved at /var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/metten-late-display-sol-pxauf6fk before tests/fix; all other work preserved. Existing SP1 failed reproduction above remains actual failing-before evidence.

### Implementation and checks before Sol round1

SP1 implementation: successful Metadata verification selects its active signed [open,close) descriptor as the current display identity using existing metadata revision fencing; it emits no transaction and does not set entry flags. Future/closed descriptors do not select display. Boundary now records entered only when enter is true, independently of whether display identity already matches. This is required to preserve exactly-once entry when metadata has already selected display; no helper/field/dependency/config or evaluator change is added. Seven added/one removed production lines; existing storage/work bounds are unchanged (constant schedule comparisons and one already-bounded128-byte current identity).

Before production correction, maintained `cargo test -p metten-engine --locked --test sources metadata_ -- --nocapture` observed2failed(new regressions),2passed(existing),25filtered: both new failures expectedSome(m),actualNone. Source/evaluator tests now cover opening before metadata → Advance → late verify without another Boundary → display selected and no transaction/candidate; old metadata stamp obsolete; late duplicate Boundary still emits nothing. Second test checks pre-open verification doesn't select display, active verification alone cannot enter, and eligible exact opening emits only once even when already displayed. Strengthened existing negative-open current+next test checks verification alone selects current and verifying the future successor doesn't replace it; real next-full-window evaluator candidate remains covered. Closed-beforeGo test also asserts no current display.

Current checks:

- `cargo test -p metten-engine --locked --test sources -- --nocapture`: **29passed**.
- `cargo test -p metten-engine --locked`: **129passed** (25unit,6session,29source,59ordinarystrategy,10storage),2release-only fixtures ignored.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy -p metten-engine --all-targets --locked -- -D warnings`: passed.
- `cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture`: **2passed**; allocated_bytes65988608 each; registered-contract3946ms, hour-average4370ms. Controlled local fixture only.
- Read-only issue refresh: OPEN,0comments,native blockers3/4closed. No tracker writes.

Candidate SHA256 (evidence file excluded to avoid self-reference):

- `crates/engine/src/sources.rs`: `3cf6870dd879fc0e34bdbdb70ccb75741292d91fefe9055039a27b8fa7e9e667`
- `crates/engine/tests/sources.rs`: `2c26bcab830c2fea047f1ca65c8459531e1af3102329cabdc6704e98a02ba172`
- `crates/engine/src/README.md`: `978bc8b8d5994ef95069a4f1b0252ecbddfa90b3321e539e1620a8292d397e69`

### SP1 Sol-loop approval and delivery

Round1/3: **APPROVE**, no blocker/should/nit findings. SP1 fixed; Sol-requested behavior changes:none; declined findings:none. Optional ST1 capacity-check refactor remains outside this correction. Lead has no remaining in-scope concern; no further round needed. Code/tests/contract stayed byte-identical through approval and final hash check; only this evidence record changes afterward. Narrow size before review → after review stays production+7/−1,tests+93/−0,contract+1/−1; evidence at review+28/−1, followed by documentation-only delivery ledger. No added state fields, interface, dependency or framework.

Reviewer Codex/gpt-6.1-sol,high reasoningEffort confirmed by returned options and review receipt; token/cost usage **not exposed**. TaskId `node:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-sp1-display-sol-r1-20261116-unique1`. [Completed Sol review](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/thread:delegated-task:command%3Amcp%3A3a025c49-1dd6-4fe6-bba9-e3474327cd96%3Adelegate-task%3Ametten-sp1-display-sol-r1-20261116-unique1). Reviewer independently ran29source tests successfully, read the complete untracked files and surrounding evaluator admission/boundary/ready_at logic, and verified the before/candidate/approved-document hashes and exact patch. It inspected the129ordinary-test log; fmt/clippy/release results remain lead-executed evidence.

Reviewer independently resolved remote/default branch and loaded pinned root AGENTS atfa3663624436e9e4df9d2b000d57f7f21e099786, traversing applicable verification/discovery, maintainability, map, domain/glossary, ADR0001, CONTRIBUTING, issue/triage and SECURITY, local source contract/product/revision5 plan/relevant design. Published/local inventory root-only; no extra ancestor instructions, missing applicable guidance or consequential conflict. Reviewer delivery refresh2026-11-16T05:18:38Z returned same guidance; lead final query 2026-11-16T05:19:33.208869+00:00 also returned same origin/main revision. Guidance unchanged, so passing review/checks reused without repetition. Receipt applies only to this lead/reviewer, not unrelated sessions.

This restores bounded local step2 acceptance for SP1; broader issue5/provider acceptance remains incomplete. All existing implementation, unrelated docs/README edit and proposal/review artifacts preserved. No staging, commits, pushes, tracker writes, credentials, provider/account calls, application launch or live trading. Provider delivery/live acceptance, production performance, legal clearance and U1–U7 remain unresolved/outside authorization.

## Owner-requested checkpoint for a fresh reviewer

Connor explicitly authorized “commit your work” and requested a RISEN code-review handoff. This adds local commit authority only, not push/PR/provider/live authority. Fresh origin/main query 2026-11-16T05:22:29.634744+00:00 returned fa3663624436e9e4df9d2b000d57f7f21e099786; reloaded pinned root/discovery/staged-check procedure and reused unchanged applicable guidance. Stage only the approved eight implementation/evidence paths; preserve unrelated docs/README and all untracked proposal/review artifacts outside this checkpoint. Those task documents therefore require this same working copy for the fresh review; the commit alone does not contain their historical approval artifacts.

`python3 scripts/check-staged.py /Users/connormoulton/.cargo/bin/cargo test -p metten-engine --locked` passed against staged tree **46ddfa861c3ea8c19380083e0d8efdf12e22c203**:129ordinary tests,2release-only ignored. The temporary exported tree proves selected code/tests compile and pass without unstaged files. Prior current-source release-hour/fmt/clippy/Sol evidence remains applicable to the byte-identical behavior files. After that pass only this checkpoint receipt was appended; final staging rechecks receipt/whitespace and verifies all other staged blobs unchanged instead of rerunning unchanged engine behavior. No review or implementation for the fresh session is executed here.

## Step 3 implementation baseline

Active session refreshed published instructions on 2026-11-16 at approximately 05:45 UTC. Selected code remains `875357ca1ab502d99b03b83193ed1e5526af9d51`, branch `t3/brainstorm-issue-5`. Fresh `git ls-remote --symref origin HEAD` resolves authoritative `https://github.com/moultonlavah/metten.git`, default `main`, guidance `fa3663624436e9e4df9d2b000d57f7f21e099786`. Loaded immutable root AGENTS, verification/discovery, maintainability, code-map, issue-tracker and CONTRIBUTING. Published tree has no nested AGENTS. Local root/standing instructions apply; preserve unrelated docs/README and every untracked proposal artifact. No staging, commits or publication authorized.

Approved design and revision-5 step-2 plan SHA-256 match the handoff exactly. Issue #5 full body/comments/native blockers read with gh: open, four unchecked criteria, no comments, #3/#4 closed, no children, parent endpoint 404 (no parent). Queue currently Backlog, unassigned; no external messages/mutations authorized. Step 3 does not complete issue acceptance. U1–U7 remain open.

Expected change path: native provider credential owner -> parsed finite signing -> fixed-host transport and shared budgets -> bounded admission/generation fence -> owned task completion; existing Quit waits for confirmed provider completion before strategy/credential/engine save. Engine, SQLite, strategy, UI and command permissions unchanged. New modules are required security/transport owners named in the approved inventory, with controlled tests at their Rust boundary. Step-4 decoding and step-5 commands/fixture IPC are excluded.

### Step-3 behavior, review and verification

Implemented native paths: strict fixed-item provider records and deterministic status/readback; Ed25519 PKCS#8 and 2048-bit RSA PKCS#1/PKCS#8 signing; finite fixed-host GET/control/handshake manifest; bounded REST/WebSocket framing; shared local traffic budgets; bounded HTTP retry with cancellation; full-queue revocation; fixed provider task slots; an optional owned synchronous worker; provider-specific cancellation and joined Kalshi/BRTI revocation before replace/remove; generation fences; retained handles and explicit recovery. Quit owns the coordinator outside its mutex while joining against one total 5-second deadline, and refuses engine save when completion is unconfirmed. No command, screen path or startup action can start these connections yet.

Lead source review caught and corrected: repeated awaiting of an already-consumed task handle; a shared lock held across source joins; signing before budget wait; interrupted socket reuse; independent-provider cancellation during credential changes; unbounded serialization before record-byte admission; and a test-only Core Foundation handle making Workspace non-Send. Initial compilation/lint failures and test fixture edits are superseded by the passing checks below, not independent review findings. No delegate or model review was requested/started.

Actual checks (absolute cargo path throughout):

- `cargo test -p metten --locked`: final **25 passed, 1 ignored**; the ignored native Keychain check was run separately. Covers offline signature verification for both RSA encodings and Ed25519, unsupported/malformed key refusal, strict/sanitized records, failed replacement/removal preservation, deterministic environment isolation, readback mismatch, exact fixed controls, actual local-peer outgoing subscription, redirect/body/frame rejection, full-queue/obsolete-generation cancellation, stopped retry, task/worker exit, retained unresponsive-worker ownership and explicit recovery. Replacement test verifies validation precedes revocation, the shared Kalshi task has actually exited before mutation, and Coinbase's fence remains current.
- `cargo test --workspace --locked`: passed earlier in this step (**129 engine tests plus then-current 22 native tests**, two engine release-hour tests and one native explicit-only test omitted as intended). Later edits affect native code only; final native tests above supersede that native count. Existing two passing release-hour fixtures are reused for byte-identical engine code.
- `cargo test -p metten --locked isolated_native_fixed_items -- --ignored --nocapture`: passed using only a newly created/deleted owned disposable file Keychain. Fixed demo/production item isolation, native duplicate refusal, whole-record update/read/delete and missing-delete outcomes passed. This is native adapter evidence, NOT packaged data-protection-Keychain evidence. No normal Keychain item was read or modified.
- `cargo clippy -p metten --all-targets --locked -- -D warnings`: passed on final native code. `cargo fmt --all --check` and `git diff --check`: passed before final evidence-only edits.
- `python3 tests/credential-runner.test.py`: passed its existing process recovery/retained-child/marker-output regression.
- `npm run build`: initially blocked by absent installed `tsc`; `npm ci` installed the existing lockfile without modifying it, then build passed. No dependency upgrade was used to clear that failure.
- `npm run tauri build -- --bundles app` with `/Users/connormoulton/.cargo/bin` in PATH: built the local release Mac bundle; the final incremental rebuild is recorded below. A build is not an interactive credential acceptance check.

Packaged dummy check attempted with `swift scripts/verify-credential.swift target/release/bundle/macos/Metten.app`. **FAILED/BLOCKED** at its initial expected status `Local credential readability unchecked.`: the app rendered, but its WebKit content was absent from the native accessibility tree, so the runner could neither observe status nor drive its ordinary quit control. The runner correctly retained child/resource ownership instead of declaring cleanup. Used the supported native app connector's actual Quit Metten menu. The runner then observed late process exit, deleted its exact disposable Keychain/directory, and confirmed unchanged default/search-list metadata. A subsequent process inventory found no Metten test process. The failed walkthrough remains failed; late cleanup does not clear it. No repeated launch, force-kill, user-browser action or normal Keychain operation occurred.

**Outstanding acceptance evidence:** the packaged provider-credential/bootstrap walkthrough cannot run through this app's current command surface: the approved sequence adds those commands and source-mode runner in step 5, which this task excludes. The direct disposable native adapter check is the strongest current substitute; it does not establish the production data-protection-store attributes/entitlements or packaged denial/prompt behavior. The existing dummy walkthrough is independently blocked by inaccessible WebKit controls. Next engineering action: repair/establish the isolated packaged accessibility/bootstrap route and complete those checks before accepting the changed native boundary. Do not silently treat unit tests or a bundle build as that proof.

Additional limits: exact full request-manifest attempt ledger under every cancellation race, partial-frame/partial-send races, body-stream deadline failures, all handshake failure classifications, worker age/combined adapter allocation charging, and the full simultaneous 32 MiB accounting profile do not yet have the requested comprehensive evidence. Counters currently count admitted attempts, which can include a request canceled before confirmed transmission; they must not be reported as measured outgoing provider traffic. Socket reconstruction/reconnect orchestration, discovery coalescing/page counts, decoding and source-state integration remain unimplemented for step 4. No source-support, latency/capacity qualification, account entitlement, legal clearance or live acceptance follows. U1–U7 stay unresolved. **Do not mark step 3's complete verification gate passed or begin step 4 on this evidence.**

Dependency review: Rust 1.100.0 stays pinned. Lock comparison removes/upgrades no existing `(name, version)` pair; 56 necessary transitive packages are added. Direct signing/transport identities: ed25519-dalek 2.2.0 (BSD-3-Clause), rsa 0.9.10 (MIT/Apache-2.0), sha2 0.10.9, base64 0.22.1, Tokio 1.54.1, reqwest 0.13.5 and tokio-tungstenite/tungstenite 0.30.0. Crypto PEM/SHA-256 and native-TLS/async features are selected in the native manifest, Tokio virtual-time features only for tests. `cargo metadata --locked` reviewed all 56 added package versions/licenses/MSRVs: MIT, Apache-2.0 and BSD alternatives; declared MSRVs <=1.85 (some crates omit an MSRV). This is compatibility/license inventory, not a security audit. No network/secret dependency or file changed in the engine crate.

Primary mechanism sources consulted 2026-11-16, supplemented by exact downloaded source: [Tokio select cancellation](https://docs.rs/tokio/1.54.1/tokio/macro.select.html), [Tokio shutdown](https://tokio.rs/tokio/topics/shutdown), [reqwest ClientBuilder](https://docs.rs/reqwest/0.13.5/reqwest/struct.ClientBuilder.html), [tokio-tungstenite](https://docs.rs/tokio-tungstenite/0.30.0/tokio_tungstenite/), [Ed25519 signing/PKCS#8](https://docs.rs/ed25519-dalek/2.2.0/ed25519_dalek/struct.SigningKey.html), [RSA source](https://docs.rs/crate/rsa/0.9.10/source/src/pss/signing_key.rs), [Kalshi canonical signing/key types](https://docs.kalshi.com/getting_started/api_keys), [fixed WebSocket controls](https://docs.kalshi.com/websockets/websocket-connection), [BRTI indexlist action](https://docs.kalshi.com/websockets/cfbenchmarks-value), [Coinbase Exchange channels](https://docs.cdp.coinbase.com/exchange/websocket-feed/channels), [Kraken v2 trade](https://docs.kraken.com/exchange/api-reference/spot-websocket-v2/trade). Apple documentation page required JavaScript; inspected installed Security bindings and native behavior, but packaged protection semantics remain unverified. The RSA HTML documentation fetch failed; exact downloaded 0.9.10 source establishes digest-size PSS salt.

Delivery guidance refresh at 2026-11-16 06:02:28 UTC again returned `fa3663624436e9e4df9d2b000d57f7f21e099786` on origin/main. No guidance change; reused consulted guidance. No commits/staging/push/PR/issue mutations; HEAD/index preserved. Unrelated docs/README and all proposal/review artifacts preserved.

Tested native source identities (SHA-256; evidence/contract-only edits are outside these behavior hashes):

- `Cargo.lock`: `82575c1298d20e259660d0efddce8deda3876bfed17b1c47ca29d662d7e5df86`
- `src-tauri/Cargo.toml`: `933b0ca44b41e08a6f0b267ab2da68814914201834c47974a2d8c27df15d4264`
- `src-tauri/src/main.rs`: `abe629bc4dc1ab6168947777d7084ad42df29147855f6f35a14e14fa65032a1f`
- `src-tauri/src/provider_credential.rs`: `dc6930a4b58a798c88d4ddbf46a246d4425124862056bfc478e42e3747941836`
- `src-tauri/src/providers/budget.rs`: `b96e949424f12c226f3b0336542cc4d310c75ab835009b89670ff99553ca655c`
- `src-tauri/src/providers/mod.rs`: `9a5364f29eaae0da90d8bdc97e7370424c51614e9bc7572cc76eb3fc3722f067`
- `src-tauri/src/providers/signing.rs`: `b78b1228adb1b5d26ddb3935657c19054f6f6322830b239fd21ce33f1f5b9be5`
- `src-tauri/src/providers/tests.rs`: `5adc368ce584ec0dad23322d823715208f51f804f40c8456cd1027a283075286`
- `src-tauri/src/providers/transport.rs`: `5f327f16c940a474060e9609c198ced0783660d1ee24a8b8ff6abe4c22387931`

Final incremental bundle rebuild passed after the provider-specific cancellation/credential-stop changes: `target/release/bundle/macos/Metten.app`, 13.29 MiB; executable SHA-256 `f321b19c5f5217321c7487d491aeee40a95f38b0fa81b7d6b27c672eea5d16e6`. That rebuilt bundle was not relaunched; the failed accessibility walkthrough applies to the preceding build and is not relabeled as evidence for this one. Final fmt/whitespace checks pass; HEAD remains `875357ca1ab502d99b03b83193ed1e5526af9d51`, index empty, both approved-document hashes unchanged.

## Step-3 completion continuation

Owner explicitly invoked `implement` to finish step 3, including local commit
authority. Published origin/main freshly queried again at continuation and remains
fa3663624436e9e4df9d2b000d57f7f21e099786; reloaded root entry/discovery/maintainability,
issue workflow, code map/contracts, glossary and applicable implement/TDD/review
skills. No nested published instruction files; local standing instructions apply.
Read-only tracker refresh: #5 open/no comments, #3/#4 closed. No authority for
push/PR/account/provider calls. CI for a new local-only head cannot be observed
without publication; it will be reported unavailable rather than inferred.

Pre-agreed seams are native credentials, finite transport, intake/stop ownership
and the isolated packaged process. Review base is the handoff's step-2 commit
875357ca1ab502d99b03b83193ed1e5526af9d51. Expected remaining path: refund unused
traffic reservations and count first-polled transport attempts; shared payload
reservations/queue age; cancellation through fence revocation; controlled partial
I/O, deadlines and failure classifications; packaged native credentials through
the existing disposable target, without UI commands or data decoding.

TDD: reservation regression initially failed to compile because its required
Permit boundary did not exist; implemented refundable reservation/start accounting.
The expired-work regression reproduced `Ok(())` from an already-expired deadline;
fixed admission and completion checks. Controlled peers now verify real GET paths,
failed/retried attempt counts, denial without retry, body-stream timeout, partial
frame discard and partial control-send cancellation. Buffer permits survive
queue/worker handoff, and stale intake is revoked. Recovery uses each message's
actual fence rather than a worker's frozen startup array.

The packaged native provider check passed on its first built candidate: actual
save/duplicate refusal/replace/status/trusted signing/remove for both fixed
environments, ack and actual process exit, output/file PEM-marker scans and owned
cleanup with unchanged default/search-list metadata. It reuses the same validated
Bootstrap and file-Keychain boundary; no generic secret read or ordinary-store
lookup is added. Its early sources-mode process exits instead of connecting.

The unchanged dummy UI walkthrough was retried once while investigating the
accessibility limitation and failed at the same initial status: no WebKit content
in its AX tree. An attempted owned-process accessibility request did not fix it
and was removed. Supported native Quit menu closed only the owned app; runner
confirmed late exit, owned Keychain/directory deletion and unchanged metadata.
This remains failed/missing UI evidence, not a passing check. Packaged native
provider verification is a separate implemented check, not a relabeling of that
failure. No further UI launch will be repeated without new evidence.

Required code review and final regression are pending in this continuation.

### Required review: findings and corrections

Review was against immutable staged trees: initial `4b902e5b39118d4afbddb3d13f52509f5f64968b`,
then corrected `059779327eef547efad62c6233faf6f5975316b2`; base is step-2 commit
875357c. WIP comparison uses `git diff <base> <tree>` (no new commit history),
so review can precede the authorized final commit. No reviewer received another
reviewer's first-pass findings.

**Standards — Sol:** found cancellation cleanup skipped when a caller drops a
public socket future, and provider-task exit leaving evidence current. Both
reproduced in controlled public-boundary regressions before correction. Socket
operation Drop guards now close/revoke; task lifetime guards revoke on return,
panic or cancellation. Full affected-flow audit also covered worker exit/panic,
which revokes the shared intake gate and requires joined recovery. Tests exercise
outer-canceled receive/send, early/panicking providers and worker failure.

**Standards — GLM:** first pass omitted required guidance and incorrectly claimed
abort detached a handle. The exact timeout branch returns StillStopping before
removal; a nonresponsive asynchronous-task regression proves ownership/recovery
blocking until observed exit. Its fresh full-coverage pass confirmed the retained
handle flow and read the omitted sources. Supported clarity fixes: share bounded
wait and Bootstrap activation, remove a redundant control guard, document counter
ordering and post-timeout rejoin, consolidate Quit failure handling. Suggestions
for a generic retry abstraction are not required: one owned retry state now
provides all pacing, while GET/handshake protocol-specific operations stay local.

**Spec — Sol:** independently found the same caller-drop cancellation defect and
accepted the narrowly stated packaged native isolation boundary. It explicitly
did not establish ordinary data-protection-store/prompt/GUI acceptance.

**Spec — DeepSeek:** verified the original numeric policy at design lines 354–358,
and found the initial 20-token burst, 1/2-second retries, handshake bucket and
pong accounting inconsistent with it. The lead had missed that detailed policy
in the earlier source read; code and tests are now corrected to the approved
50-token burst/10 tokens per second, at most two metadata operations, one handshake
in flight/provider, five-second exponential retry start capped at 60 seconds,
five-failure slow retry and 30-second sustained-usable reset. Request/handshake
waits have ten-second total caps. Pongs/data messages and obsolete admission
reasons have separate bounded accounting. The new runner mode now has process-only
mode/bootstrap/acknowledgment failure regressions, not just the existing dummy
self-test. The future provider-frame dispatcher protocol remains step 5.

The 2-second raw queue-age ceiling is a provisional buffering limit, not a price
lease; the 4-MiB per-socket reservation covers bounded frame, fragmented message,
write and protocol/read headroom. These are the design's required finite ownership
limits, with workload/actual allocation qualification still assigned to step 6.
A new virtual-clock regression also reproduced deferred permits accumulating a
later burst; reservation now holds capacity and actual start spends tokens, so
unused reservations neither inflate counts nor bank future traffic.

All four required roles ran (Sol Standards/Spec through the existing subscription;
GLM and DeepSeek through configured free trial models). Reviews are static evidence,
not runtime acceptance. A targeted correction review and final suite follow below.

### Correction-flow review and final candidate

A fresh Sol correction reviewer independently refreshed the same published entry
and traced the entire changed credential/transport/budget/task/worker/runner flow
in staged tree `1e9979d8546f6f57b2137c3dbba53a5c8a17ebe0`. It found no additional
cleanup/ownership/isolation defect, but reproduced by source analysis a shared
retry-admission gap: waiters could snapshot an earlier readiness time, and a
successful slow-mode attempt did not claim its next minute. The lead removed
snapshot-based admission. Actual attempt start now atomically rechecks and claims
the shared Retry slot after connection-capacity admission, releasing the mutex
before waits. Slow mode claims its next minute even after success, until the
verified 30-second usability reset. Controlled concurrent-handshake and virtual
slow-mode success regressions cover the complete failure class; GET and handshake
use this same actual-start owner. No second retry loop or per-operation state
reset remains.

Final pre-regression native checks: 47 ordinary tests passed, one native test
explicit-only; workspace Clippy passed. Runner process-only regression passed,
including finite mode selection, exact bootstrap mode shape, rejected/late/closed
acknowledgments and retained child/output cleanup. Initial closed-peer setup
failed its socket-deadline operation before reading EOF; that is now the same
finite private-channel failure category, still refusing acknowledgment/success.
No check was weakened to accept a failed child or missing exit.

Source/profile rationale is explicit: 2-second raw buffering is provisional and
unrelated to provider freshness; socket charge reserves frame/fragmented-message/
write bounds plus 1 MiB protocol headroom. The shared payload ceiling remains
32 MiB, and decoded-state/real-workload qualification remains in steps 4/6.
Counters distinguish REST polls, handshakes, subscription controls, necessary
pongs and complete incoming data messages (not a claim to inspect TLS packets or
physical fragmented-frame counts). Default route cost 10 remains the documented
conservative default, not an account/IP guarantee; the primary Kalshi rate-limit
page was freshly consulted again and confirms public/IP limits and potentially
missing/lagged headers. Account-specific limit/cost inspection is not authorized.

All supported review findings are resolved by the accountable lead. Clarity
corrections and the final retry correction are lead-verified; the static reviewer
reports are not relabeled as a fresh four-model review of later source. No review
score is used as completion evidence. The final applicable suite, packaged native
check, staged snapshot and exact commit receipt follow.

## Final step-3 receipt and limits

**Completed:** the bounded protected-credential/read-only transport foundation,
controlled regressions, required review and final applicable local checks. No
provider data decoding/reconstruction, source/evaluator integration, source UI/IPC,
account calls, trading or calibration was started. Original issue #5 remains open
with its broader acceptance unmet. U1–U7 remain unresolved.

The two-axis roster completed: Standards Sol + GLM and Spec Sol + DeepSeek, with
the additional targeted Sol correction-flow review documented above. Supported
findings were consolidated, traced through the complete affected paths, corrected
and regression-tested. Standards: caller-drop cleanup and unexpected task exit
were the consequential findings; Spec: cancellation and explicit budget/retry/
runner accounting mismatches were corrected. The unsupported detached-handle
claim was rejected against source and a nonresponsive-task regression, then the
GLM follow-up confirmed the retained ownership flow. No score or consensus is
used as runtime proof.

After the targeted review, actual attempt admission was made atomic under the
shared provider Retry owner, after capacity admission. It rechecks updated backoff
and claims the next paced slot, including successful slow-mode attempts; no mutex
is held across network or timer waits. The new queued-handshake controlled-peer
regression and virtual-clock slow-success regression pass. This last correction
was lead-verified with the full suite, not relabeled as a new four-reviewer pass.

Final local checks on byte-identical behavior sources:

- `python3 tests/credential-runner.test.py`: **passed**, including new finite mode/
  bootstrap/acknowledgment regressions plus existing output/retained-exit cleanup.
- `python3 tests/check-staged.test.py`: **5 passed**.
- `npm run check`: TypeScript passed; **12 UI/VM tests passed**.
- `npm run build`: passed.
- `cargo fmt --all --check`: passed.
- `cargo test --workspace --locked`: **176 ordinary tests passed** (47 native +
  129 engine); one explicit-only native test and two release-only engine fixtures
  omitted in this ordinary run. No failed test was treated as cleared by a later
  unrelated command.
- `cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture`:
  **both fixtures passed**; allocated_bytes=65988608 each, elapsed 4339/4801 ms.
  Controlled engine fixture measurements are not native/provider qualification.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed.
- `npm run tauri build -- --bundles app`: passed; local arm64 bundle **13.64 MiB**.
- Explicit `isolated_native_fixed_items` ignored-test invocation: passed on a
  freshly owned disposable Keychain, with cleanup.
- Fresh finished-bundle invocation of `swift scripts/verify-credential.swift
  --provider-credentials target/release/bundle/macos/Metten.app`: **passed**,
  native operations/signature verification/ack, separately observed process exit,
  marker scans, exact owned Keychain/directory deletion and unchanged global
  default/search-list metadata. No Metten test process remains.

Finished executable SHA-256:
`cca5bb1a59163d58aff81af98ef8891858375e24ce8228567a9b1c6393345de8`.

Mixed-work checkpoint check:

`env CARGO_TARGET_DIR=<selected-worktree>/target python3 scripts/check-staged.py
sh -c 'npm ci && npm run build && /Users/connormoulton/.cargo/bin/cargo test --workspace --locked'`

passed against immutable staged tree **9bea789612987bee568761a2556f562f7e90f11d**.
It exported tracked/staged source into a disposable folder, installed the existing
locked frontend dependencies, built its assets and passed all 176 ordinary Rust
tests without any unstaged proposal/code dependency. The real index/worktree were
preserved. Afterward only this receipt/current index and owning documentation
wording changed; source/dependency/test blobs are verified identical before commit.

**Evidence limits, not waived checks:** the existing dummy GUI walkthrough failed
because this Mac's external AX tree exposes no WebKit controls. The bounded retry
and exact late cleanup are recorded above. The new packaged native check supplies
actual changed-boundary evidence through the prescribed validated disposable
Keychain seam, without pretending that GUI test passed. Ordinary data-protection
Keychain entitlements/prompt/refusal behavior in a distributed app is not proven
by the file-Keychain seam. Step 5's full source/frame/renderer walkthrough remains
future work. Provider access/rights, actual delivery, live acceptance, capacity
qualification and legal clearance remain unverified. No controlled result grants
real-account authority.

Remote CI for the new local-only head is **unverified/not published**: Foundation
checks are triggered by push/pull_request, while owner authority here permits a
local commit only. The equivalent local configured suite passed; that is not a
claim that GitHub CI ran. No lint/typecheck gate is unwired: configured TypeScript,
Rust formatting and workspace Clippy all ran. No push/PR/deployment was attempted.

Delivery freshness query at 2026-11-16 15:03:51 UTC returned origin/main guidance
**fa3663624436e9e4df9d2b000d57f7f21e099786**, unchanged from the loaded baseline;
therefore no applicable guidance/review repeat was required. Rust remains pinned
to 1.100.0. Existing dependency version pairs were preserved; the earlier exact
added-version/license inventory remains applicable.

Final native behavior/dependency SHA-256 identities:

- `Cargo.lock`: `82575c1298d20e259660d0efddce8deda3876bfed17b1c47ca29d662d7e5df86`
- `src-tauri/Cargo.toml`: `933b0ca44b41e08a6f0b267ab2da68814914201834c47974a2d8c27df15d4264`
- `src-tauri/src/main.rs`: `c3c494d0cd0673720455ce7c9241baeeee1376111cb053749884b449109d0b54`
- `src-tauri/src/credential.rs`: `8ea2ec329da6fc68d5896806dbbaf62bc1488a4f06e2cce32298ddc98f0092fe`
- `src-tauri/src/provider_credential.rs`: `09a54c390739e87be6292ce25a9079d8cd42570ec9e24228bcb9a590c34037b1`
- `src-tauri/src/providers/budget.rs`: `8a7c20e01782adfac54ad705101be83b261184c8bf677d449167e5f5af816682`
- `src-tauri/src/providers/memory.rs`: `46449f0cba81174cc9985b36930698f22c141eca9e25bccc2fa63fc9364c8460`
- `src-tauri/src/providers/mod.rs`: `d7e6aa84dd4941477aaa35345a56ced7c09ea931f49e2f75e71840f5fae270c1`
- `src-tauri/src/providers/signing.rs`: `b78b1228adb1b5d26ddb3935657c19054f6f6322830b239fd21ce33f1f5b9be5`
- `src-tauri/src/providers/tests.rs`: `4fb72f758ed8948143bfc4b82885f388ec9cfef939de56ade3f6630e0b50d012`
- `src-tauri/src/providers/transport.rs`: `6d71f4e2c167e43fe2405681898312c607ebfafbbc387826427c3a32b8976d11`
- `scripts/verify-credential.swift`: `5fda0ac5f17fc9619e691f03901aa49bd5b7a1e536abb2a7bd6e6c8bd05f81a3`
- `tests/credential-runner.test.py`: `abe333bc6596f8a9cf725277d60ecbb83320a5ba9bdc51b5cbf21c69ced24635`

## Step-3 established-socket retry correction

Owner requested one implementing subagent followed by a separate review of the
supported Ponytail finding against commit `163a0b7dc962595f5f1f8f59a6fecefa719ccf60`.
Expected path before production edits: established socket outcomes in
`providers/transport.rs` feed the existing provider Retry owner; focused native
peer/clock regressions and the owning native contract describe the correction.
No new owner, dependency, configuration, command, engine change or step-4 work is
needed. Preserve the unrelated docs README edit and every proposal artifact.

Implementer activation independently resolved origin
`https://github.com/moultonlavah/metten.git`, default branch main, and freshly
queried published guidance `fa3663624436e9e4df9d2b000d57f7f21e099786`.
Loaded its root AGENTS entry and discovered verification/published-instruction
discovery, maintainability, CONTRIBUTING, issue workflow and domain guidance;
local/published instruction inventory contains root AGENTS only. Local code map,
native contract, product credential/local-app boundaries, ADR 0001, glossary,
approved design and current issue-5 receipt were consulted. The approved design
SHA-256 matches `21e2926329f67fef05b264dcb8a0dfca0455aaa697c753bfc4f2d95b19fe4bc7`.
The older local entry omits published process additions; the published additions
apply without changing approved product behavior. No missing consequential
guidance or unresolved conflict was identified. Issue #5 remains open with its
existing product/high/prepared labels; native blockers #3/#4 are closed. Tracker
publication remains outside this bounded local correction.

Correction stays on that path. The existing in-flight socket guard now records a
completed send/receive failure once with the existing Retry owner and still
closes/revokes the socket. Dropping an unfinished public operation resets usable
continuity with Stopped, without inventing a failed network attempt. Transient
I/O, EOF/abrupt loss and received Close are distinguished from malformed protocol;
the same classifier covers reads, control writes and protocol flushes. No new
retry loop is introduced. Deadline remains the existing finite Deadline result:
it resets socket usability but is not newly counted as Transport, so local permit
expiry/caller cancellation cannot consume a transient-attempt count. HTTP/handshake
timeout policy is otherwise unchanged by this established-socket correction.

Executed checks:

- Added `established_socket_loss_paces_reconnect_and_restarts_usable_window` first.
  `/Users/connormoulton/.cargo/bin/cargo test -p metten --locked established_socket_loss_paces_reconnect -- --nocapture`
  failed against unchanged production source: expected Transport after abrupt
  controlled-peer loss, received Protocol (exit 101). After the fix it passed.
  It checks the public connect/receive path, no new handshake during pending
  backoff, slow retry reached at the fifth failed socket operation, and a usable
  interval interrupted at 20 seconds requiring a new full 30 seconds to reset.
  Direct loopback fixture sockets generate later receive failures without real
  backoff waits; production handshake admission is checked separately in the same
  test. No provider host or account is used.
- Added a bounded classifier check for BrokenPipe/closed/abrupt EOF versus invalid
  opcode. It covers the shared read/write error mapping; it does not claim an
  observed provider write failure.
- `cargo test -p metten --locked`: **49 passed, 1 explicitly ignored**. Existing
  caller-drop/partial-I/O, stop, ownership, budget, signing and credential tests
  also passed. The explicit-only native Keychain check was not rerun.
- `cargo fmt --all --check`, `git diff --check`, and
  `cargo clippy -p metten --all-targets --locked -- -D warnings`: **passed**.

Unchanged engine/dependency/frontend/native-credential/runner sources reuse the
final step-3 receipt above within its limits. No engine/release-fixture, frontend,
Python, Mac bundle or packaged Keychain/GUI rerun was warranted for this local
transport-only correction. The old finished bundle is not evidence that this new
transport source was packaged or externally accepted. No commit, staging,
push/PR, provider/account/credential access, trading or step-4 work occurred.
Independent fix review remains for the lead's separately requested reviewer.

Implementation source SHA-256 for review:

- `src-tauri/src/providers/transport.rs`: `1b53c4f87c82739a5277c21fbd5cfb6aff7e9445667c238b76c028b00a22c5d3`
- `src-tauri/src/providers/tests.rs`: `606c3dd34b475ca92033f26555af5f5d4a86b61a56da9adede0236e76b6bf6b1`
- `src-tauri/src/README.md`: `9ee6c72d30b6e94afea1dad498efbc2eb3782371d68a73fe5eeaaa52660f088d`

Implementer delivery query at **2026-11-16 15:37:53 UTC** returned the same
published guidance `fa3663624436e9e4df9d2b000d57f7f21e099786`; no affected
instruction refresh/review repeat was needed. The lead must refresh again at
review handoff/delivery and review the source hashes actually delivered.

### Independent correction review and lead delivery

A separate read-only subagent independently loaded the published root entry at
`fa3663624436e9e4df9d2b000d57f7f21e099786`, traversed discovery/verification,
maintainability, CONTRIBUTING, domain, issue/triage and applicable local contracts,
and verified the three correction hashes above. No consequential missing guidance
or unresolved conflict was found. Its final query at 2026-11-16 15:40:06 UTC
returned the same guidance revision. It reported **Ship**, with no supported
must-fix, should-fix or lean finding in this bounded correction.

The reviewer independently executed the established-socket regression and shared
failure-classifier test (one passing test each), plus `git diff --check`. Static
tracing checked exactly-once failure recording, caller-drop reset without a
transient count, guarded receive-capacity rejection, closure/revocation and
terminal malformed-frame errors. The 49-test native suite, formatting and Clippy
remain implementer-executed evidence. Explicit-close/already-revoked-send lifecycle
integration and broader timeout policy were not expanded by this correction;
later connection integration must preserve sustained-usability semantics.

The lead inspected the complete correction and its connected retry/drop paths,
verified the source/test/contract hashes remained identical to the reviewed
candidate, and refreshed remote guidance at **2026-11-16 15:40:07 UTC**: unchanged
`fa3663624436e9e4df9d2b000d57f7f21e099786`. No affected re-review was required.
Only this evidence receipt changed after review. HEAD remains
`163a0b7dc962595f5f1f8f59a6fecefa719ccf60`, index unchanged/empty, with the fix
uncommitted; the inherited docs README edit and all proposal artifacts remain.
No stage-4 work, provider/credential access, bundle/GUI walkthrough, commit, push
or PR was performed. Issue #5's broader acceptance and U1–U7 remain open.

### Owner-authorized correction commit

Owner requested committing the reviewed fix in this selected worktree, on top of
original step-3 commit `163a0b7dc962595f5f1f8f59a6fecefa719ccf60`; no amend or push.
Fresh published discovery on 2026-11-16 16:11 UTC resolved origin/main to
`354f731bc75224d9724ec460b2bb0037a9fa12ea`. Its root AGENTS was loaded; root,
applicable agent guidance and CONTRIBUTING have no changes from the previously
loaded `fa366362` baseline. Published nested inventory still has root AGENTS only.
Prior guidance traversal and independent review remain applicable. The reviewed
transport/test/contract SHA-256 identities above still match. Stage only the four
correction paths; preserve unrelated docs README work and untracked proposals.
Run the published staged-snapshot check for native tests before committing;
reuse the unchanged passing formatter/Clippy and independent review evidence.

Staged verification passed against immutable tree
`f67aef839c2feeacd875ab988374e1b7f4036138` using the existing
`check-staged.py` with `npm ci && npm run build && cargo test -p metten --locked`
(and the selected shared Cargo target directory). Frontend assets built;
**49 native tests passed, 1 explicit-only Keychain test ignored**. Only this
receipt was added afterward; all reviewed behavior/test/contract blobs are
verified identical to the passing staged tree. Final staged whitespace check
passed. Delivery freshness at 2026-11-16 16:12:09 UTC returned unchanged
`354f731bc75224d9724ec460b2bb0037a9fa12ea`; no affected review repeat was needed.
The correction is committed separately atop the original step-3 commit, with
unrelated README/proposals excluded. No push or step-4 work is authorized here.

## Step 4 bounded implementation planning

Connor requested a bounded implementation plan following maintainability after
committing the reviewed step-3 fix. Prepared [step-4-plan.md](../proposals/issue-5-egm/step-4-plan.md),
revision 1, SHA-256 `1db46320dd4a1d55f101f8f0bcccf75e2e20c8727e4eb79a369a0dcf6d5a96eb`. This request authorizes planning only.
Implementation, provider/account/credential access, step 5/6, tracker changes,
commits and publication were not performed or newly authorized.

Baseline: selected code `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`, same worktree
and branch; inherited docs README edit and all proposal/review artifacts preserved.
Fresh origin/main query returned `354f731bc75224d9724ec460b2bb0037a9fa12ea`.
Loaded immutable root AGENTS and traversed applicable published discovery and
verification, maintainability, CONTRIBUTING, issue workflow, domain/triage
instructions. Local/published inventory has root AGENTS only. Consulted the map,
relevant native/engine contracts, source types/worker/transport/memory and public
test seams, product/ADR/glossary, approved design, preserved step-2 plan and source
profiles. Approved design and step-2 hashes match the prior identities. No
consequential missing repository guidance or conflict was identified. Issue #5
remains open with unchecked acceptance, prepared product/high labels, no assignee
or comments; native/body blockers #3/#4 closed, no children returned. No tracker
mutation or project-stage verification is claimed.

The plan assigns one owner per fact and reuses existing transport, retry,
reservations, worker and synchronous source/evaluator interfaces. It specifies
Kalshi-first reconstruction, bounded discovery/current-next handoff, named BRTI
and spot mapping, capability stops and meaningful controlled tests. Source reading
identified two explicit integration requirements: service outbound controls without
canceling healthy receives, and distinguish a replaceable socket session from the
terminal provider-task permission fence. Neither is claimed implemented. U1–U7
remain unresolved; controlled readiness/evaluation does not enable real profiles.

A bounded public-document recheck consulted official Kalshi book/BRTI,
Coinbase Exchange channels and Kraken v2 trade references linked in the plan.
It verifies documented shapes only, not access/rights/precision completeness.
Before implementing exact routes/recovery commands and library polling behavior,
recheck those primary contracts as stated in the plan.

Planning checks passed: 7 local document links/anchors, U1–U7 coverage, five
ordered implementation items, explicit numeric ceilings/deadlines/retry policy,
fenced command shape, whitespace and unchanged approved-document hashes. The
first numeric checklist caught an omitted explicit 30-second retry-reset value;
the plan was corrected and the complete checklist then passed. A lead content
review traced expected files, ownership, failure flows and completion conditions
against current code/design. No independent agent review or runtime/build tests
were run for this documentation-only task. Native source/tests/dependencies and
HEAD/index remain unchanged; existing runtime evidence retains its original limits.
Delivery query at **2026-11-16 16:21:33 UTC** returned unchanged published guidance
`354f731bc75224d9724ec460b2bb0037a9fa12ea`; no affected review repeat was required.
Implementation handoff must refresh guidance again.

## Independent step 4 maintainability plan review

Connor requested one subagent review before implementation. The separate reviewer
read the unchanged revision-1 [step-4 plan](../proposals/issue-5-egm/step-4-plan.md),
SHA-256 `1db46320dd4a1d55f101f8f0bcccf75e2e20c8727e4eb79a369a0dcf6d5a96eb`,
against code `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`. No implementation or
plan edits were performed. It independently loaded root AGENTS at freshly resolved
origin/main `354f731bc75224d9724ec460b2bb0037a9fa12ea` and traversed published
discovery/verification, maintainability, CONTRIBUTING/domain/issue/triage guidance,
local map/contracts, product/ADR/glossary, approved design, step-2 plan, source
profiles and connected source/tests. Both instruction inventories contain root
AGENTS only. No consequential missing guidance or conflict was found. Reviewer
freshness at 2026-11-16 16:39:46 UTC and lead delivery freshness at
2026-11-16 16:40:27 UTC returned that same published revision.

**Verdict: revise finding 1 before implementation.**

1. **Must fix — define changed-descriptor recovery.** Plan lines 103–105 require
   changed-schedule revalidation/handoff; line 168 expects no engine production
   edits. For a retained successor N whose verified opening changes from 12:15,
   SourceState::Metadata only toggles verified and continues using the retained
   descriptor (sources.rs 938–975). RegisterMarket rejects an existing market ID
   (400–408) and a third retained market (885–889). Re-verification of unchanged
   descriptors in sources tests does not establish replacement. The approved
   design's changed-schedule recovery therefore has no specified realizable path.
   Define the owner-controlled transition before implementation: narrow source
   descriptor replacement/retirement with public tests and contract/inventory
   updates if recovery must preserve the context, or an explicit stopped/new-context
   policy reconciled with promised behavior. Cover same ticker/changed schedule,
   old completion, fresh metadata and recovered data. Do not add a second mutable
   native schedule. Skipping this leaves an unplanned engine change, unrecoverable
   market or risk of re-verifying stale times. This is a static interface finding,
   not a reproduced runtime defect in the existing app.

The lead checked the cited Metadata/RegisterMarket/validation paths and confirmed
the gap. The fallback instruction to revise the plan if a missing engine feature
is discovered is safe, but the demonstrated case should now be resolved in the
plan. No other supported blocker was found in the reviewed ownership/locality,
proposed session fence/socket service, bounded parser/worker/memory/ID ledger,
capability stops or controlled evaluator checks. These proposals still require
implementation evidence; the review does not claim their library or capacity
assumptions passed. No runtime tests/builds/provider/credential/app operations
were run. Plan hash, code, index and unrelated work remain preserved. Only this
existing evidence record was updated by the lead; no commit or publication.

## Step 4 plan correction and requested re-review

Connor authorized correcting plan finding 1 and a further subagent review before
implementation. Revised only the existing step-4 plan to revision 2, SHA-256
`c18c4d20b4f65890d49b4366455fbd77d701504830192d82df754614ad192322`.
Code remains `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`; original proposals,
unrelated README edit and index are preserved. Fresh published origin/main query
still resolves `354f731bc75224d9724ec460b2bb0037a9fa12ea`; root AGENTS and the
published discovery procedure were refreshed, with unchanged applicable linked
maintainability/verification guidance reused from the prior independent traversal.
No new consequential instruction conflict or missing required source was found.

The plan now specifies a bounded same-identity, unentered-market ReplaceMarket
transition in SourceState, expected-old/token validation, invalidation before
new schedule use, initial unverified state and fresh-token metadata/data recovery.
Added engine source/public tests/contract to the inventory and required affected
source work/storage proof review and ordinary engine checks. Tests cover stale
completions, invalid/overlapping/no-op replacement, late opening, Stop/Pending
races, fresh recovery and once-only boundaries. Entered schedules cannot be
rewritten; they require stopped/new-context recovery.

The connected Evaluator catalog is also immutable. Revision 2 makes the controlled
consumer stop an attached evaluator on descriptor replacement instead of feeding
a duplicate registration or new-schedule updates into its old catalog. Retained
history/checkpoints remain evidence in the stopped evaluator; explicit new-pair
recovery starts a fresh epoch/history without claiming uninterrupted evaluation.
Native source-only unentered-successor recovery remains in-context. No evaluator
production changes or future financial recovery semantics are added by this plan.

Local plan checks passed: seven local links/anchors, corrected scope/inventory/test
requirements, removal of the obsolete no-engine-change assumption and whitespace.
The same independent reviewer was reactivated with this exact candidate, unchanged
code/guidance baseline, prior finding and bounded affected-review instructions.
Review is pending; no implementation or runtime checks have started.

### Revision 2 independent approval and delivery

The same separate read-only reviewer independently refreshed the published root
and discovery baseline and approved revision 2: **finding 1 resolved at the
planning level; no supported remaining must-fix or should-fix issue**. Reviewed
plan hash remains `c18c4d20b4f65890d49b4366455fbd77d701504830192d82df754614ad192322`.
It reused unchanged review coverage and retraced the source replacement,
invalidation/tokens/stamps/current-display and evaluator registration/stop paths.
It confirmed that explicit evaluator Stop removes the published candidate while
retaining owned history, so the plan's fresh-pair recovery does not feed changed
schedules into the old catalog or silently claim history continuity. The new
public regression requirements address the demonstrated same-ticker case.

Reviewer guidance freshness at 2026-11-16 16:46:15 UTC and lead delivery query at
2026-11-16 16:46:40 UTC both returned unchanged
`354f731bc75224d9724ec460b2bb0037a9fa12ea`. No missing consequential guidance
or conflict was found. Lead verified the delivered plan hash and unchanged code,
HEAD and index. Reviewer/lead whitespace checks passed. Only this existing receipt
changed after review; the plan was frozen. No runtime tests, builds, source edits,
provider/credential operations, commits or publication occurred. Approval covers
the bounded implementation plan, not implemented behavior or U1–U7/external
acceptance. Stop remains before implementation pending owner authorization.

## Step 4 implementation in progress

Owner authorized implementation of approved revision-2 plan, hash
`c18c4d20b4f65890d49b4366455fbd77d701504830192d82df754614ad192322`,
against selected HEAD `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`.
Fresh origin/main query returned unchanged `354f731bc75224d9724ec460b2bb0037a9fa12ea`.
Loaded published root/discovery/maintainability and applicable linked guidance;
reuse the unchanged product/ADR/map/contracts/approved-document traversal above.
Issue #5 is open; native/body blockers #3/#4 remain closed. Preserve unrelated
README edit and proposal artifacts. Expected change path and limits are the
approved plan inventory: source replacement, native concrete adapters, bounded
worker/transport integration and controlled tests. No step5/6, real provider or
credential access, financial run, commit or publication is authorized here.
Completion and required checks remain pending; no partial result is acceptance.

Implementation inventory reconciliation: the shared `wire.rs` helper serves all
three concrete decoders and owns lexical/depth/list bounds and exact raw numbers;
Serde's existing `raw_value` feature is enabled without a library/version upgrade.
`discovery.rs` owns fixed-route orchestration, bounded metadata validation and the
60/30/10 opportunity calculation. Its metadata bodies are decoded by the owned
source worker, not an async HTTP task. Added `source.rs` keeps the source writer,
view and owned integration together instead of growing the lifecycle/control
module into an unrelated second responsibility. These are concrete step-4 owners,
not a provider registry or a new crate. The frozen plan's engine scope is unchanged.

Handoff refresh: fresh default-branch query on 16 November returned
`28c9f3f337f1eb26456a459c5e83024ab739b946`; missing object fetched with no destination
ref and no FETCH_HEAD update. Loaded its root AGENTS and CONTRIBUTING, compared
applicable discovery/verification, maintainability, domain/issue procedures and
map with the prior `354f731` baseline. The only applicable-tree difference is an
unrelated UI-helper map pointer; standards and required guidance are unchanged.
No new missing source or consequential conflict. The task remains selected code
`cc22fa9` plus this uncommitted implementation; no existing tracking ref/index,
HEAD, unrelated edit or proposal was replaced.


## Step 4 implementation and verification

Connor authorized the approved revision-2 plan's implementation. The selected code
remains `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`, branch
`t3/brainstorm-issue-5`, plus the uncommitted step-4 source identified below. There
are no staged changes. The unrelated one-line `docs/README.md` edit and all
proposal/review artifacts remain preserved. No commit, push, PR or tracker mutation
was performed. Stop is before step 5 and step 6; this is not issue-5 acceptance.

### Published guidance and maintainability disposition

Authoritative remote remains `https://github.com/moultonlavah/metten.git`, default
branch `main`. Fresh handoff discovery and the delivery query at **2026-11-16
19:59:44 UTC** returned **`c1a6fa973ea0aee8f635a42a9f44923d1ec321df`**.
The immutable root AGENTS and verification/discovery entry were loaded directly.
Applicable published maintainability, CONTRIBUTING, code-map, issue workflow,
triage and domain guidance were traversed; local root instructions, product
specification, ADR0001, glossary, native/package/engine/tool contracts, source
profiles and approved task documents were consulted. Published/local discovery
found root AGENTS only. The selected older worktree lacks the published
maintainability file; that standard was read from the immutable published tree,
not treated as absent or copied into this worktree. No required guidance remains
missing and no consequential conflict was identified. The intermediate published
updates from `354f731` through `28c9f3f` to this revision add unrelated UI-map
navigation; applicable standards/procedures are unchanged. Their unchanged evidence
was reused. Missing objects were fetched without a destination ref or FETCH_HEAD
update; HEAD, index and existing tracking refs were not moved. The current session
received Connor's standing discovery rule and explicitly refreshed it after handoff.
No new agent session or independent implementation review is claimed.

The expected path remains fixed transport → concrete decoder/reconstruction → the
existing sole source worker/SourceState → derived view. Controlled tests additionally
use the real Evaluator. Mutable normalized facts stay in SourceState, provider
format/identity/book state stays in its native decoder, and strategy/history and
financial decisions remain in the unchanged evaluator. The only engine production
change is bounded `ReplaceMarket`; the five new public source tests cover its
invalidity, stop and fresh-pair obligations. A wire-field rename remains local to
its decoder and fixture; thresholds remain validated JSON. There is no strategy-
name branch, new crate, library version, compiler change, provider registry,
per-message task, general scheduler or durable source state.

The implementation inventory departure is the three concrete ownership splits
recorded above: shared `wire.rs` for bounds/exact numeric tokens used by all
adapters, `discovery.rs` for fixed-route metadata orchestration, and `source.rs` for
the sole writer/view and owned integration. Existing Serde's `raw_value` feature
preserves lexical numbers; Cargo.lock is unchanged. Map and owning contracts now
point to those owners, behavior and checks. The independently approved plan is
unchanged; its approval covers planning rather than this implementation's correctness.

### Implemented behavior and corrections

- Kalshi reconstructs both retained YES/NO books, derives complementary asks,
  preserves quantity/clock/sequence, checks sequence before irrelevant-ticker
  filtering and recovers both books independently on the same socket. Empty
  liquidity expires the absent binding; later reconstruction can initialize it.
  Trade identity ledgers fail capacity rather than evict. BRTI raw values remain
  separate from average/count/window details. Known command rejection disables
  its scope; unknown error codes do not become invented entitlement findings.
- Coinbase and Kraken validate fixed products, snapshots/initialization, monotone
  identity, same-price new observations and replay/conflict behavior. Kraken
  validates the whole bounded batch before publication and chooses the newest
  snapshot identity even when its array is descending. Prices retain exact
  six-place USD or six-place cents meaning; incompatible precision is refused.
- Six startup/four successor metadata GETs, two-page/400-candidate bounds,
  canonical-series/detail validation, opaque cursor encoding, 60/30/10 checks and
  paced post-opening fallback use existing budgets. Expired metadata cannot
  readmit a retired slot. Prepared rollover retires before adding, preserves
  underlying identities and performs no boundary-search GET. Same-ticker schedule
  replacement revokes before revalidation; entered source contexts stop. A
  controlled evaluator with an immutable old catalog must stop and recover with
  an explicit fresh pair/epoch, retaining old history only as stopped evidence.
- Reader/writer halves are concurrently polled inside each of the three owned
  provider futures. Progress/control wakes retain the same pending receive. A
  late old-token frame cannot restore evidence; one coalesced counted snapshot
  retry can obtain the replacement under the next receive identity. ACK and
  first/replacement-data deadlines remain separate. Progress starts with the
  captured tokens, avoiding redundant initial recovery controls.
- Metadata/connection/gap changes clear reconstruction baselines. Per-market book
  and trade details retire with their slot. Native permission epochs, session and
  worker fences invalidate retained views before more ordinary work. Exhausted
  epochs never wrap. A nonqueued scheduling-loss fact blocks market views until
  the worker applies core invalidation; a weaker fact cannot erase terminal loss.
  Only the worker owns the reanchored schedule epoch. Normal slow HTTP work alone
  is not treated as sleep. A delayed handshake rechecks identities and retained
  market membership before sending its initial subscriptions.
- Control receiver ownership returns through Drop even after task return, panic,
  cancellation or an unpolled future. Explicit recovery joins the old owner first.
  Shared retry state resets its usable interval on close/drop without adding a
  transient failure. Ping wakeups coalesce while automatic pongs remain independent
  of subscription permits. The keepalive counter counts explicit flush attempts,
  not every automatic wire pong. Existing total-deadline shutdown, retained
  unfinished handles and guarded engine-save behavior remain covered.

The original RegisterMarket-only schedule replacement failed its new regression;
`ReplaceMarket` corrects that demonstrated case. An early controlled three-provider
case with HTTP404 also exposed stale-generation handling that incorrectly ended
unrelated Kalshi work; the corrected test now retains BRTI and all seven initial
controls. Other lifecycle/parser corrections above came from static tracing and
focused tests added during implementation. Do not describe those as individually
executed red/green reproductions. The delayed-handshake, nonqueued schedule notice,
ping-burst and shared retry interval cases pass on the final source; they were not
rerun against a reverted faulty tree. Intermediate compile/fixture/lint failures
were corrected; only the final results below are passing evidence.

### Executed evidence and reuse

All commands below exited zero against the source manifest below:

| Check | Result and meaning |
| --- | --- |
| `/Users/connormoulton/.cargo/bin/cargo test --workspace --locked --quiet` | **221 ordinary tests passed:** 87 native and 134 engine (25 library, 6 session, 34 source, 59 strategy, 10 storage). One native disposable-Keychain check and two release-hour strategy fixtures remain intentionally ignored in this ordinary command. No failed test. |
| `/Users/connormoulton/.cargo/bin/cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed strict workspace lint, including test targets. |
| `/Users/connormoulton/.cargo/bin/cargo fmt --all -- --check` and `git diff --check` | Passed formatting/whitespace; new Rust files are included by cargo fmt. |
| `PATH=/Users/connormoulton/.cargo/bin:$PATH npm run tauri -- build --bundles app` | Passed TypeScript/frontend build and optimized Mac app bundling; artifact `target/release/bundle/macos/Metten.app`, reported 13.65 MiB. It was not launched for a GUI/credential walkthrough. |

Named controlled checks navigate the consequential behavior in
`src-tauri/src/providers/tests.rs` and owning embedded tests: actual learning JSON
through reconstructed books/SourceState/Evaluator; empty side expiry; shared gap
and same-socket old-receive replacement; current/next detail retirement; stale
metadata/baseline revocation; three repeated rollovers with two books/shared SID;
six/four metadata GETs and obsolete parsing; three owned tasks/seven initial
controls; explicit recovery after join; unpolled service Drop; retained context
charge; native Stop during controlled Pending; ping burst before a rate-limited
control; and delayed handshake subscribing only retained markets. Embedded checks
cover exact precision/exponents, depth/string/list/identity limits, 8192-plus-one
book admission, replacement overlap refusal, whole-batch validation, independent
ACK/both-book deadlines, scoped command rejection, terminal schedule-loss merging,
clock discontinuity and permanent epoch exhaustion. Discovery checks include
initialized-future versus elapsed metadata and scratch ownership through canceled
handoff. Existing ordinary budget, partial-I/O/drop, canceled-reservation,
slow-retry, worker/task failure and nonresponsive shutdown/save-guard cases also
passed in the final suite. These are bounded fixtures and controlled peers, not
an exhaustive scheduler proof or provider acceptance.

Reuse the identified passing evidence in the final step-3 receipt for both
release-hour engine fixtures, TypeScript/12 UI checks, Python runner/checkpoint
regressions, disposable Keychain and packaged native credential/signature/exit/
marker/cleanup checks. The final bundle rebuild also refreshed TypeScript. Source
identity comparison confirms unchanged `strategy.rs`, strategy fixtures, engine
storage, native main/credential/provider-credential/signing/bootstrap/permissions,
UI and runner/checkpoint files, Cargo.lock and toolchain. Ordinary tests reran the
relevant shared Rust behavior; no changed release arithmetic, UI, credential or
runner behavior justifies repeating the longer unchanged checks. The prior staged-
snapshot evidence applies to its named tree only, not this uncommitted tree. No
step-4 staged snapshot or remote CI is claimed. The blocked dummy WebKit GUI
walkthrough and ordinary data-protection-Keychain entitlement/prompt evidence
remain unverified; no walkthrough was relaunched.

### Retained resources and work argument

The fixed topology remains three provider tasks, one existing worker, one ordinary
raw slot, one metadata-command slot and seven finite mandatory controls per
provider. Recovery retains three replaceable stream records, not an event log;
scheduling invalidity retains one nonqueued fact. Socket reservations are 4 MiB
each (12 MiB total). Bounded raw ownership can include three receiving tasks,
one queued payload and one worker payload (5 MiB at the per-message maximum).
Worker decoding reserves 4 MiB before typed parsing. Each parsed metadata handoff
retains its 4-MiB permit through the command/reply payload until consumed/dropped;
HTTP bodies are at most 1 MiB and the bounded discovery collector reserves256 KiB.
Caller cancellation releases or transfers each owned reservation with its value.

Each retained book reservation is `2 * 8192 * 128 + 65536` = **2,162,688 bytes**.
Two books reserve4,325,376; a replacement coexisting with both reserves6,488,064,
below the separate8,388,608 two-book/overlap ceiling. The unchanged pinned Rust
1.100 BTree convention has11 keys/12 edges and minimum5-key occupancy; node count
is bounded from that occupancy. An i128-pair node plus edges is under512 bytes,
so128 bytes per level plus fixed headroom conservatively bounds these trees.
Each opaque trade identity reserves its text length plus2048 bytes for owned key/
record/node growth. IDs persist across reconnect until contract retirement, and
capacity fails explicitly; there is no uncharged ever-growing archive or eviction.

A shared64-KiB base permit follows the native context through worker, source handle
and provider futures. Every retained derived view reserves256 KiB before cloning;
this includes its bounded requirements, at most four per-market detail records,
three underlying detail records and fences in addition to the bounded core view.
The existing core view bound is below160 KiB; the additional bounded native
requirements/details/permissions remain inside256 KiB. Constructor overlap has
its own64-KiB reservation and initial temporary view256 KiB before retained copies.
All these adapter payload reservations use the same32-MiB owner, including bodies,
books, scratch and retained consumers. Independent maxima need not fit
simultaneously: admission refuses before growth when their combined reservation
would exceed32 MiB. This is a fail-closed allocation argument, not a claim that
all maximal workloads are supported or that SDK/TLS/kernel/allocator RSS is capped.
Step-6 measurement and capacity qualification remain pending.

SourceState's existing owner/input/effect/view/constructor proof stays below1 MiB,
separate from unchanged evaluator8-MiB/4096 work admission. ReplaceMarket checks
three bounded128-byte identities, the exact expected descriptor, two market slots,
nonoverlap/duration and revision headroom before commit. Old/replacement descriptor
text overlap is bounded to768 bytes plus fixed records, with no proportional
allocation for rejected input. It reuses bounded existing invalidation scans
(max128 bindings) and two-market selection; it introduces no elapsed-time loop or
history allocation. Native decoder work is bounded by the admitted1-MiB message,
depth48,16-KiB text and8192 entries per side. These are bounded work/input claims,
not measured wall-clock latency guarantees.

### Source identities and limits

SHA-256 values below identify final code/configuration and unchanged proof inputs.
The manifest digest is the SHA-256 of ordered `path SPACE hash NEWLINE` entries.
Documentation receipts are not part of that code manifest.

Manifest SHA-256: `369199615e6cc7b27db3c16a427e160fc2570230d4ab9993f7a6451e3d430372`.

| Path | SHA-256 |
| --- | --- |
| `crates/engine/src/sources.rs` | `2819aef575e388310f312a4f384d0baf6e6a9d76cefa3b907fc61d34cc7a1c95` |
| `crates/engine/tests/sources.rs` | `abaa7acba5bea64383911405668241e33a5086e592631f30658c915e0be9d34e` |
| `src-tauri/Cargo.toml` | `44109ec5c61faa70d14c93d2bc56f0d501df8bcf8ec2404a70cf3dbee2f059e3` |
| `src-tauri/src/providers/mod.rs` | `7f6c89db192c934ca59fea7f3c78981d628170135435af101340fa051a299cda` |
| `src-tauri/src/providers/memory.rs` | `a18f18263bbcf07d762642e7ef08ee56c1b9dde32741d585e16df0a8d3b6f73b` |
| `src-tauri/src/providers/transport.rs` | `948b1c59508f9bb12c5644d33c7f21bdc39f083d531a719355adb99babd4c835` |
| `src-tauri/src/providers/wire.rs` | `d617adbfd15c1a0751a362efdc27a32e54e2655732d92f54b4c2d10ad14048c0` |
| `src-tauri/src/providers/kalshi.rs` | `989a5780249c264d40448718baab090758b305f9f1e1f8663ab2253992c614cd` |
| `src-tauri/src/providers/coinbase.rs` | `0329f02900875c567cab60593ac96146d093fb3ee688b502cebb1c8101e9ba71` |
| `src-tauri/src/providers/kraken.rs` | `71f81e097b5836b2de97a343dfc68e96645aba258bbed94c154a0a22029ff6f6` |
| `src-tauri/src/providers/discovery.rs` | `fe88eb8c4b8de0ecc1457df10c0a045bd595ab1732ee1393378c5f72b433a730` |
| `src-tauri/src/providers/source.rs` | `f2ef0f7ac38b142a5df22bcf302272c633455d4021e82ddadc4c8d20f51dba89` |
| `src-tauri/src/providers/tests.rs` | `e976f7538d0b5c229842d03c944342d84a8d494a0fe8f607614de8fb2ba03d7c` |
| `Cargo.lock` | `82575c1298d20e259660d0efddce8deda3876bfed17b1c47ca29d662d7e5df86` |
| `rust-toolchain.toml` | `6b1cf8341d9a08d76c6deb1e9df90a0b208d0ef0d00ee05e0173e9cd4f0f425d` |
| `crates/engine/src/strategy.rs` | `54e76b54a69526aacd710887ba017b6fe7f442467db005caae5969d006961322` |
| `examples/learning.json` | `a79bd747078601cd16bc292c7896cdcb9f2fe3db0feebf412284a50a2bd50aa5` |

Bundled executable SHA-256: `5129e145d10c05ac14d0ec64115bea1c1b852287e2e1cce8711f0f9edea3c6b4`.

Approved document identities were rechecked unchanged: design
`21e2926329f67fef05b264dcb8a0dfca0455aaa697c753bfc4f2d95b19fe4bc7`, step-2
revision5 `528b87824818515a4349ee99b658f9bc77ed2deb532e514b26f7d7ffa8b60a76`,
step-4 revision2 `c18c4d20b4f65890d49b4366455fbd77d701504830192d82df754614ad192322`.
The exact policy is the approved design, not inferred transport timeouts.

**Limits:** U1–U7 remain unresolved. No real credentials, ordinary credential-item
access, provider/demo/production account calls, trading, paid service, deployment,
publication or other trading application was used. Synthetic current-use leases
are test-only; production capabilities remain unsupported. Transport success
establishes neither price freshness, genuine book-observation semantics, complete
history, opening/settlement references, account entitlement nor rights. Actual
OS sleep/wake, production throughput/RSS/capacity, provider delivery/live acceptance,
legal clearance, ordinary Keychain prompts/entitlements, GUI acceptance and remote
CI for this unpublished work remain unverified. Source snapshots are point-in-time
facts, not a price lease; consumers must recheck full dependencies, native
permissions and current clock before use. The lead reviewed final ownership,
callers, changed source and unrelated-work preservation; no independent
implementation review result is asserted. The requested implementation ends here.

Delivery consistency check: final query at **2026-11-16 20:01:53 UTC** again
returned unchanged `c1a6fa973ea0aee8f635a42a9f44923d1ec321df`; no affected
review was repeated. All28 newly added/changed local Markdown links/anchors
resolved. Final formatting and whitespace checks passed. Approved hashes,
selected HEAD and empty index were checked; protected tracked code/fixtures/tools
outside the bounded edits matched HEAD. The unrelated docs README still has its
original single added line; proposal/review files remain untracked and untouched.
Only documentation receipts changed after the final executable checks.


## Step 4 owner-requested sol-loop

Connor requested one Sol subagent loop before committing. This authorizes the
bounded review/correction loop; no push/publication or later-stage work. The lead
will not commit until agreement on the current behavior and covering checks.
Fresh origin/default-main query on16 November again returned published guidance
`c1a6fa973ea0aee8f635a42a9f44923d1ec321df`; immutable root/discovery and applicable
linked standards remain the loaded baseline. No new missing source/conflict.
Objects fetched with no destination refs/FETCH_HEAD update preserve tracking refs.
The published merge-base is `47d0f384669f38aaf76e3c5bc50060836b235f02`.
Use the accepted step-3 HEAD `cc22fa9d2e4e749df0ef624aac89fa7cc745700e`
as the bounded step-4 review base rather than widening the review to prior stages.
One fixed diff command reviews that base through the selected paths; six new
native source files are read explicitly. The unrelated docs README and proposal
artifacts are excluded from changes, while approved plans remain review inputs.
The source manifest and unchanged working copy allow reuse of the just-passed221
ordinary tests, strict workspace Clippy, formatting and Mac bundle checks above.
No additional runtime check is needed merely to start review. Remote CI, real-feed,
GUI/ordinary-Keychain and U1–U7 limits remain unchanged.

Capabilities exposed Codex `gpt-6.1-sol` as the latest available Sol. One review task
per round, default3-round budget; only evidenced state/concurrency or security
findings can justify extending to5. The reviewer is read-only and may not delegate.
The lead makes/dispositions minimal corrections after each result and records the
ledger here. No scoped edit occurs while a review is running.

Initial scope before this ledger: **18 files, +7254/−26 lines**, including new files; code/config **+6644/−15**. Documentation receipts can grow without behavior changes. Round1 pending; no finding or approval yet. Usage not yet returned.


### Sol round 1: S1 and bounded correction

Round1 completed with **CHANGES** using Codex `gpt-6.1-sol` on verified high
reasoning effort. Task ID:
`node:delegated-task:command%3Amcp%3Aebe304f2-6e01-499b-90d4-997fa0f55c9a%3Adelegate-task%3Ametten-step4-sol-r1-20261116`.
The reviewer independently loaded immutable c1a6fa9 root AGENTS and traversed the
published discovery, map, maintainability, CONTRIBUTING, domain, issue and triage
sources; inspected local root/contracts/product/ADR/approved plans; found no nested
instructions, missing required source or consequential conflict; and matched all
three approved-document and17 code/proof-input hashes. It read the full scoped
change and six untracked code owners with connected callers. No second actionable
finding was returned. Usage was not exposed; it is not estimated.

| ID | Severity | Lead disposition and evidence |
| --- | --- | --- |
| S1 | blocker | Agreed. An ordinary5-ms worker tick could pass the exact opening millisecond, so the equality-only dispatcher emitted no prepared boundary. The reviewer reproduced0 instead of1 entry in a disposable rollover variant. The lead's new between-ticks regression independently failed with the same0/1 assertion before the fix, then passed after it. Correction implemented; independent resolution pending round2. |

`SourceWorker::advance` now orders at most two due retained openings, applies each
at its exact scheduled clock before advancing to sampled time, and refuses to
relabel post-opening accepted facts as pre-opening readiness. Existing core
ready-since, generation, lease, history and once-only-entry checks remain the
entry authority. Already elapsed windows are not given a late entry. The worker
applies scheduling/connection invalidity first, dispatches due boundaries before
ordinary metadata work, and repeats that ordering after resampling time before
raw data. This closes the second crossing point between a tick's initial advance
and its actual raw processing. A normal missed millisecond is handled without a
new scheduler, timer task, persistent boundary ledger or engine behavior change.

The existing repeated-rollover fixture is shared by exact-tick and1-ms-late tests;
both assert one boundary at the scheduled timestamp on each of three handoffs,
retire-before-add, two retained books, stable subscription/source identity and no
duplicate entry. A second new test covers invalidated preparation and readiness
rebuilt only after opening: neither creates an entry; late verified readiness can
still display the new current market. The new tests target S1 rather than mirror
implementation details. Sharing the existing fixture avoids duplicate setup;
no unrelated refactor, new public API, strategy or engine code was added.

The dispatcher can transiently hold its normal source view and one read-only
opening-clock view; the latter is dropped before the boundary effect is built.
Each core view is below160 KiB by the unchanged bound. The retained core owner
(under128 KiB) plus two such views is under448 KiB before fixed bounded boundary
input; after dropping the second view, one effect (under128 KiB) fits the same
allowance. Constructor/input phases do not coexist with this synchronous advance.
It stays within the existing1-MiB source envelope, separately from adapter32 MiB
and evaluator8 MiB. Sorting/checking at most two descriptors adds fixed bounded
work, not an elapsed-time loop. No production latency or capacity is inferred.

Final covering commands after correction passed: **223 ordinary Rust tests**
(89 native +134 engine), strict workspace/all-targets Clippy, cargo fmt check and
whitespace. The Mac optimized app bundle rebuilt successfully against corrected
production behavior. An intermediate negative-test import error made test/lint
attempts fail; those were corrected and both checks explicitly rerun to zero.
The two targeted rollover tests and late-readiness/invalidity test pass. Existing
unchanged release-hour/UI/Python/credential evidence retains the prior reuse
rationale. No GUI, Keychain or real-provider walkthrough was run.

Guidance refreshed before round2 at **2026-11-16 20:31:08 UTC**: default main now
publishes **`dc892f8e19c710cc2c435b43d90b14d440ff6730`**. Loaded that root;
compared applicable guidance and read changed CONTRIBUTING and map pointers plus
published Oxlint configuration/evidence navigation. Root/discovery/maintainability/
domain/issue instructions are unchanged. The addition is JS/TS correctness lint
for unchanged source/tests; this older selected package has no new lint command.
No JS/TS changes require an affected lint rerun, and no lint configuration or
unrelated tooling is imported into step4. Required review scope and native checks
are unchanged; no missing applicable guidance/conflict. Objects fetched without
updating refs, index, HEAD, working files or FETCH_HEAD. Round2 must independently
read the new published entry and compare that discovered set.

Corrected17-input manifest SHA-256: `d063d290528b7df6485885dfee03c582968884eecc7ad328d8f65f1397ced228`. All earlier entries remain unchanged except these overrides:

| Path | SHA-256 |
| --- | --- |
| `src-tauri/src/providers/source.rs` | `aa4cb6540f4b21624a3f01180e318d4acab9b67d82c59a6a69f168626b8f8b37` |
| `src-tauri/src/providers/tests.rs` | `5b2051b99a8d33b5ac4e9ffb869d113bd182942f5f71742f9c1f6a9bd58bedf8` |

Corrected bundled executable SHA-256: `5129e145d10c05ac14d0ec64115bea1c1b852287e2e1cce8711f0f9edea3c6b4`.

Round2 candidate measurement before this final ledger line:18 files,+7465/−26
including untracked code; code/config+6743/−15 versus initial+6644/−15. The99
additional code/config lines are the bounded dispatcher/worker ordering and two
focused regressions using shared fixture setup; no optional behavior was added.
Tracked-only `git diff --shortstat` omits all six new source files and is not a
complete change-size claim. No finding was declined. S1 is fixed with passing
checks, awaiting independent resolution; no approval/commit/push yet.


### Sol round 2 approval and checkpoint

**Sol verdict: APPROVE. S1 resolved; no open blocker or should-fix finding.**
Two rounds total with Codex `gpt-6.1-sol` on verified high reasoning effort:
round1 full change/connected-code review, round2 only S1/changed callers and
regressions with unchanged coverage reused. No finding was declined and no
additional round is required. Usage was not exposed and is not estimated.
Round2 task ID:
`node:delegated-task:command%3Amcp%3Aebe304f2-6e01-499b-90d4-997fa0f55c9a%3Adelegate-task%3Ametten-step4-sol-r2-20261116`.

The reviewer independently ran all three targeted regressions: exact-tick repeated
rollover,1-ms-late repeated rollover at the exact boundary timestamp, and revoked/
late-rebuilt preparation. All passed. It traced changed worker callers and
confirmed known invalidity precedes dispatch at both sampling points, ordering
and once-only readiness remain enforced, and no scheduler/owner/engine behavior
was added. It independently refreshed/loaded published root at dc892f8 and
traversed discovery/verification, maintainability, map, CONTRIBUTING, domain,
issue and triage sources plus local contracts/product/ADR/approved sections.
All three approved-document identities and17 current code/proof inputs matched
the corrected manifest above. No missing applicable guidance or consequential
conflict. The reviewer changed no repository file, index or ref. Its approval
reuses round1 coverage and does not claim another whole-code audit or broad
check execution. The lead read the completed result and accepts the disposition;
no remaining concern was identified within this bounded task.

Lead delivery freshness query at **2026-11-16 20:35:26 UTC** again returned
unchanged dc892f8e19c710cc2c435b43d90b14d440ff6730. No affected guidance review
needs repetition. Current reviewed code is frozen. The99-line code/config growth
from+6644/−15 to+6743/−15 is confined to S1's bounded dispatcher/callers and two
regressions sharing existing setup. Documentation/reporting growth is separate.
The approved behavior has223 passing ordinary tests, strict Clippy, formatting,
whitespace and rebuilt Mac app evidence above; unchanged evidence is reused only
within its named limits. The next operation is explicit18-file staging and the
published disposable staged-snapshot check before the local commit. No push,
publication or later-stage work is authorized by this checkpoint.


Checkpoint verification passed against immutable staged tree
**`bf9ac69d8ccbb40800737eaec8e5e7deebf20faa`** using:

`env CARGO_TARGET_DIR=<selected-worktree>/target python3 scripts/check-staged.py
sh -c 'npm ci && npm run build && /Users/connormoulton/.cargo/bin/cargo test --workspace --locked'`

The published helper exported the exact18-file staged change over its tracked
base into a disposable folder, installed locked frontend dependencies, built assets
and passed223 ordinary Rust tests (89 native +134 engine; same3 intentional ignored
checks). It exited zero and cleaned its owned temporary folder. No unstaged
proposal, source or dependency was required. Clippy/format/bundle and the Sol
reviews are reused for byte-identical reviewed code. Only this documentation
receipt changed after that snapshot; staged source/configuration/test blobs are
compared with its tree before committing, and whitespace/link checks are repeated.
All29 changed local Markdown links/anchors resolve in the selected working copy;
approved untracked plans remain deliberately outside the checkpoint. The20
preserved README/proposal/review files still match their pre-staging hashes.
Final remote query at **2026-11-16 20:37:38 UTC** returned unchanged dc892f8.
The bounded local commit follows this evidence; nothing is pushed or published,
and no later stage or real-provider/financial capability is enabled.


## Step-5 planning and remaining acceptance path

### Selected checkpoint and published instruction activation

Planning-only handoff, 16 November 2026. Selected worktree is
`/Users/connormoulton/.t3/worktrees/metten/t3-5ff0c48e-8b502ba4b606ca11`, branch
`t3/brainstorm-issue-5`. HEAD is the approved step-4 checkpoint
`d593a13d9c175d747f44ca0b5e43b815656d12d2`, parent
`cc22fa9d2e4e749df0ef624aac89fa7cc745700e`; no movement required reconciliation.
The index was empty. The unrelated tracked docs/README edit and all 19 existing
proposal/review files were present and preserved. No repeated step-4 commit,
checkout/reset/rebase, staging, push or PR was performed.

Authoritative remote from repository metadata is
`https://github.com/moultonlavah/metten.git`. Independent live
`git ls-remote --symref <remote> HEAD` at entry resolved `refs/heads/main` to
**dc892f8e19c710cc2c435b43d90b14d440ff6730**; the timed refresh at
**2026-11-16 20:53:38 UTC** returned the same branch/commit. This is fresh remote
evidence, not origin/main. Objects were already available; no fetch/ref update
was needed. The current lead explicitly loaded immutable root `AGENTS.md` and
followed its `docs/agents/verification.md#published-instruction-discovery`.

Applicable published sources consulted at that same commit: root AGENTS,
verification (including discovery, bounded evidence reads, mixed checkpoints and
maintainability review), maintainability, CONTRIBUTING, code-map, issue-tracker,
triage-labels and domain guidance; also the actual lint config/npm commands,
Foundation workflow and issue-53 evidence for the newly applicable lint path.
Recursive published instruction discovery found root AGENTS only. Local discovery
found root AGENTS only, with no additional applicable ancestor/nested file;
Connor's standing and project instructions were also supplied directly in the
handoff. The older local entry lacks the published maintainability/discovery
pointers; the immutable published sources were used without importing unrelated
main code. The published frontend map points to view.ts/shared harness work that
is absent here: local view.mjs/view.d.mts and the actual VM callers were traced.
No missing required repository guidance or unresolved consequential process
conflict was identified. No new session/agent was started; this receipt proves
activation only for this lead, not installation/delivery to unknown sessions.

Local behavioral/ownership inputs consulted: product specification, ADR0001,
glossary, approved design and step-2/step-4 plans, local code map and verification
procedures, engine/native/package/frontend/tool/test contracts, source profiles
and U1–U7, current issue-5 index and relevant step-3/step-4/Sol/checkpoint receipts.
Consequential claims were checked against actual Workspace/admission/Quit code,
SourceWorker/SourceView/SourceHandle/Coordinator, transport/fence/control paths,
fixed provider credentials and Bootstrap activation, engine saved-detail and
requirement APIs, frontend refresh/credential/strategy handlers and VM callers,
runner modes/process tests, build command registration/capabilities and selected
lint/tooling files. These were bounded contract/source reads, not a whole-repo
code audit or new runtime verification.

Approved working-copy identity checks passed at entry and refresh:

| Input | SHA-256 |
| --- | --- |
| market-data-design.md | `21e2926329f67fef05b264dcb8a0dfca0455aaa697c753bfc4f2d95b19fe4bc7` |
| step-2-plan.md, revision 5 | `528b87824818515a4349ee99b658f9bc77ed2deb532e514b26f7d7ffa8b60a76` |
| step-4-plan.md, revision 2 | `c18c4d20b4f65890d49b4366455fbd77d701504830192d82df754614ad192322` |

Fresh read-only `gh issue view 5 --repo moultonlavah/metten` (including comments,
labels, assignee and Project items), native blocked_by/sub_issues REST and parent
GraphQL reads confirm: OPEN, all four criteria unchecked, zero comments, no
assignee, product / priority:high / ready-for-agent, Project Backlog, native/body
blockers #3/#4 closed, no children returned, no parent. Blocker states were
reconfirmed at the timed refresh. The body retains an old owner-form product
link; the canonical selected repository specification was read directly, not
silently rewritten. Owner exclusions override routine workflow mutation steps:
no tracker metadata, comment or closure was changed. Next actor is Connor,
reviewing this local plan; external access and later stages remain unauthorized.

### Concrete plan and maintainability disposition

Prepared [step-5-plan.md](../proposals/issue-5-egm/step-5-plan.md), revision 1.
It names four bounded gates: command/ownership integration, truthful Sources UI,
sole private packaged receiver/walkthrough, and affected checks/evidence. The
ordered later map separates offline exploratory calibration, frozen criteria,
exact-candidate qualification, actual documentation and externally authorized
acceptance. U1–U7 remain attached to affected criteria; no fixture grants a
freshness lease, quiet-history guarantee, reference mapping, entitlement or rights.

The expected path is real source owners/credential admission → finite native
projection → literal screen; the private validated receiver feeds the same decoder
and source publication path. No new source-state owner or descriptor reader is
planned. The existing native-only packaged credential check is explicitly reused
and extended at its existing validated bootstrap owner; it is not mistaken for
the future source-frame protocol or GUI/ordinary Keychain acceptance.

Two proposed inventory departures are explicit for owner review: a fixed display
constructor sharing the engine source constructor's registration checks (Connect
has no strategy selector, while current preparation requires a validated strategy),
and narrow adoption of published pinned Oxlint config/tools/npm commands plus its
existing workflow invocation. Neither is implemented. No synthetic saved strategy,
rule evaluator, source plugin framework, new runtime library or broad main-branch
frontend refactor is proposed. All new structure/checks have current named callers.

Ponytail and Impeccable planning guidance were consulted. Existing product and
visual implementation are the brief; planning adds no product/design setup
artifacts, hooks or redesign. Impeccable context was read for src/main.ts; no
UI implementation, detector run or browser verification was performed. Owner's
precise task and planning-only artifact scope govern any broader skill workflow.
No subagents, Council, independent review workflow or scores were requested/run.

### Evidence reuse and remaining limits

Completed evidence is reused from the step-4 checkpoint: 223 ordinary Rust tests
(89 native, 134 engine), strict workspace/all-target Clippy, formatting/whitespace,
frontend and Mac bundle build, staged tree bf9ac69d8ccbb40800737eaec8e5e7deebf20faa
with documentation-only receipt changes afterward, and two-round high-effort Sol
approval after corrected handoff regression. This task ran no application tests,
build, benchmark, Keychain access, provider call or packaged walkthrough. The plan
labels every new native/UI/protocol/integrated check proposed. Release-hour,
storage/arithmetic, Python and isolated native/package evidence is reusable only
for its unchanged boundaries and identified version; changed shared bootstrap/
runner/admission paths require the affected new evidence. Remote CI is unverified.

The dummy GUI blocker (inaccessible WebKit controls) remains. The plan does not
rerun that known blocked route solely for reassurance; future affected GUI proof
must be identified separately from native publication/IPC/credential proof.
Ordinary data-protection-Keychain entitlements/prompts and GUI acceptance remain
unproved. Real delivery/access/retention/legal clearance and production capacity
remain separate missing gates. No issue acceptance box was completed.

Planning completion checks and final freshness/preservation results follow below.


Final planning checks passed: all 15 local links/anchors in the new plan and
receipt resolve; approved-document hashes match; all 20 preserved README/proposal/
review inputs match the preservation manifest; selected HEAD/parent remain the
approved checkpoint and index remains empty. New plan/receipt whitespace and
`git diff --check` pass. The only task edits are the new untracked step-5 plan and
this record/index; the existing docs/README edit remains byte-identical.
Manual consistency review covered command selectors and positive decimal IDs,
source/credential/Quit ordering, fixed display ownership, one read in flight,
byte/deadline profiles, sole receiver/ack failure cases, lint path, U1–U7 and all
four acceptance criteria. This is a lead planning check, not independent code
review or executed future-test evidence.

Final delivery freshness query at **2026-11-16 20:55:35 UTC** returned unchanged
main/dc892f8e19c710cc2c435b43d90b14d440ff6730; no affected guidance repetition was
needed. Step-5 revision-1 SHA-256 is
`7582746f163c364e21f901cffc9f699f30263e7110932909fade88ad0f285a59`.
Stop here for Connor's review and explicit implementation authorization. A fresh
implementation session must independently refresh guidance and report its loaded
entry/revision/discovery; no activation of a future session is inferred here.


## Step-5 independent Sol plan review

Connor subsequently requested: “Review it with a Soul 6.1 subagent before
presenting anything to me.” Interpreted as GPT-6.1 Sol, consistent with the prior
step-4 reviewer. Invoked one native review subagent with the explicit
`gpt-6.1-sol` model override, a fresh bounded brief and no parent history.
Canonical task: `/root/step5_plan_review`. No other reviewer or workflow was
started. The brief authorized read-only plan/source investigation only; the
reviewer changed no files, index or refs and ran no implementation/build/runtime
checks. This request authorizes review, not step-5 implementation.

**Verdict: APPROVE — no blocking or should-fix finding in revision 1.**
Reviewed plan SHA-256 remains
`7582746f163c364e21f901cffc9f699f30263e7110932909fade88ad0f285a59`;
HEAD remains `d593a13d9c175d747f44ca0b5e43b815656d12d2`. All four supplied
plan/design identities matched independently. No correction/re-review was needed.

The reviewer independently loaded published root AGENTS at the pinned
`dc892f8e19c710cc2c435b43d90b14d440ff6730` and traversed discovery/verification,
maintainability, CONTRIBUTING, map, domain, issue/triage and SECURITY guidance,
plus applicable local instructions, product/ADR/glossary, original plans/protocol,
evidence, component contracts and consequential source/callers. Published/local
discovery found root AGENTS only. It reported no unavailable required process
guidance or consequential instruction conflict. Newer published frontend layout
and lint requirements were reconciled against this older worktree's actual files.
This explicitly refreshes the targeted reviewer session; no other-session or
shared-host delivery claim follows.

The lead accepts the review after reading its source-grounded dispositions:

- Fixed display initialization is justified: existing `SourceState::new` and
  `SourceWorker::prepare` derive from a validated strategy, whereas independent
  named-source Connect has no strategy selector. Sharing bounded registration
  checks without a fabricated strategy or competing state owner is appropriate.
- Narrow pinned lint adoption is required by current published guidance. Actual
  view.mjs/view.d.mts coverage probes are required, rather than assuming main's
  view.ts inventory covers this layout.
- Command admission/moved ownership and persistent revocation address the actual
  current Quit take-owner assumption. Blocking native operations, retained joins,
  one total source deadline and fresh-generation refusal are concrete checks.
- Independent Kalshi/BRTI desired membership and stream fencing preserve shared
  socket ownership; restarting that socket is not accepted as scoped disconnect.
- Exact decoding, decimal-string wire fields, encoding caps and projection-time
  fence checks cover the gap between internal SourceView and native IPC.
- Private source protocol matches the approved design: one validated receiver,
  immutable isolated mode, no production fallback and acknowledgment only after
  accepted or explicitly invalid publication. Existing credential-only checks,
  enqueue success and revision changes cannot substitute for that proof.
- Proposed tests expose wrong outcomes and request counts; evaluator evidence
  reuse remains conditional on unchanged assumptions. Calibration, frozen
  qualification, U1–U7 and actual GUI/Keychain/account/rights acceptance remain
  separate gates.

Remaining implementation risks are the mutation/Quit handoff, shared-socket
membership changes and cancellation of completion replies without late
publication. The approved plan requires focused failure checks for each; this
review executes none of that behavior. WebKit GUI and ordinary data-protection
Keychain acceptance remain unproved. Approval establishes a viable proposed
approach, not implementation correctness, external acceptance or issue closure.

Lead delivery query at **2026-11-16 21:55:41 UTC** again resolved authoritative
main to unchanged dc892f8e19c710cc2c435b43d90b14d440ff6730. No guidance change
requires affected re-review. Final plan identity, receipt links/whitespace and
preserved-work checks passed. Only this existing evidence record/index changed
for review delivery; the plan and application sources were not edited. Stop for
Connor's review and implementation authorization; no commit or publication.


## Step-5 authorized implementation

Connor approved the independently reviewed revision-1 plan: “I approve. Implement step 5,” and explicitly reiterated alignment with coding/maintainability standards. Selected HEAD remains d593a13d9c175d747f44ca0b5e43b815656d12d2. Preserve existing evidence/proposal files and unrelated docs/README edit; no commit/publication or step 6 is authorized.

Fresh remote/default query resolves https://github.com/moultonlavah/metten.git main to f4a22ea67fbf26eb57fc0f98fb19bfba47166150. Loaded immutable root AGENTS, discovery/verification, maintainability, CONTRIBUTING, domain, code-map and applicable issue/triage guidance. Relative to the reviewed dc892f8 baseline only unrelated workflow-map navigation changed in applicable guidance. Use published lint requirements with this selected worktree's actual mjs/d.mts layout. Objects already existed; no fetching/tracking-ref/index/HEAD movement. Current issue remains open, all criteria unchecked/no comments, blockers 3/4 closed.

Expected change path is the approved plan's inventory: shared fixed display registration → existing native worker/coordinator/credential owners → finite native projection/admission → Sources interface, with the sole validated private receiver feeding actual decoder/publication. Keep one owner per mutable fact; preserve strategy arithmetic, financial semantics, SQLite revision3, pinned toolchain and unrelated work. Required meaningful checks are plan gates1–4. The published source discovery is activated for this lead; no additional agent or free-model dispatch is being started.

Implementation inventory refinement: scoped disconnect/reconnect also reaches the existing Kalshi decoder's subscription identity owner (`providers/kalshi.rs`). A retired SID must release its bounded slot and book baseline without resetting the shared BRTI stream; late data for a disabled logical subscription must be ignored without revoking the shared provider. This is necessary for the approved logical-disconnect contract, not a new provider feature. Add a focused reconnect/late-SID regression; keep original plans unchanged.

Owner steering during implementation: “/unslop this ui after you are done or after a first pass,” followed by “And using our branding standards.” After the first functional pass, read the accepted brand README, specifications and complete guide; reused the unchanged approved primary SVG at255.42px canvas (~240px visible), cobalt/ink/chalk and Inter400/500/600. The authorized visual pass moves Sources first, replaces repeated cards/technical row output with flat concise rows and disclosures, and preserves all existing controls. A pinned, unmodified Inter runtime font and its upstream OFL/provenance were added under assets/fonts; no artwork or master geometry was changed. This explicitly expands the initial plan's legacy-green preservation instruction under the owner's newer branding request.

Focused implementation corrections included: initial controlled dispatch demonstrated the startup membership/token race (failed before, passed after publication acknowledgment); shared-control ledger caught an extra unsubscribe on a blocked-but-enabled stream (corrected, seven-control regression passes); source SID retirement handles logical reconnect without a new socket; pending Connect captures a checked intent revision so immediate Disconnect cannot be undone. The watch record carries mandatory revocation revisions, and an off/on coalescence regression confirms old permission is not restored. No new source-state owner, descriptor reader or per-message task was introduced.


## Step-5 final implementation receipt

**Implemented; packaged Sources GUI acceptance remains blocked.** This is an
uncommitted working-copy checkpoint on d593a13d9c175d747f44ca0b5e43b815656d12d2,
not an accepted release or completed issue. Connor's implementation approval and
later visual/branding steering are recorded above. The independent Sol approval
covers the unchanged plan only; the lead reviewed the implementation and its
connected callers. No independent implementation review was requested or claimed.

### Final discovery and preservation

Delivery query at **2026-11-16 23:49:01 UTC** independently resolved authoritative
https://github.com/moultonlavah/metten.git, default main, to unchanged
**f4a22ea67fbf26eb57fc0f98fb19bfba47166150**. Loaded published root AGENTS and its
verification/discovery, maintainability, CONTRIBUTING, code-map, issue/triage,
domain and security guidance, with applicable local root instructions and original
product/ADR/design/plans/component contracts. Only root AGENTS was found locally
and in published discovery. No required guidance is missing or consequential
instruction conflict unresolved. This older worktree uses its actual mjs/d.mts
frontend; no main-branch frontend refactor was imported. Tracker mutations remain
outside authority. No other session's activation or host-wide enforcement is
inferred.

All **21** files in the implementation-entry preservation manifest remain
byte-identical, including unrelated docs/README and every pre-existing proposal
artifact. The approved design, step-2/4 plans and reviewed step-5 plan still match
their recorded SHA-256 identities. HEAD/parent, existing refs and empty index are
preserved; no commit, push, PR, issue write or closure. No application state,
ordinary credential item or provider/account service was accessed by verification.

Candidate identity: **42** changed/new implementation, test, tooling, asset
and owning-contract paths, excluding this evolving receipt, unrelated docs/README
and preserved docs/proposals. Sort their relative paths, concatenate each
`SHA256(file)`, two spaces, path and newline, then hash that UTF-8 manifest:
**3e4a44f922f6557dd2823578ef0aa1949444dbfe72c72a88994d39433791842b**. Paths come from the union of
`git diff --name-only --diff-filter=ACM` and
`git ls-files --others --exclude-standard`, with the exclusions just stated.
This is a working-copy identity, not a Git tree or reviewed commit.
Finished bundle executable SHA-256:
**5316ca3988ec0ea9a1c226a939c93e959d75908e21e41f6899fe8474a8b88a03**.
Bundle size reported by Tauri: **17.12 MiB**. Only documentation changed afterward.

### Implemented route and maintained bounds

The [code map](../agents/code-map.md#step-5-source-setup-and-verification-route)
and [native contract](../../src-tauri/src/README.md#step-5-source-commands-and-isolated-receiver)
lead to the real owners and checks. Fixed engine display registration reuses the
existing validation owner without a fabricated strategy or evaluator. Native
`sources` and `provider_credential` decode exact finite requests, use fixed Demo
configuration, sanitize failures, and expose only local-main permissions. Local
snapshot/details/requirements reads do not connect or make provider requests.
No source activity starts merely by opening, refreshing, inspecting, saving a key
or starting the existing controlled session.

The same Coordinator, credential owner, SourceWorker/SourceState and transport
own the mutable facts. Four logical sources share three connections; Kalshi/BRTI
share the credential/socket while disconnect fences only the requested logical
source. Exclusive mutation work moves owned handles into blocking work and
restores them on failure. Pending Connect cannot undo a later Disconnect; checked
revocation revisions survive off/on watch coalescence. Credential replacement
stops/joins shared work before mutation. Quit revokes before waiting, retains
unfinished handles and refuses a false successful save/exit.

The display catalog has eight requirement records, three initial underlying
bindings and at most nineteen bindings with two retained markets. Existing
32-MiB adapter payload admission, two-book/8192-level-side bounds, one raw queue
slot/two-second age ceiling, three task slots and the engine's 8-MiB/4096-work
boundaries remain unchanged. No financial arithmetic, strategy semantics,
SQLite revision, Cargo dependency or pinned toolchain change. Snapshot/details
encode to at most256KiB; a512-KiB shared reservation remains owned through response
serialization. Source fixture envelopes are at most256KiB with128-KiB lexical
payload, with640KiB reserved for frame/decoded overlap plus the separately charged
intake copy. These are application ownership arguments, not RSS or capacity
qualification. No raised limit or granted freshness/history capability supplies
missing provider proof.

The sole validated bootstrap/descriptor reader selects dummy or Sources mode.
Sources mode is immutable and its ordinary credential/production transport paths
are refused, including after EOF/protocol failure. Exact session, canonical
string sequence and bounded envelopes feed the real decoder/worker. Ack1 follows
accepted or explicitly invalid published native state; it does not mean healthy
data or GUI paint. Cancellation, failed/late acknowledgment and receiver failure
revoke fixture evidence and retain owned shutdown handles.

The interface serializes local reads at the approved one-second active-screen
cadence, discards stale replies, clears private-key input before invocation, and
renders provider facts literally. Last values, units, identities and receipt ages
remain distinct from freshness/history/rights. The visual pass uses the unchanged
approved lockup, cobalt/ink/chalk and locally bundled Inter400/500/600; Sources
comes first in flat rows with optional details. Existing session, saved strategy,
dummy credential, history and recording controls remain. The Inter file/license
have pinned official provenance in [assets/fonts](../../assets/fonts/README.md).

No runtime dependency, new source platform, duplicate state owner or descriptor
reader was added. Two pinned development lint packages and published correctness
configuration are the required narrow tooling addition; the existing Foundation
job runs the same command. Actual mjs, declaration and test coverage were checked;
a temporary floating typed promise and unreachable JS probes failed as intended,
and handled/clean code passed after exact-byte restoration.

### Executed checks and limits

- `cargo test --workspace --locked --quiet`: **237 ordinary tests passed**
  (102 native,135 engine); one isolated Keychain test and two release-hour
  fixtures remain intentionally outside this ordinary run. Native real-owner
  tests cover finite/oversized inputs, exact decimal IDs, local-only inspection,
  main-window authority, scoped membership/SID retirement, shared credentials,
  pending Connect cancellation, invalid decoder publication and production refusal.
- Real Unix-socket protocol tests passed: wrong mode/session/provider/kind/field,
  duplicate/skipped/noncanonical sequence, oversize/partial/deadline framing,
  dispatch-before-ack ordering, no controlled connection, late completion,
  EOF/Quit during dispatch and failed acknowledgment. Invalid data is explicitly
  invalid before successful acknowledgment through the real worker test.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`,
  `cargo fmt --all --check` and `git diff --check`: **passed**.
- `npm run lint`, `npm run check`: **passed**, including all18 interface tests.
  These exercise actual frontend entry points for literal rendering, immediate
  secret clearing, serial active-only reads, exact large revisions and stale
  success/failure/action suppression. Existing session/strategy/dummy cases pass.
- Locked frontend build and `npm run tauri build -- --bundles app`: **passed**
  on the final application source, with local SVG/font artifacts emitted.
- `python3 tests/credential-runner.test.py`: **passed** on the final Swift runner,
  covering all three mode selectors, lexical framing/sequence, rejected/late/
  closed acknowledgments, retained child ownership, noisy output and cleanup.
- Finished bundle `--provider-credentials`: **passed** save/duplicate/replace/
  fixed-read/local signature/remove, actual process exit, marker scans and owned
  disposable Keychain/directory deletion. Default/search-list metadata remained
  identical after cleanup. This uses generated controlled material only.
- Finished bundle `--sources`: **BLOCKED**, not passed. The runner opened Sources
  and its credential disclosure, then could not find `provider-key-id` through
  WebKit Accessibility. No generated key was entered and no source frame was
  dispatched. Guarded window-close confirmed successful actual app exit; exact
  owned disposable Keychain/directory were deleted and global metadata stayed
  unchanged. No bypass, app kill or unchanged dummy-GUI rerun followed.
- Controlled headless rendering passed at1100,640,390 and320px, including locally
  loaded Inter, approved lockup size, keyboard focus and no horizontal overflow.
  Rendered desktop/mobile output was visually inspected. This used the explicitly
  resolved automation-only Chromium headless shell and a disposable profile;
  Browser.close, actual exit and profile deletion were confirmed. Its local native
  response shim establishes presentation only, not packaged IPC or WebKit access.

The optional Impeccable scan reported preserved proposal HTML warnings, a CSS-import
heading-size false positive and a generic Inter warning. The proposal is unrelated
and unchanged; actual rendered headings were32/20px, and Inter is owner-required
branding. No mechanical clean-scan or whole-product branding claim follows.

Reused evidence: unchanged engine arithmetic/history and release-hour fixtures,
existing storage/revision3 and staged-snapshot-helper behavior retain their
recorded step-4/earlier evidence. The display constructor and native/UI seams got
fresh focused/full ordinary checks. No new staged export is claimed with an empty
index and no checkpoint commit authorized. Shared bootstrap/protocol changes got
fresh dummy/native tests and process regressions plus the packaged credential
check. The known-inaccessible dummy GUI remains blocked; its old walkthrough was
not repeated. Ordinary data-protection-Keychain entitlements/prompts and GUI
acceptance are not established by the isolated native check.

### Remaining gates and acceptance

First obtain the missing ordinary packaged Sources/UI evidence, including visible
secret clearing, credential lifecycle, four logical sources, scoped disconnect,
invalid frame visibility and successful final guarded Quit. The narrow blocker
is WebKit Accessibility field access in the owned test app. Resolve that adapter
or perform an explicitly owned manual walkthrough of this exact bundle; preserve
isolation and acknowledgment semantics. Unit/process/render results cannot replace
that gate. Independent implementation review remains a separate possible owner
request; only plan review is currently independently approved.

After the bounded implementation and missing integrated proof are accepted,
step6 begins separately authorized exploratory offline calibration. Record all
failures, then freeze workload/environment/byte-age limits/numeric thresholds
before a distinct qualification run on the exact candidate. Include active-screen
contention, parser/book sizes, simultaneous ownership, queue age/loss, completion
delay, resource plateau, rollover and request/control counts. Preserve limits and
strategy observation semantics; never filter failures or raise limits for a pass.
Finish actual contracts, owner setup guide and criterion-specific evidence through
the ordered [remaining-work map](../proposals/issue-5-egm/step-5-plan.md#ordered-route-after-step-5).

U1–U7 remain unresolved in the [source capability register](../sources/README.md).
Public official documentation can support bounded research; actual account/data
availability, owner-selected policy, usage/legal rights and external qualification
need their explicit decisions or separately authorized access. Missing proof stays
attached to the affected criterion. All four issue acceptance criteria remain
unchecked/unaccepted; no real-provider delivery, ordinary Keychain/GUI acceptance,
remote CI, production capacity, account access, trading or publication is claimed.

Final affected Markdown paths/anchors and whitespace passed after the receipt update.
All21 preserved-file identities, the42-path candidate manifest, empty index and
unchanged HEAD were rechecked. No owned packaged app, runner or test browser
remained running at delivery. All changes after the final application build were
documentation only; no repeated application run is needed for those edits.


## Step-5 owner-authorized checkpoint commit

Connor subsequently instructed “Commit it.” This authorizes a local checkpoint
commit of the implemented step-5 scope and this evidence, not publication, step6
or issue acceptance. Fresh authoritative remote/default query again returned
main/f4a22ea67fbf26eb57fc0f98fb19bfba47166150. Root AGENTS and the published
verification/checkpoint procedure were reread; the unchanged applicable discovery
set and previously consulted standards remain the baseline. No new missing
guidance or consequential conflict. The42-path candidate manifest still matches
the final implementation receipt; all21 preserved files remain identical.

Stage only those42 paths plus this record. Exclude unrelated docs/README and all
proposal/review artifacts. Verify the disposable staged snapshot using the
existing helper before committing. Earlier application/build/package evidence
remains evidence for byte-identical code; the packaged Sources GUI blocker and
all external acceptance limits remain unresolved.

Disposable staged snapshot **79b9763bc1b2b074410d7f9ddc69a11d3833b331** passed
exact identity checks for all43 staged paths, offline locked npm installation,
correctness lint, all18 interface tests, frontend build and all237 ordinary Rust
workspace tests. The helper removed its disposable export after success. Only
this documentation receipt changed afterward; no application/test/tooling/asset
bytes changed. Reused Clippy/formatting/Mac bundle/native credential evidence
remains applicable to the identical code. The final staged export rechecks that
identity and whitespace with this receipt included, without rerunning unchanged
behavior. Delivery remote refresh again returned unchanged main/f4a22ea67.
The checkpoint preserves the explicitly recorded Sources GUI blocker.
