# Chinese (Standard Mandarin) profile: validation report

*The annotating agent was not permitted to write report files. This report was
reconstructed from its final message. Sentence ids refer to `manifest.json`.*

There was one annotator, one pass and no second rater.

All seven `fires` verdicts hold. The weak points were the marker texts:
- K4 was restricted to agent and topic slots.
- K5 and K7 missed the forms Mandarin actually used.
- K6 missed possessors that were dropped or joined without *的*.

## Corpus (7 URLs)

The corpus has 100 sentences:
- **Wikipedia, 60 sentences.** 12 consecutive sentences from the start of each of five
  articles: 鸦片战争 (history), 光合作用 (science), 通货膨胀 (economics), 鲁迅
  (biography) and 春节 (culture).
- **News, 40 sentences:**
  - a Xinhua commentary of 2026-11-04 (ids 61–79, the whole body);
  - a DW Chinese press review of 2026-11-05 (ids 80–100).

Each news piece was the most recent on its index page.

Deviations:
- The Xinhua body has only 19 sentences, so DW supplies 21.
- Sentences 82–93 lie inside one quotation whose source is named in 81.
- Sentence 3 is ungrammatical in the source: it has no 是.
- The corpus was built before annotation began, by a separate script, from
  the zh-cn variant of each article.

## Counts

These are the markers **before** revision.

| | both | trigger_only | marker_only | neither | recall | precision |
|---|---|---|---|---|---|---|
| K2 | 65 | 8 | 1 | 26 | 89% | 98% |
| K3 | 56 | 0 | 34 | 10 | 100% | 62% |
| K4 | 43 | 16 | 1 | 40 | 73% | 98% |
| K5 | 10 | 8 | 21 | 61 | 56% | 32% |
| K6 | 31 | 13 | 5 | 51 | 70% | 86% |
| K7 | 13 | 6 | 4 | 77 | 68% | 76% |
| K8 | 85 | 3 | 5 | 7 | 97% | 94% |

## Proposed changes — clear (all applied)

1. **K4: any slot.**
   - Change: the marker covers a population, mass or institution noun in any
     slot, including possessor and object. The noun may be bare or carry a
     demonstrative or group classifier (此类, 这类, 一群).
   - Evidence: 16 trigger_only (8, 14, 18, 22, 38, 39, 49, 51, 66, 68, 69, 71, 74–77).
2. **K7: more causal forms.**
   - Add: 因, 故, 由…(而)引发, 凭借, 结果 and 受…影响.
   - Note: count causal verbs only where one cause is named.
   - Evidence: 2, 4, 7, 10, 55, 59; on the note, 28 and 33.
3. **K5: bare narrative verb.**
   - Add: a bare narrative verb (often with 彻底, 不再, a date or a period).
   - Evidence: 1, 5, 6, 15, 16, 22, 44, 47.
   - Also: 了 and resultative compounds sat on bounded point acts in 21
     sentences, which are not the trigger.
4. **K6: possessors without 的.**
   - Add: a kin or inalienable noun whose possessor is dropped (37, 40, 41,
     44, 72).
   - Add: a possessor joined without 的 (9, 10, 13, 14, 20, 58, 74, 77).
   - Narrow: 有 counts only with an owner as subject (12, 31).
5. **K3: more hedges and a note.**
   - Add: 被认为, (很)可能 and 有可能 (2, 22, 28, 95).
   - Note: the surface test fired on 90 of 100 sentences; the qualifier
     "borrowed or inferred content" does the work.
6. **K8: verb–object modifiers.**
   - Add: a verb–object modifier with its noun: 便民服务, 施政方式 (67, 72, 94).

## Tentative

- **K2: 受…影响.** Add 受…影响 with a non-actor as the influence (43, 55).
  Applied as a tentative note.
- **K4: Hausa.** Hausa (pluractional verbs) reaches 4 of 10 here, against 8 of
  10 in Japanese. In the other sentences, 次, 多次 or 第一次 had already counted
  the event.

## Miss patterns

- **K2:**
  - The trigger_only cases are occurrence or stative verbs (11, 14, 36),
    organisms as subjects (18, 21), 受…影响 (43, 55), and a dropped subject
    whose referent is the content of the previous clause (42).
  - The one marker_only case is 85.
  - Occurrence verbs were not added to the marker, following the German note
    that they are not action verbs.
- **K3:**
  - All 34 marker_only cases are definitions, present facts or generic
    statements.
  - Named sources appeared as 认为, 相信, 写道, 注意到, 报道, 说 and 评价.
- **K4:**
  - In the trigger_only cases the noun is a possessor or modifier, an object
    or oblique, an appositive, or sits under a demonstrative or classifier.
  - The one marker_only case is 财政 (90).
- **K5:**
  - Written narrative Mandarin completes an event with a bare verb.
  - 了 and resultative compounds sat on point acts (爆发了, 判处了, 改为, 列为,
    取消了).
  - 着, 在, 一直, 逐渐 and 越来越 sat on a trigger verb in 0 of 18 sentences.
- **K6:** the marker_only cases are attributes (48, 66, 78) and 有 of an
  attribute (12, 31).
- **K7:** the marker_only cases are:
  - 使得 (15);
  - 导致 with several causes (28);
  - a reported 是…导致 (33);
  - 使 in a purpose clause (46).
- **K8:** the marker_only cases are N 的 N with nominalised heads (35, 48, 78,
  84, 100). This was an annotation choice, not a marker error.

## Verdict check

All seven `fires` verdicts hold.
- **K5:** 了 or a resultative compound sat on the trigger verb in 9 of 18
  sentences, always optionally.
- **K6:** the note "a tendency, not an obligatory split" holds. 的 was dropped
  for kin and institutions (3, 37, 40, 41, 47, 51, 54), and as often for
  non-kin compounds (9, 10, 14, 20, 74, 77).

## Lens check

"Forces" means the lens forces a choice the Mandarin sentence has not already
made. The per-sentence data is in `lens_check.json`.

| Lens | Combination | Forces | Verdict |
|---|---|---|---|
| Yucatec Maya | K4 | 0 of 10 | the removal is right |
| Hausa (pluractional verbs) | K4 | 4 of 10 | the replacement works, weakly |
| Hungarian (associative plural) | K4 | 7 of 10 | keep: 们 occurs 7 times and never on a K4 noun; "X他们" never occurs |
| Russian | K5 | 8 of 10 | keep |
| Finnish (partitive object) | K5 | 5 of 6 applicable | keep; 4 sentences have no object |
| Navajo (inalienable possession) | K6 | 8 of 10 | keep |
| Mongolian (reflexive possession) | K6 | 10 of 10 | keep; only 4 are non-trivial, and 自己 occurs 0 times |
| Classical Chinese (Wényán) | K7 | 10 of 10 | a contrast, not redundant: every K7 trigger sentence states the relation |
| Ancient Greek (article & substantivization) | K8 | 7 of 10 | keep: zero conversion (延续, 回应, 控制) is unmarked, and 的-nominalisation is rare |

On Wényán: Mandarin left a cause to juxtaposition only outside the trigger
sentences (47, 53, 73).

No other roster lens was redundant in 7 or more of 10. Tagalog came out at 3 of 10.

## Doubts

- **K3:** the line between report and documented fact. All history and
  biography events were counted as the trigger.
- **K2 and K4:** both were judged by role, so states count as non-actors.
  This leaves one inconsistency in the blind pass: 3 and 34 against 85.
- **K5:** whether a bare verb with 彻底 or a date presents a process as a point.
- **K7:** whether 随着 (17) is causal.
- **K8:** the blind pass did not count N 的 N.
- **Wényán:** the Classical forms 因 and 故 themselves carried the trigger in 2,
  10 and 59.
