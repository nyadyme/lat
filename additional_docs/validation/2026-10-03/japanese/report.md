# Japanese profile — validation report

*Reconstructed verbatim from the annotating agent's final message; the agent
was not permitted to write report files. Sentence ids refer to `manifest.json`.
No separate lens-check file was written for Japanese; the lens counts below
are the only record.*

All seven `fires` verdicts hold. The weak points are the marker texts, one of
the two K4 substitutions, and the K7 marker, which matched nothing.

**Corpus (7 URLs).** 60 Wikipedia sentences, 12 consecutive from each article
start: 鎌倉幕府 (history), 光合成 (science), 消費税 (economics), 夏目漱石
(biography), 茶道 (culture). 40 news sentences, 20 consecutive from each of two
editorials dated 2026-10-03: Tokyo Shimbun and Akita Sakigake. Okinawa Times
was paywalled and replaced by Akita Sakigake; the 消費税 lead was split by
hand; verbless lead sentences were kept so the run stays consecutive; the
romanization is machine-made (pykakasi) and unreviewed.

| | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 46 | 14 | 12 | 28 |
| K3 | 44 | 0 | 50 | 6 |
| K4 | 34 | 3 | 22 | 41 |
| K5 | 13 | 1 | 50 | 36 |
| K6 | 30 | 1 | 38 | 31 |
| K7 | 0 | 5 | 4 | 91 |
| K8 | 78 | 4 | 4 | 14 |

**Proposed changes — clear**
1. K2: add a collective or non-actor subject (*kuni wa…*, *kitei wa kinshi
   suru*), and limit drops to "a dropped subject the preceding sentence does
   not supply". 13 misses had such a subject; 10 of 12 marker_only were
   resolvable zero anaphora.
2. K3: add *to sareru*, *to iwareru*, *to mirareru*, *ni yoreba / ni yoru to*.
   *Sō da / rashii / yō da* occur 0 times in 100 sentences; the other devices
   12 times. The current marker fires on 94 of 100.
3. K5: drop *-te iru* (22 sentences, 0 triggers); use *-ta*, a continuative or
   a verbal-noun ending over a process that ran for a span, where *-te kita*,
   *-yō ni natta* or *shidai ni* was available.
4. K6: belonging *no* only, plus *motsu* and *ni … ga aru*. 38 *no* hits were
   not possessive; 3 possessions were carried by a verb (14, 83, 91). The note
   "*no* joins nouns under any relation" describes the K8 trigger.
5. K7: no sentence had both trigger and marker. All 4 *tame* hits meant
   purpose. The 5 triggers used *ni yotte*, *koto de*, *ni tomonai* (twice),
   *o riyū ni*; add those plus *ni yori* and *koto kara*.
6. Delete the substitution Hungarian → Hausa and keep Hungarian: in 8 of 10 K4
   sentences it still forces a choice the sentence never made (幕府, 朝廷,
   北条氏, 平氏). *-tachi / -ra* occurs once in 100 sentences and cannot attach
   to institutions.

**Tentative**
- K4: add proper-named institutions, clans and states acting as one unit (7
  misses at span level).
- K8: add *N no N* (3 sentences).
- Yucatec → Spanish DOM: removing Yucatec is right (0 of 10), but Spanish DOM
  reaches only 1 of 10 K4 sentences (3 of 37 corpus-wide), because the K4 noun
  sits in subject or topic position. Hausa scores 8 of 10.

**Lens verdicts.** Tuyuca (K3) 10 of 10. Mongolian (K6) 10 of 10, but only 2
non-trivial (自分 appears 0 times). Russian (K5) 10 of 10. K5: Japanese marked
a process as gradual in 5 of 19 such sentences, always optionally; `fires`
holds. Tagalog (K2, unprompted) flagged 7 of 10 by the literal test; no change
recommended, since the same test would flag German.

**Doubts.** The K3 line between report and general knowledge; whether a dropped
subject counts as recoverable (K2); K4 judged by role, not form; two-character
words and proper names add noise to K8; *ni tomonai* read as causal; one
annotator, one pass.
