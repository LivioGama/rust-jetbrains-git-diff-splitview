
📑 Prompt Spec — JetBrains Git Diff Viewer (Part 1 of N)

You are tasked with building a Git diff side-by-side viewer with precision and behavior modeled after JetBrains IDEs (IntelliJ IDEA, WebStorm, PyCharm, etc.). The goal is to replicate not just the visual appearance, but the functional mechanics, highlighting logic, and synchronized scrolling behavior. This spec defines the core layout, structural rules, and synchronization scaffolding.

⸻

1. General Principles
    1.	The viewer must display two panes:
          •	Left Pane → represents the “original” (base) file or revision.
          •	Right Pane → represents the “modified” (target) file or revision.
    2.	Both panes are vertically scrollable, and scrolling must be synchronized such that corresponding diff blocks remain visually aligned whenever possible.
    3.	The system must be capable of:
          •	Rendering millions of lines (with virtualization).
          •	Handling insertions, deletions, and modifications at both line-level and word-level granularity.
          •	Dynamically adjusting scroll mappings when file structures diverge (e.g., large deletions or additions in one side).

⸻

2. Layout Structure
   •	Container:
   •	Parent element hosting the two panes.
   •	Width is split 50/50 by default, but resizable with a draggable splitter bar.
   •	Splitter must be pixel-precise and allow continuous resizing without reflow glitches.
   •	Left Pane (Original):
   •	Monospaced font rendering.
   •	Line numbers visible in a gutter aligned to the left.
   •	Lines highlighted when part of a diff (see Section 5 for highlight rules).
   •	Right Pane (Modified):
   •	Mirrors left pane.
   •	Own gutter with line numbers.
   •	Aligns vertically with left pane using diff mapping.
   •	Central Connector Column:
   •	Narrow vertical band between left and right panes.
   •	Hosts curved connector lines that map diff blocks across panes.
   •	Connectors visually link corresponding changed blocks, even when displaced vertically.
   •	Connectors must support Bezier curves with smooth interpolation.

⸻

3. Line Numbering & Gutters
   •	Each pane has an independent gutter showing:
   •	Line numbers (incremented sequentially, no gaps).
   •	Change markers (color-coded icons for added/removed/modified).
   •	Gutters must remain fixed width to prevent content shift during scrolling.
   •	When a block of lines exists only on one side (e.g., new lines on right), the opposing gutter shows empty slots to preserve alignment.

⸻

4. Scroll Synchronization Basics
    1.	Anchor Blocks:
          •	Each diff block (add/remove/modify) is assigned as an anchor point.
          •	Anchors act as reference markers for scroll synchronization.
          •	Example: Line 20–25 on left aligns with Line 20–32 on right.
    2.	Mapping Algorithm:
          •	When user scrolls one pane, the system computes the proportional offset relative to nearest anchor blocks.
          •	Uses linear interpolation between anchors.
          •	Ensures that inserted/deleted blocks stretch or compress visually, keeping alignment intact.
    3.	Overscroll Handling:
          •	If one file is longer, the scroll range extends independently, but alignment persists until last mapped anchor.
          •	After final anchor, scrolling becomes free (unsynchronized) to allow exploring trailing unmatched lines.

⸻

5. Highlighting Philosophy (Overview — detailed rules in Part 2)
   •	Diff granularity:
   •	Line-level highlighting: entire line background tinted.
   •	Word-level highlighting: inline highlights within a changed line (precise tokens).
   •	Visual colors:
   •	Additions (Right only) → green background.
   •	Deletions (Left only) → red background.
   •	Modifications (both sides) → yellow/orange background for changed words.
   •	Neutral unchanged lines remain unhighlighted.
   •	Connector lines between panes inherit the same color (green, red, yellow).

⸻

6. Performance Requirements
   •	Must support incremental rendering (only visible viewport + margin buffer).
   •	Must support lazy connector drawing — only draw curves intersecting the viewport.
   •	Must handle files up to 50 MB efficiently.

⸻

7. Interactivity (Preview)
   •	Splitter drag → resizes panes dynamically.
   •	Mouse hover over connector → highlights both linked blocks across panes.
   •	Click connector → jumps scroll to center that diff in viewport.
   •	Keyboard navigation → next/previous diff block with arrow shortcuts.
   •	Context menu on gutter → “Accept change”, “Reject change”, etc.

⸻
Great 🚀 Let’s keep rolling, Livio.
Here’s Part 2: Highlighting & Diff Mapping in Extreme Detail.
This section is dedicated to the core logic of highlighting (line vs. word level), how diffs are calculated, how rendering happens, and how they synchronize with connectors. This will go very deep into the mechanics.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 2 of N)

You are tasked with building the highlighting and diff-mapping engine for the JetBrains-style Git side-by-side diff viewer. This section defines algorithms, rendering rules, tokenization strategies, and connector mapping.

⸻

1. Levels of Highlighting

There are two nested levels of diff highlighting:
1.	Line-Level Highlighting
•	Entire lines are tinted when they represent additions, deletions, or modifications.
•	This level communicates block-level structural differences.
2.	Word-Level (Inline) Highlighting
•	Inside a modified line, highlight only the precise segments that changed.
•	Example:

Original: const value = calculateSum(a, b);
Modified: const result = calculateTotal(a, b);

	•	value → highlighted red (deleted) on left.
	•	result → highlighted green (inserted) on right.
	•	calculateSum vs. calculateTotal → token difference highlighted.

Both levels must coexist without ambiguity. Line background always exists beneath word-level tokens.

⸻

2. Diff Calculation Algorithm

The diff engine must calculate structural and granular differences:
1.	Line Alignment
•	Use a Longest Common Subsequence (LCS) algorithm to align lines between original and modified.
•	Output is a sequence of operations: equal, insert, delete, replace.
2.	Inline Token Diffing
•	For lines marked as replace, run a secondary diff at character or token level.
•	Strategy:
•	Whitespace-aware: preserve alignment of indentation.
•	Tokenization: break line into tokens based on language syntax (identifiers, keywords, operators, literals, whitespace).
•	Run LCS again at token level.
•	Highlight additions and deletions at inline span granularity.
3.	Efficiency
•	Diff must be computed incrementally (streaming) to support very large files.
•	Use O(ND) diff algorithm (Myers’ diff) optimized for LCS with low memory overhead.

⸻

3. Highlight Rendering Rules

Rendering follows strict priority:
•	Background Colors (Line-Level):
•	Addition → soft green background spanning full line width.
•	Deletion → soft red background spanning full line width.
•	Modification → soft yellow/orange background spanning full line width.
•	Unchanged → transparent.
•	Foreground Inline Highlights (Word-Level):
•	Additions → bright green underline or background on tokens.
•	Deletions → bright red strike-through or background.
•	Modifications → highlight only changed substrings.
•	Combination Handling:
•	Example: Modified line with changed word inside → line background yellow, changed token highlighted darker yellow or with underline.
•	Whitespace Handling:
•	Trailing spaces must be visible with subtle gray dots when diff mode is active.
•	Whitespace-only changes (e.g., indentation) must be configurable: show or ignore.

⸻

4. Connector Lines & Block Mapping

Each diff block across left and right panes is connected visually:
•	Shape:
•	Smooth Bézier curves connecting start of block on left to corresponding block on right.
•	Gradient shading (transparent fade) ensures no visual clutter.
•	Coloring:
•	Additions → connector shaded green.
•	Deletions → connector shaded red.
•	Modifications → connector shaded orange/yellow.
•	Overlaps:
•	When multiple connectors cross, use z-index layering.
•	Hover interaction highlights selected connector and dims others.
•	Empty Block Handling:
•	If block exists only on one side (pure insertion or deletion), connector links to a ghost placeholder region on the other side.
•	Placeholder must show as empty gray block with dashed outline.

⸻

5. Synchronization with Scroll

Highlighting is deeply tied to scroll mapping:
1.	Anchored Mapping
•	Each highlighted block is an anchor point in scroll sync.
•	Scroll interpolation ensures aligned rendering:
•	Example: Left side shows 3 lines deleted → Right side shows 0 lines → Connector maps 3 lines to an empty block of height 3.
2.	Proportional Scaling
•	When one side has more lines, diff engine computes scaling ratio.
•	Connectors stretch vertically to preserve relative positions.
3.	Viewport Awareness
•	Only render connectors and highlights for visible lines.
•	Pre-calculate anchor positions to avoid jitter when scrolling.

⸻

6. Advanced Scenarios
   •	Moved Blocks
   •	Lines moved from one place to another should be detected separately (move detection).
   •	Highlight as blue connectors, distinct from add/remove.
   •	Example: function moved from line 200 → line 800.
   •	Collapsed Context
   •	Unchanged regions may collapse into a “…” placeholder.
   •	When collapsed, connectors must jump across collapsed space seamlessly.
   •	Clicking the placeholder expands lines and updates mapping.
   •	Inline Merge Markers (3-way diff)
   •	For future extension: support base/original/modified triple-view.
   •	Highlight merge conflicts with red/yellow mixed regions.

⸻

7. Accessibility Considerations
   •	All highlights must have color + texture/shape indicators (not color-only).
   •	Users with colorblindness must distinguish add/delete/modify by pattern (dashed underline, cross-hatch background, dotted outline).
   •	Keyboard navigation must move between diff spans, not just whole lines.

⸻

8. Testing & Validation
   •	Diff correctness tested against:
   •	Small trivial edits.
   •	Whitespace-only changes.
   •	Large blocks of insertions/deletions.
   •	Renamed variables in multiple places.
   •	Visual QA: ensure alignment of connectors with highlighted spans under all scroll positions.

⸻
Awesome — continuing with the deep-dive.
Here’s Part 3: Connector Geometry & Scroll Synchronization Algorithms.
This section specifies the math, data structures, rendering pipeline, and runtime behaviors for curved connectors and tightly synchronized scrolling across uneven diff blocks.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 3 of N)

You are implementing the connector geometry (curved links between panes) and the scroll synchronization layer. Precision matters: this spec defines coordinate systems, cubic Bézier construction, layout constraints, hit-testing, path clipping, level-of-detail (LOD), and the piecewise mapping function that keeps panes aligned even when insertions/deletions change heights.

0. Definitions & Coordinate Systems

0.1 Coordinate Spaces
•	CS_viewport: Top-left of each pane’s visible area is (0, 0); y increases downward.
•	CS_document: Absolute coordinates from the start of the file; independent from scrolling.
•	CS_canvas: Off-screen drawing surface used for connectors (central column).
•	It has its own vertical scroll tracking or is re-rendered on viewport scroll.

0.2 Geometry Primitives
•	Line height: lh (in CSS pixels). Identical on both panes (enforced).
•	Block: A contiguous range of diff lines:

Block = {
left: {startLine, endLine},   // inclusive, in base/original
right: {startLine, endLine},  // inclusive, in modified/target
kind: "add" | "delete" | "replace" | "move"
}


	•	Anchor: A mapping tie-point: the vertical centers of corresponding blocks, used to synchronize scroll.

Anchor = {
yLeft_doc: Number,   // document-space Y (px) at center of left block
yRight_doc: Number,  // document-space Y (px) at center of right block
weight: Number       // optional: affect interpolation stiffness
}



0.3 Sides & Columns
•	Left pane = Original; Right pane = Modified.
•	Connector column sits between panes; width Wc (configurable, default ~20–48 px).
•	Connector start x = xL = leftPaneInnerRightEdge
•	Connector end x = xR = rightPaneInnerLeftEdge

⸻

1. Connector Curves (Cubic Bézier)

1.1 Control Point Construction

For each diff block pair (BL, BR):
•	Start point P0 = (xL, yL) where yL is the vertical center of block on left (in CS_canvas).
•	End point   P3 = (xR, yR) where yR is the vertical center of block on right (in CS_canvas).

Use a cubic Bézier B(t) = (1−t)^3 P0 + 3(1−t)^2 t P1 + 3(1−t) t^2 P2 + t^3 P3, t ∈ [0,1].

Heuristic for control points:
•	Horizontal tension Tx: fraction of connector column width (Wc):

Tx = clamp( Wc * τx , min=6, max= max(14, Wc*0.75) )

with default τx = 0.35.

	•	Vertical easing Vy: reduce curvature when vertical displacement is large:

dy = yR - yL
Vy = sign(dy) * min( |dy| * τy , 0.75*|dy| )

with default τy = 0.10.

Control points:

P1 = (xL + Tx, yL + Vy)
P2 = (xR - Tx, yR - Vy)

This yields a smooth S-curve for big vertical shifts and a near-straight line for aligned blocks.

1.2 Monotonicity & Crossing Reduction
•	Slope constraint near endpoints: Ensure connectors leave P0 mostly rightward and arrive at P3 mostly leftward:
•	Enforce P1.x > P0.x + min(6, Wc*0.2) and P2.x < P3.x - min(6, Wc*0.2).
•	Jitter suppression: For nearby connectors with similar (yL, yR), apply small vertical offsets ±ε (1–2 px) alternating to avoid perfect overlap.
•	Crossing avoidance (optional):
•	For a group of connectors within the viewport, sort by (yL + yR) average. If two curves cross, swap z-order or slightly adjust Vy to reduce intersections.

1.3 Directional Encoding by Type
•	Color / pattern by kind:
•	add: green fill / gradient (or hatch A for accessibility).
•	delete: red fill / gradient (or hatch B).
•	replace: amber/orange (or hatch C).
•	move: blue/purple (or dashed stroke).
•	Fill vs. stroke:
•	Default: semi-transparent filled ribbon of width Rw = clamp( max(2, lh * 0.6), 2, 12 ) centered on Bézier.
•	If rendering ribbon is expensive, fall back to a stroked path with strokeWidth = clamp(2, lh*0.35, 6).

1.4 Connector to Placeholder (One-sided Blocks)

For pure additions or deletions:
•	Draw to a ghost placeholder rectangle on the opposite side:
•	Width: 80–100% of Wc.
•	Height: proportional to block line count × lh.
•	Style: dashed outline + light neutral fill, with the connector attaching to the center.

1.5 Hover & Selection Feedback
•	On hover: increase alpha, add outer glow, and highlight the linked blocks.
•	On selection: lock highlight; dim all other connectors to alpha = 0.2–0.4.

⸻

2. Rendering Pipeline & Performance

2.1 Virtualization

Render only connectors whose Y-range intersects the viewport (plus a buffer margin of ~4–8 lh above and below).
•	Maintain an index of blocks by yLeft_doc and yRight_doc for O(log n) range queries.

2.2 Caching & Reuse
•	Cache Bézier control points and bounding boxes per block; invalidate on:
•	Resize (pane widths or Wc changed).
•	Fold/expand operations altering line heights.
•	Font size or lh changes.
•	Clip each Bézier to the connector column’s visible rectangle (GPU accelerated if possible).

2.3 Level of Detail (LOD)
•	High LOD (zoom ≥ 1.0): render ribbons with anti-aliasing.
•	Medium LOD: stroke-only with rounded caps.
•	Low LOD (zoom out / minimap): straight quadratic approximation or even linear segment.

2.4 Hit Testing
•	Compute distance-to-curve via recursive subdivision or analytic Bézier distance approx:
•	Threshold: max(6 px, 0.5 * Rw).
•	Pre-check: use the curve’s AABB (axis-aligned bounding box) for broad-phase.

⸻

3. Scroll Synchronization — Mathematical Model

3.1 Piecewise Linear Mapping Function

We define a mapping F: yLeft_doc → yRight_doc, continuous and piecewise linear between anchors.

Given Anchors = [A0, A1, ..., Ak] sorted by yLeft_doc:
•	For any y such that Ai.yLeft_doc ≤ y ≤ Ai+1.yLeft_doc, map by linear interpolation:

t = (y - Ai.yLeft_doc) / (Ai+1.yLeft_doc - Ai.yLeft_doc)
F(y) = lerp( Ai.yRight_doc, Ai+1.yRight_doc, t )


	•	Before first anchor (y < A0.yLeft_doc): constant slope = slope of first segment.
	•	After last anchor (y > Ak.yLeft_doc): constant slope = slope of last segment.

The same notion is mirrored for G: yRight_doc → yLeft_doc to allow bidirectional scroll sync.

3.2 Anchor Construction

For each diff block (BL, BR):
•	Compute block centers:

cyL = ( yStartLeft_doc(BL) + yEndLeft_doc(BL) ) / 2
cyR = ( yStartRight_doc(BR) + yEndRight_doc(BR) ) / 2


	•	Build Anchor(cyL, cyR).
	•	For large blocks, add auxiliary anchors at quartiles (25%, 50%, 75%) to stabilize interpolation over tall regions.

3.3 Handling Insertions/Deletions

When a left block maps to zero height on right (deletion):
•	The segment slope decreases; near the deleted block, F compresses Y to match the absence.
•	To prevent visual snap, apply localized easing (see 3.6).

When a right block has extra lines (insertion):
•	Segment slope increases across the block span (expansion).

3.4 Folded (Collapsed) Context

When unchanged regions are collapsed on either side:
•	Recompute affected anchors using effective visible heights.
•	Keep an unfolded-metric cache to restore anchors instantly on expand.
•	Mapping must remain continuous at fold boundaries (avoid jumps by inserting small “splice anchors” at fold edges).

3.5 Mixed Granularity

If word-level diffs are dense within a line cluster:
•	Optionally derive micro-anchors at sub-line resolution (e.g., 0.25 * lh steps) purely for visual connector stability, not for scroll mapping.

3.6 Smoothing (C¹ Continuity)

To avoid perceivable kinks at anchor joints:
•	Replace raw piecewise linear F with cubic Hermite interpolation per segment:
•	For segment [Ai, Ai+1]:
•	Derive tangents mL and mR from neighboring slopes (Catmull–Rom style with tension).
•	Interpolate:

F(y) = h00(t)*Yi + h10(t)*Li + h01(t)*Yi1 + h11(t)*Ri

where t normalized in [0,1], Yi = Ai.yRight_doc, Yi1 = Ai+1.yRight_doc,
basis functions:

h00(t)= 2t^3 - 3t^2 + 1
h10(t)=   t^3 - 2t^2 + t
h01(t)= -2t^3 + 3t^2
h11(t)=   t^3 -   t^2


	•	Li = mL * ΔyLeft, Ri = mR * ΔyLeft (scaled appropriately).

	•	Clamp local slopes to avoid over/undershoot (monotone cubic interpolation if needed).

⸻

4. Scroll Controller & Interaction Rules

4.1 Master–Follower Model with Hysteresis
•	The pane that receives direct user scroll becomes Master; the other is Follower.
•	Hysteresis window: after the last master scroll event, keep master status for Δt_hys ≈ 400–700 ms.
•	While a pane is Master, its scrollTop_doc is not overridden by mapping from the follower.

4.2 Event Loop (Pseudo)

onScroll(pane):
now = time()
if pane != state.master && (now - state.lastMasterTime) > Δt_hys:
state.master = pane

if pane == LEFT:
yL = left.scrollTop_doc + left.viewportCenterOffset
yR_target = F(yL) - right.viewportCenterOffset
right.scrollTo_doc( easeTowards(yR_target) )
else:
yR = right.scrollTop_doc + right.viewportCenterOffset
yL_target = G(yR) - left.viewportCenterOffset
left.scrollTo_doc( easeTowards(yL_target) )

state.lastMasterTime = now

	•	easeTowards() blends current follower position with target via critically damped spring or exponential smoothing:

y_next = y_curr + α * (y_target - y_curr), with α in [0.25..0.6]

to avoid micro-jitter on high-frequency wheel events.

4.3 Edge & Overscroll Behavior
•	If master is at top/bottom hard limit, follower still seeks mapped position.
•	If follower mapping lands beyond its scrollable range (due to uneven lengths), clamp to [0, max].
•	Past the last anchor (tail of a much longer file), decouple sync gradually:
•	Linearly fade α to 0 across the final K lines (e.g., K = 200 * lh), allowing independent exploration of trailing content.

4.4 User-Initiated Desync
•	Holding a modifier (e.g., Alt) while scrolling temporarily disables sync.
•	A visible badge “Desynced” appears; clicking it re-enables sync and recenters using the nearest anchor by viewport center.

⸻

5. Building & Maintaining the Anchor Map

5.1 Data Structures
•	anchors[]: sorted by yLeft_doc.
•	segments[]: half-open intervals [yLeft_i, yLeft_{i+1}) with precomputed:
•	slope = (yRight_{i+1} - yRight_i) / (yLeft_{i+1} - yLeft_i)
•	for Hermite: left/right tangents clamped to monotonic range.
•	Index: balanced binary search (or flat array + binary search) for O(log n) segment lookup by yLeft_doc.
•	Optional: Interval tree keyed by yRight_doc for reverse mapping.

5.2 Updates & Invalidations

Trigger rebuild or partial recompute when:
•	Diff recomputed (content changes).
•	Folding state changes.
•	Font size / lh changes.
•	Pane width or layout shift alters wrapping (if wrapping is enabled).
•	Accessibility mode toggled (patterns can change height if rendered as overlays).

5.3 Stability Guarantees
•	Anchor persistence: Each anchor ties to a stable “block id”. When content edits happen:
•	Re-match anchors by block id or LCS with anchor affinity (prefer to keep old anchors if matching content remains).
•	Temporal coherence: Animate mapping changes over ~120–200 ms to avoid sudden jumps in the follower pane.

⸻

6. Minimap & Overview Synchronization

6.1 Minimap Track
•	Optional thin vertical strip mirroring document height, colored with condensed diff markers.
•	The viewport window of each pane is drawn on its side of the minimap; a connector ghost may show the mapped viewport on the opposite side.
•	Clicking the minimap jumps the master pane; the follower follows via F/G.

6.2 Large-Gap Handling
•	If one side has a huge insertion/deletion, the minimap shows a gap bridge: a semi-transparent band with directional arrow indicating compression/expansion mapping across that region.

⸻

7. Edge Cases & Special Behaviors

7.1 Extremely Tall Blocks
•	Add quartile anchors; cap per-segment slope to avoid “rubber band” effects.
•	During fast scroll, throttle connector rendering (draw as simple straight faded bars) until idle.

7.2 Zero-Length Mappings
•	When block maps to zero height (full deletion), ensure continuity by placing twin anchors at yLeft = y0 ± ε with identical yRight, producing a local flat segment.

7.3 Mixed Wrap Modes
•	Recommended: disable soft wrap in diff panes for deterministic mapping.
•	If soft wrap is enabled:
•	Compute anchors in visual line units (wrapped lines) so lh stays uniform.
•	Recompute on width changes or font size adjustments.

7.4 DPI & Fractional Pixels
•	Maintain positions in float; snap paint to device pixels at draw time with subpixel AA on.
•	Ensure connector stroke widths are ≥ 1 device pixel after scaling.

7.5 RTL/LTR & Bidirectional Text
•	Mapping is y-only; unaffected by x text direction.
•	Word-level highlight overlays must respect BiDi embedding; connectors remain centerline-to-centerline.

⸻

8. Connector–Highlight Integration

8.1 Color Consistency
•	Connector color derives from the dominant change type within the linked blocks:
•	If mixed, choose by weighted fraction of changed lines; if tie, prefer replace > add > delete.

8.2 Hover Propagation
•	On hovering a connector:
•	Highlight both blocks with a focus ring (2 px outline) in the same color.
•	Dim non-related word-level highlights to 60–70% opacity for emphasis.

8.3 Click Navigation
•	Clicking a connector recenters both panes so that the block centers align to viewport center:

left.scrollTo_doc( y = cyL - viewportHeight/2 )
right.scrollTo_doc( y = cyR - viewportHeight/2 )


	•	Respect master–follower logic; for this interaction, temporarily force dual-master with a one-shot sync to the exact centers, then restore hysteresis.

⸻

9. Algorithms (Detailed Pseudocode)

9.1 Build Anchors From Diff Blocks

function buildAnchors(blocks, lh):
anchors = []
for b in blocks:
yL0 = (b.left.startLine - 1) * lh
yL1 = b.left.endLine * lh
yR0 = (b.right.startLine - 1) * lh
yR1 = b.right.endLine * lh

    cyL = (yL0 + yL1) * 0.5
    cyR = (yR0 + yR1) * 0.5
    anchors.push({ yLeft_doc: cyL, yRight_doc: cyR, weight: w(b) })

    if ( (yL1 - yL0) > 12*lh || (yR1 - yR0) > 12*lh ):
       // add quartiles for stability
       for k in [0.25, 0.5, 0.75]:
         anchors.push({
           yLeft_doc: yL0 + k*(yL1 - yL0),
           yRight_doc: yR0 + k*(yR1 - yR0),
           weight: w(b) * 0.6
         })

anchors.sortBy(a => a.yLeft_doc)
return smoothAnchors(anchors)

9.2 Build Segments & Hermite Data

function buildSegments(anchors):
segs = []
for i in 0..anchors.length-2:
a0 = anchors[i]; a1 = anchors[i+1]
ΔL = a1.yLeft_doc - a0.yLeft_doc
ΔR = a1.yRight_doc - a0.yRight_doc
slope = ΔR / max(ΔL, ε)

     // Tangents using neighboring slopes (Catmull–Rom)
     s_prev = (i > 0)              ? (anchors[i].yRight_doc - anchors[i-1].yRight_doc) 
                                     / max(anchors[i].yLeft_doc - anchors[i-1].yLeft_doc, ε) 
                                   : slope
     s_next = (i+2 < anchors.length)? (anchors[i+2].yRight_doc - anchors[i+1].yRight_doc) 
                                     / max(anchors[i+2].yLeft_doc - anchors[i+1].yLeft_doc, ε)
                                   : slope
     mL = 0.5*(s_prev + slope)
     mR = 0.5*(s_next + slope)

     // Clamp to monotone range
     [mL, mR] = clampMonotone(mL, mR, slope)

     segs.push({
       L0: a0.yLeft_doc, L1: a1.yLeft_doc,
       R0: a0.yRight_doc, R1: a1.yRight_doc,
       slope, mL, mR
     })
return segs

9.3 Evaluate Mapping F(y) with Hermite

function mapLeftToRight(y, segs):
i = binarySearchSegment(segs, y)  // find segment with L0 <= y < L1
if i < 0: return extrapolate(segs, y)

s = segs[i]
t = (y - s.L0) / max(s.L1 - s.L0, ε)
h00 =  2*t*t*t - 3*t*t + 1
h10 =    t*t*t - 2*t*t + t
h01 = -2*t*t*t + 3*t*t
h11 =    t*t*t -   t*t

ΔL = s.L1 - s.L0
return h00*s.R0 + h10*(s.mL*ΔL) + h01*s.R1 + h11*(s.mR*ΔL)

9.4 Scroll Synchronization Loop (Frame-Based)

function syncFollower(masterPane, followerPane, segs, dt):
// dt: elapsed ms since last frame; run at ~60 fps or on scroll events
yM_center = masterPane.scrollTop_doc + masterPane.viewportHeight/2
yF_target_center = (masterPane == LEFT)
? mapLeftToRight(yM_center, segs)
: mapRightToLeft(yM_center, segs) // build reverse segs similarly

yF_target_top = yF_target_center - followerPane.viewportHeight/2

// Easing
α = chooseAlpha(dt)  // e.g., α = 1 - exp(-k*dt), with k ≈ 0.02–0.06
followerPane.scrollTop_doc = followerPane.scrollTop_doc
+ α * (yF_target_top - followerPane.scrollTop_doc)


⸻

10. Testing & Validation Protocols

10.1 Visual Consistency
•	Alignment test: Place markers at centers of each diff block on both panes; ensure markers stay within ±1 px vertically under slow scroll and ±2 px under fast scroll.
•	Stress test:
•	10k+ line files, 30% random insert/delete blocks of random heights.
•	Track connector recalculation time < 1 ms per visible connector on average.

10.2 Interaction Smoothness
•	Jitter test: With mouse wheel and touchpad kinetic scrolling, measure follower “overshoot”; should converge critically within ≤ 180 ms.
•	Desync mode: Confirm follower remains stationary; re-sync button recenters within ≤ 120 ms.

10.3 Edge Case Matrix
•	Pure insertion of 500 lines on right at top, middle, bottom.
•	Pure deletion of 500 lines on left at top, middle, bottom.
•	Multiple adjacent replace blocks with small gaps (1–3 lines) of unchanged context.
•	Collapse/expand unchanged regions around anchors; verify no visible jump (animated remap allowed).

⸻

11. Accessibility & Theming

11.1 Non-Color Indicators
•	add: hatch pattern angled 45°.
•	delete: cross-hatch.
•	replace: dotted overlay.
•	move: dashed outline on connector.

11.2 High-Contrast Mode
•	Increase connector stroke width by 1–2 px.
•	Ensure min contrast ratio ≥ 4.5:1 for highlights against background.

11.3 Reduced Motion
•	Disable follower easing animation; snap to mapped positions but gently (single frame).
•	Keep connector fade-in/out disabled; use static rendering.

⸻

12. Implementation Notes

12.1 Rendering Tech
•	Web: Canvas 2D or WebGL for connectors; DOM for code lines + overlays for word-level diffs.
•	Desktop: Skia/Direct2D/Metal-backed drawing for connectors; text rendered in a monospaced layout engine.

12.2 Precision & Rounding
•	Keep anchor and mapping math in double precision.
•	Round only at paint time to device pixels; keep scrollTop in float for smoothness.

12.3 Threading
•	Precompute anchors and segments off the UI thread (Web Worker or background thread).
•	Main thread only receives immutable mapping data structures.

⸻

13. Developer Hooks & Extensibility
    •	onConnectorHover(blockId): surfaces metadata (file, line ranges, change stats).
    •	onConnectorClick(blockId): centers both panes on that block.
    •	onMappingRebuilt(prevVersion, newVersion): consumer can log performance, re-run tests.
    •	getMappingAt(yLeft|yRight): programmatic access to F/G.

⸻

14. Acceptance Criteria (Connector & Sync)
    1.	Connectors render within the central column with cubic Béziers that smoothly link corresponding blocks; no visible jaggies; anti-aliased.
    2.	Under normal scrolling, centers of mapped blocks differ by at most 1–2 px across panes.
    3.	When lines are collapsed/expanded, mapping recalculates without perceptible jumps (or with a short interpolation ≤ 200 ms).
    4.	Hovering a connector highlights both blocks and dims others; clicking recenters both panes.
    5.	In long-tail regions beyond the last anchor, follower gently desynchronizes, letting user explore independent tails.
    6.	Accessibility patterns are visible and unambiguous without color.

⸻

Great — here’s Part 4: a deep, implementation-ready specification for User Interaction Model & Editing / Merge Operations for your JetBrains-style Git diff side-by-side viewer. This part covers every user-facing behavior you’ll need to implement—precise event semantics, keyboard mappings, gesture behavior, selection semantics, applying/reverting hunks, three-way merge flows, undo/redo, staging, and accessibility-driven interactions. Write this into your agent pipeline and keep it next to Parts 1–3.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 4 of N)

This section defines how users interact with the diff viewer and how the viewer exposes programmatic hooks for merge and edit operations. It emphasizes determinism and testability: each interaction maps to a finite set of state transitions and commands.

⸻

0. Goals & Principles
    1.	Deterministic interactions — same sequence of user actions produces identical editor state transitions.
    2.	Idempotent operations when possible — repeated apply/revert of the same hunk should be handled safely.
    3.	Minimal surprise — prioritize predictable scrolling, selection, and commit staging behaviors.
    4.	Accessibility parity — every mouse/touch interaction must have keyboard equivalents.
    5.	Atomic hunk operations — applying or rejecting changes should happen at hunk granularity unless the user explicitly edits inline.

⸻

1. Interaction Model Overview
   •	Main interaction surface: two code panes + central connector column + gutters + context menus + command palette.
   •	Primary interaction types:
   •	Navigation (keyboard and mouse)
   •	Selection (line, multi-line, token)
   •	Edit (inline edit in modified pane)
   •	Hunk operations (apply/reject/accept/undo)
   •	Merge resolution (three-way conflict UI)
   •	Staging / Unstaging (Git index changes)
   •	Clipboard operations (copy/paste between panes)
   •	Drag & Drop (reorder/move hunks)
   •	Touch gestures (swipe to accept/reject on mobile/tablet)
   •	Command palette (text commands & fuzzy find)

⸻

2. Focus Model & Keyboard Navigation

2.1 Focusable Elements

Focusable elements (tab-order):
1.	Left gutter (line numbers)
2.	Left code pane (text)
3.	Connector column (interactive region)
4.	Right code pane (text)
5.	Right gutter
6.	Minimap (optional)
7.	Command palette field

Focusable state should be visually clear with a 2px focus ring and ARIA aria-activedescendant semantics for inner virtualized rows.

2.2 Core Keyboard Bindings (default)

Provide configurable keymap. Defaults modeled on JetBrains with common alternatives (VS Code-style) available.
•	Navigation:
•	ArrowUp/ArrowDown — move caret / selection by visual line.
•	PageUp/PageDown — page scroll maintaining caret column.
•	Home/End — move caret to line start/end.
•	Ctrl+G — jump to line (prompt).
•	Diff block navigation:
•	F7 — previous diff block (center it).
•	Shift+F7 — next diff block.
•	Alt+Up / Alt+Down — previous/next connector (same as F7/F8).
•	Selection:
•	Shift+Arrow — expand selection linewise.
•	Ctrl+Shift+Arrow — expand selection by token/word.
•	Ctrl+A — select all.
•	Hunk operations:
•	Ctrl+Enter — apply current hunk (if caret inside a hunk).
•	Ctrl+Backspace — revert current hunk.
•	Ctrl+Shift+S — stage current hunk.
•	Merge:
•	Ctrl+M — open merge inspector on current conflict.
•	Alt+1/2/3 (within merge inspector) — choose base/left/right or accept combined resolution.
•	Undo/Redo:
•	Ctrl+Z / Ctrl+Y — undo/redo local editing in modified pane (tracked separately from git staging history; merging operations produce separate change steps).
•	Command palette:
•	Ctrl+P — open command palette (fuzzy commands).
•	Context:
•	ContextMenu or Shift+F10 — open context menu at current focus point.

All keyboard operations must be bindable and exportable as JSON keymaps.

⸻

3. Mouse & Pointer Semantics

3.1 Clicks & Selection
•	Single click inside a code pane places caret at visual character position.
•	Double-click selects word (token-aware).
•	Triple-click selects the entire line.
•	Shift + Click extends selection from caret to clicked position.
•	Click-drag selects a range; dragging across the split lines should allow selection to span both panes (maps to multi-range selection model).

3.2 Click on Gutter
•	Single click in gutter toggles breakpoint-like marker (user-configurable).
•	Click on change marker icon (gutter) opens the hunk context menu directly.
•	Shift+Click on gutter selects entire line range between last selected gutter line and current.

3.3 Click/Drag on Connector
•	Click on a connector — highlight the linked blocks.
•	Click+Drag vertically on connector — scroll both panes simulta­neously (acts as a pan handle).
•	Right-click opens connector-specific menu: Center both, Stage hunk, Collapse other hunks, Show diff details.

3.4 Hover & Tooltips
•	Hover on token highlight shows inline diff tooltip with:
•	Exact token change preview.
•	Option to accept token-level change to either side (if enabled).
•	Hover on a connector shows a compact tooltip with hunk metadata: lines changed left/right, author/commit id (if available), and a small preview snippet.

⸻

4. Context Menus & Commands

Context menus are granular and contextual; right-clicking different zones yields different menus.

4.1 Gutter Context Menu (line/gutter)
•	Copy line
•	Copy lines as patch (creates a patch text)
•	Stage hunk
•	Discard changes
•	Annotate (show blame)
•	Set breakpoint (editor integration)

4.2 Code Pane Context Menu
•	Cut / Copy / Paste
•	Copy as patch
•	Apply change to left/right (when caret inside modified token)
•	Stage selection
•	Toggle wrap
•	Format selection
•	Open in external editor (hook)

4.3 Connector Context Menu
•	Center both panes on this hunk
•	Accept change (apply right into left or accept right)
•	Reject change (apply left into right)
•	Stage hunk
•	Create patch file
•	Mark as resolved (for conflicts)
•	Move hunk (drag to reorder if supported)

Menus must be keyboard accessible via Menu key or Shift+F10.

⸻

5. Hunk Model & Atomic Operations

5.1 Hunk Definition

A hunk is the minimal contiguous range of changed lines (Left and Right) produced by the diff engine (replace, add, delete). Each hunk object:

Hunk = {
id: UUID,
leftRange: {startLine, endLine},
rightRange: {startLine, endLine},
type: "add"|"delete"|"replace"|"move",
tokens: [ tokenDiffs ]  // optional list of inline token diffs
metadata: {commitId?, author?, timestamp?}
}

5.2 Atomic Apply / Revert
•	Apply: copy right-range content into left document (or apply patch to current working tree). Apply must:
1.	Validate that the left text at leftRange matches expected baseline for safe patching. If not, display a conflict modal.
2.	Begin an atomic edit transaction: replace leftRange with rightRange.
3.	Emit onHunkApplied(hunkId, txId, result).
4.	Optionally stage the change (if user opted in).
•	Revert: copy left-range content into right document (if user wants revert in modified view), or reset modified pane to baseline version.
•	Undo/Redo: all apply/revert actions produce undo steps in both editor undo stack and change history stack (separate but synchronized).

5.3 Token-Level Patch Application (Optional)
•	If inline token diff is available, allow token-level application:
•	User selects tokens, chooses Apply token to other side.
•	System constructs a minimal patch for token replacement.
•	Validate context before applying; if context mismatches, block and surface conflict resolution options.

5.4 Staging vs. Committing
•	Staging an applied hunk marks it in the Git index UI. Provide clear UI states:
•	Unstaged changes — modified pane differs from HEAD.
•	Staged changes — user staged hunks.
•	Committed — commit applied to local repo.
•	Staging flow:
•	Stage hunk API call: stageHunk(hunkId) -> Promise<Result>.
•	After staging, gutter and hunk marker change visual state (e.g., small index icon).
•	Provide Unstage option.

5.5 Safety & Conflict Modes
•	Before apply/stage:
•	Optionally calculate a 3-way merge against common ancestor to ensure clean patch.
•	If conflict risk detected, open merge inspector (see Section 8).
•	If user forces apply despite conflicts, create conflict markers in the target file (typical Git <<<<<<< style) unless the user opts to auto-resolve using heuristic (whichever side wins).

⸻

6. Three-Way Merge & Conflict Resolution

6.1 Merge Inspector UI

Support a three-column modal or inline three-way split (Base | Left | Right) with the following features:
•	Each column is scroll-synced appropriately (three-way mapping).
•	Conflicts are highlighted and listed in a vertical navigator.
•	For each conflict:
•	Show Choose Left, Choose Right, Choose Base, Manual Edit, Combine.
•	Provide Accept Both where sensible (concatenate).
•	Provide Edit Combined — inline editor for the merged result.
•	Provide Auto-merge button which tries language-aware merge heuristics (whitespace normalization, renamed identifiers mapping) and shows summary of changes.

6.2 Conflict Actions & State
•	Mark as Resolved sets the hunk state to resolved and stages the merged result if requested.
•	Abort Merge reverts working tree to pre-merge state (with explicit confirmation).
•	Store per-conflict metadata: resolutionChoice, timestamp, userId, enabling audit/undo.

6.3 Merge History & Undo
•	Each conflict resolution step is recorded as a separate transaction and can be undone step-by-step.
•	A Revert Merge operation restores all files to pre-merge snapshot.

⸻

7. Editing Model in Modified Pane

7.1 Inline Edits
•	Modified pane is fully editable (unless read-only mode). Inline editing:
•	Changes are immediately reflected in the in-memory modified buffer.
•	Recompute diff incrementally around changed lines (local region), not whole-file.
•	Maintain hunk ID stability where possible (preferred: preserve hunk identity if unchanged outside edit area).

7.2 Save & Apply Changes
•	Save writes modified buffer to working tree file.
•	If user edits content that moves or alters existing hunks, the anchor map should be updated incrementally; reflow and recalc only adjacent anchors.
•	Auto-save behavior must be configurable.

7.3 Merge of Concurrent Edits
•	If both panes are editable (rare), edits in left pane are allowed only in specialized “two-way edit” mode. Default behavior: left pane is read-only to preserve original baseline.

⸻

8. Drag & Drop & Reordering

8.1 Hunk Dragging
•	Dragging a hunk from connector/gutter:
•	Allows reordering hunks within the same file (if semantics permit).
•	Drop target shows a preview insertion point with ghosted lines.
•	Validate context: if insertion breaks context, show warning and require confirm.

8.2 Drag-to-Apply
•	Drag a hunk from right pane to left pane gutter to apply (visual affordance: drop to apply).
•	Support Ctrl to copy instead of move (platform conventions).

8.3 Multi-hunk Drag
•	Support multi-select hunk dragging (Shift + click to select contiguous; Ctrl/Cmd to multi-select arbitrary).

⸻

9. Clipboard & Patch Operations

9.1 Copy As Patch
•	Copy as patch generates a unified diff snippet for the selected hunk(s) and copies text to clipboard.
•	Patch text must include context lines configurable by user (default 3 lines).

9.2 Paste Patch
•	Pasting a patch into a code pane should invoke importPatch():
•	Validate patch header; if safe, preview patch application.
•	Option to Apply as new hunk or Preview changes.
•	Provide API: applyPatch(patchText, options) -> Promise<Result>.

⸻

10. Touch & Mobile Gestures

10.1 Touch Scrolling
•	Two-finger scroll across connector pans both panes.
•	Single-pane one-finger scroll acts like master per heuristic.

10.2 Swipe to Accept/Reject
•	Swipe right on a hunk in the right pane — Accept (apply to left).
•	Swipe left on a hunk in the left pane — Reject (revert in right).
•	Swipes must have threshold and confirm intent by revealing affordance icon before commit.

10.3 Long Press
•	Long press on a connector brings up quick actions (stage, apply, center, copy patch).

⸻

11. Accessibility & Screen Reader Support

11.1 ARIA Roles & Labels
•	Code panes: role="textbox" with aria-multiline="true".
•	Each hunk: role="region" with aria-label="Hunk X: N lines changed".
•	Connector: role="link" with aria-label describing mapping (e.g., “Connector linking left lines 42–50 to right lines 43–55; 12 lines added”).

11.2 Keyboard-only Workflows
•	All operations available via keyboard sequences and command palette (including staging, patch copy, apply/reject).
•	Provide focusMode where tab cycles through hunks rather than inside text (helpful for reviewers).

11.3 Screen Reader Output
•	On hunk focus, read summary: number of changed lines, dominant change type, author/commit message.
•	On apply/revert, announce success/failure and provide undo hint.

⸻

12. Visual States & Affordances

12.1 Hunk Visual States
•	Unchanged — default.
•	Changed — highlighted (line-level).
•	Staged — icon in gutter + pale overlay.
•	Applied — ephemeral animation (e.g., flash) then persisted state.
•	Conflict — red glowing border + conflict marker.
•	Resolved — green tick in gutter + disabled apply action.

12.2 Animations
•	Animations must be subtle and optional:
•	apply and revert animations when hunk is applied: fade-in on new text + 120 ms slide.
•	collapse/expand context: smooth height animation ≤ 200 ms (reduce-motion disables).

⸻

13. Programmatic API & Events

Expose an API surfaced to hosting app or LLM agent. Prefer Promises and event callbacks.

13.1 Key Methods

getHunks(fileId) -> Promise<Hunk[]>
applyHunk(hunkId, options) -> Promise<{success, conflicts?}>
revertHunk(hunkId, options) -> Promise<{success}>
stageHunk(hunkId) -> Promise<{success}>
unstageHunk(hunkId) -> Promise<{success}>
applyPatch(patchText) -> Promise<{success, rejected?}>
openMergeInspector(hunkId) -> Promise<{result}>
exportPatch(hunkIds[]) -> Promise<string>
scrollToHunk(hunkId, options) -> void
centerOnLine(side, lineNumber) -> void
setMasterPane(side) -> void
toggleSync(enabled) -> void

13.2 Events

on('hunkApplied', ({hunkId, txId, result}) => {})
on('hunkStaged', ({hunkId}) => {})
on('hunkReverted', ({hunkId}) => {})
on('mappingRebuilt', (mapping) => {})
on('selectionChanged', (selection) => {})
on('conflictDetected', (hunkId, details) => {})
on('undo', (txId) => {})

Use consistent event payload shapes; include timestamp and origin (user, remote, API) for auditability.

⸻

14. Testing Matrix (Interaction)

Design an exhaustive interaction test suite—automated where possible.

14.1 Unit Tests
•	Keyboard navigation: ensure F7/F8 moves to correct hunk across varied file lengths and collapsed contexts.
•	Hunk apply/revert: applying a hunk in modified file with unexpected context should raise conflict.
•	Token apply: patching token-level changes updates only targeted spans.

14.2 Integration Tests
•	Stage → commit cycle: stage multiple hunks, commit, ensure git history updated.
•	3-way merge: simulated diverging changes, auto-merge, manual conflict resolution followed by staging.
•	Drag & drop reorder: drop validation and undo behavior.

14.3 UI Tests (E2E)
•	Touch gestures: swipe to accept/reject with threshold boundaries.
•	Accessibility: screen reader reads accurate hunk metadata, keyboard-only sequences cover all menus.

⸻

15. Error Handling & UX around Failures
    •	When apply/stage fails due to external reasons (file system lock, permission):
    •	Show a modal with error + retry/abort options.
    •	Provide an Export patch fallback to allow manual apply.
    •	On merge errors (unresolvable automatically):
    •	Provide Create backup and Open Merge Inspector.
    •	If a hunk changes while user is interacting (concurrent update from remote):
    •	Notify the user; attempt to preserve current selection/caret as best-effort.
    •	Provide Refresh mapping and Rebase pending changes options.

⸻

16. Security & Permissions
    •	When running in an embedded environment (e.g., cloud editor), all file operations must respect host sandbox permissions.
    •	Provide a dry run mode for all destructive operations (applyHunk, revertHunk, stageHunk) that returns predicted diffs without changing files.

⸻

17. Extensibility Hooks (LLM Agent Integration)

Design the UI to be automatable with an LLM agent:
•	All actions should have declarative names and stable IDs (action.applyHunk, action.stageHunk).
•	Expose a describeState() API that returns a JSON snapshot of:
•	Current hunks and statuses.
•	Anchor map.
•	Focused element and selection.
•	Last N user actions.
•	Provide simulateAction(actionName, params) for automated testing by agents (useful for LLM-driven UI automation).
•	Provide explainHunk(hunkId) that returns a concise machine-readable summary (diff stats, token diffs, commit metadata) and a human-readable summary.

⸻

18. Acceptance Criteria (Interaction & Merge)
    1.	Keyboard-only reviewer workflow completes with no mouse in ≤ 20 steps for average file (500 lines, 4 hunks).
    2.	Applying a hunk that matches baseline results in no conflict and an immediate staged change if requested.
    3.	Token-level apply works when contexts match; otherwise provides clear conflict resolution choices.
    4.	Merge inspector resolves conflicts and stages resolved hunks on Mark as Resolved.
    5.	Touch gestures and swipe thresholds are stable across device pixel ratios.
    6.	All operations emit logged events with structured payloads for telemetry and automation.

⸻

Perfect 🙌 Let’s keep building.
We’re now at Part 4: Connector Lines and Scroll Mapping.

This section is crucial — it defines exactly how JetBrains-style curved connectors work between left and right panes, how they are drawn, how they synchronize with scroll, and how overlapping or large blocks are handled.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 4 of N)

1. Purpose of Connector Lines
   •	Connector lines serve as visual bridges between corresponding diff blocks in left and right panes.
   •	They communicate alignment, displacement, and magnitude of change.
   •	Users rely on connectors to understand whether an added block on one side maps to a deleted block on the other, or if a block has simply shifted position.

⸻

2. Connector Column
   •	A narrow vertical strip (between 40–80 px) separating left and right panes.
   •	Background is neutral (light gray in light theme, dark gray in dark theme).
   •	All connectors are drawn inside this column, without overlapping into text areas.
   •	When no diff blocks are visible in viewport, the connector column is empty.

⸻

3. Connector Line Shapes
   •	Connector lines are curved Bezier splines, not straight lines.
   •	General shape:
   •	Start anchor → left block edge.
   •	End anchor → right block edge.
   •	Curve bends outward, occupying middle space of connector column.
   •	Formula:
   •	Use cubic Bezier with two control points biased toward column center.
   •	Curve must feel smooth, fluid, and symmetric, never jagged.

⸻

4. Color Coding
   •	Connector inherits the diff type color:
   •	Additions → green.
   •	Deletions → red.
   •	Modifications → yellow/orange.
   •	Color is semi-transparent (30–40%) to avoid visual clutter.
   •	On hover: opacity increases (70–90%), making the connector stand out.
   •	Multiple overlapping connectors should retain individual color identity.

⸻

5. Connector Thickness & Style
   •	Default thickness: 2 px.
   •	On hover: thickness animates to 3–4 px.
   •	Line ends are rounded (no sharp edges).
   •	Animations use ease-in-out cubic timing for smooth transitions.

⸻

6. Multi-Line Block Connectors
   •	For blocks spanning multiple lines:
   •	Connector is not one line per line.
   •	Instead, draw a polygonal filled region connecting the two blocks.
   •	Top edge aligns top line, bottom edge aligns bottom line.
   •	Interior region is lightly shaded with the diff color (transparent).
   •	Example: if lines 20–25 on left map to lines 30–40 on right:
   •	Draw a trapezoid or curved quad linking those vertical spans.
   •	Fill with 15–20% opacity color.
   •	Edges of region are curved to follow column shape.

⸻

7. Overlap & Layering
   •	If multiple blocks overlap in vertical space:
   •	Smaller (shorter height) connectors draw above larger ones.
   •	Hovered connector always renders on topmost layer.
   •	Connectors never clip text.
   •	Connector blending uses alpha compositing, so overlapped regions form a natural gradient.

⸻

8. Scroll Synchronization with Connectors
   •	Connectors are tied to anchor blocks (see Part 1).
   •	When scrolling, connectors:
   •	Move proportionally with corresponding text blocks.
   •	Must remain visually aligned to correct line numbers.
   •	If one side has insertions/deletions, connector adjusts length dynamically:
   •	Example: deletion on left (shorter span) maps to addition on right (longer span).
   •	Connector polygon stretches to match.
   •	Connectors off-screen are not drawn.

⸻

9. Hover & Interaction Rules
   •	Hover over connector:
   •	Highlights both linked blocks across panes.
   •	Expands connector opacity + thickness.
   •	Displays tooltip with block metadata (line numbers, diff type, # of tokens changed).
   •	Click connector:
   •	Scrolls both panes to center the linked block in viewport.
   •	Optionally opens inline block diff preview (mini popup).

⸻

10. Edge Cases
    •	Very large blocks (100+ lines):
    •	Connector polygon fills column but fades opacity (10%) to avoid overwhelming.
    •	Tooltip must indicate “Large block (120 lines)”.
    •	Tiny inline changes (single-line modification):
    •	Connector remains a thin line, not a filled region.
    •	Still colored according to diff type.
    •	Unmatched orphan lines (additions/deletions with no counterpart):
    •	Connector does not render.
    •	Instead, block connects to empty region marker (dashed outline).

⸻

11. Rendering Performance
    •	Connectors are GPU-accelerated vector paths.
    •	Use WebGL, Canvas2D, or Skia for smooth rendering.
    •	Only connectors within ±1 viewport height are drawn.
    •	Re-render is throttled (max 60 fps).

Perfect — continuing with Part 5. This one goes deep: mathematics, data structures, interpolation schemes, stability constraints, and practical pseudocode for mapping lines between left and right panes while keeping scrolling smooth and visually coherent. Treat this part as the authoritative reference an engineering team would implement verbatim.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 5 of N)

Topic: Scroll Mapping Algorithms & Anchor Synchronization — Exact Math, Data Structures, and Robust Edge Handling

Scope: Provide a mathematically precise mapping F: yLeft_doc → yRight_doc and its inverse G, algorithms to build and maintain anchor maps, numerical stability constraints, interpolation choices (piecewise-linear vs. Hermite vs. monotone cubic), smoothing, temporal filtering, and complete pseudocode for the runtime scroll controller and update flows.

⸻

0. Goals & Requirements (restated succinctly)
    1.	Map vertical positions (document-space Y) from one pane to the other so corresponding diff blocks stay visually aligned.
    2.	Guarantee continuity of mapping (no large jumps), preferably C¹ continuity (continuous first derivative) to avoid kinks during scrolling.
    3.	Provide stable behavior under:
          •	large insertions/deletions,
          •	collapsed contexts,
          •	dynamic edits (local incremental diffs),
          •	viewport resizes,
          •	varying line heights (but same per-pane lh).
    4.	Support efficient query: mapping a value must be O(log n) or better, where n = number of anchors.
    5.	Allow smooth follower motion with no micro-jitter; support desync, hysteresis, and user override.

⸻

1. Basic Notation & Definitions
   •	lh — line height in CSS device-independent pixels (same on both panes).
   •	L — number of logical (unwrapped) lines in the left document.
   •	R — number of logical lines in the right document.
   •	yLeft_doc — document-space vertical coordinate on left (0 at top).
   •	yRight_doc — document-space vertical coordinate on right.
   •	lineToY_left(lineIndex) = (lineIndex - 0.5) * lh gives center Y of lineIndex (1-based).
   •	Block — diff block linking left range and right range (consult Part 1/2 definitions).
   •	Anchor_i = (x_i, X_i) where:
   •	x_i = yLeft_doc center of block i,
   •	X_i = yRight_doc center of block i,
   •	w_i = weight (optional).
   •	Anchors[] sorted ascending by x_i.

⸻

2. Anchor Generation — Deterministic Procedure

Purpose: Build a compact, stable set of anchors which represent the essential mapping between documents.

Algorithm (deterministic, stable):
1.	Compute diff blocks using robust Myers’ diff (or language-aware LCS) at line granularity → blocks[].
2.	For each block:
•	Compute left start/end line: L0, L1. Compute right start/end: R0, R1.
•	Compute centers in doc-space:

yL0 = lineToY_left(L0)
yL1 = lineToY_left(L1)
cyL = (yL0 + yL1) / 2
yR0 = lineToY_right(R0)
yR1 = lineToY_right(R1)
cyR = (yR0 + yR1) / 2


	•	Primary anchor: (cyL, cyR, weight = blockHeight)
	•	If max(yL1 - yL0, yR1 - yR0) > AnchorTallThreshold (e.g., 12 * lh), add auxiliary anchors at quartiles:

for q in [0.25, 0.5, 0.75]:
ax = yL0 + q*(yL1 - yL0)
AX = yR0 + q*(yR1 - yR0)
add Anchor(ax, AX, weight = blockWeight * 0.6)


	3.	After all primary+aux anchors collected, sort by x_i ascending.
	4.	Merge anchors that are closer than ε_merge in x (e.g., ε_merge = 2 * lh) — merge by weighted average of X and x.
	5.	Optionally inject two sentinel anchors:
	•	Anchor at top: x_0 = 0, X_0 = 0.
	•	Anchor at bottom: x_k = leftDocHeight, X_k = rightDocHeight.
These sentinels ensure mapping is defined across full domain.

Properties: anchor count m is O(number of hunks + aux anchors). For typical diffs m ≪ number of lines.

⸻

3. Piecewise Mapping Options & Tradeoffs

We present three mapping interpolation choices with pros/cons and exact formulas.

3.1 Option A — Piecewise Linear (PL)

Formula (for segment i between anchors Ai and Ai+1):

t = (y - x_i) / (x_{i+1} - x_i)        // normalized in [0,1]
F(y) = (1 - t) * X_i + t * X_{i+1}

Pros: simple, fast, stable, easy to invert.
Cons: derivative discontinuities at anchor joints → visible kinks in some cases.

3.2 Option B — Cubic Hermite / Monotone Cubic Interpolation (preferred)

Objective: C¹ continuity with monotonicity preservation to avoid overshoot.

Data per segment i:
•	x_i, x_{i+1} (left Y)
•	X_i, X_{i+1} (right Y)
•	m_i_left, m_i_right tangents (derivative dX/dx at segment endpoints)

Tangents computation (Catmull-Rom-style but monotone-clamped):
1.	Let Δx_i = x_{i+1} - x_i
2.	Let ΔX_i = X_{i+1} - X_i
3.	Compute slope s_i = ΔX_i / Δx_i (handle Δx_i ≈ 0 with epsilon)
4.	Preliminary tangents:

m_i = (s_{i-1} + s_i) / 2

with boundary handling s_{-1} = s_0, s_{n} = s_{n-1}.

	5.	Monotone clamp (Fritsch–Carlson method):
For each endpoint tangent m_i ensure it doesn’t introduce overshoot:

if s_i == 0:
m_i = 0
else:
α = m_i / s_i
if α < 0: m_i = 0   // sign mismatch -> flatten
else if α > 3: m_i = 3 * s_i  // cap for stability

Use symmetric logic for both endpoints if needed. Alternative: use monotone cubic interpolation algorithm (Fritsch–Butland or Fritsch–Carlson) to compute m_i robustly.

Hermite basis functions:

h00(t)=  2 t^3 - 3 t^2 + 1
h10(t)=    t^3 - 2 t^2 + t
h01(t)= -2 t^3 + 3 t^2
h11(t)=    t^3 -   t^2

Mapping:

Δx = x_{i+1} - x_i
t = (y - x_i) / Δx
F(y) = h00(t)*X_i + h10(t)*(m_i_left * Δx) + h01(t)*X_{i+1} + h11(t)*(m_i_right * Δx)

Pros: smooth, visually pleasant, avoids kinks.
Cons: more complex; need careful tangent clamping to preserve monotonicity.

3.3 Option C — Spline with Localized Flattening + Microanchors
•	For very tall or irregular blocks, add microanchors (sub-line granularity) and fall back to piecewise linear on those micro segments to avoid overshoot.
•	This is hybrid and used when PL or Hermite both produce unsatisfactory visual alignment.

Recommendation: Use Option B (Hermite with monotone clamping) as default, with fallback to Option A in degenerate or performance-sensitive contexts.

⸻

4. Inverse Mapping G (yRight -> yLeft)

Requirement: Allow follower updates when right pane is master and map back to left. Two approaches:
1.	Build reverse anchor set: anchors_rev sorted by X_i with pairs (X_i, x_i). Build Hermite segments in reverse domain equivalently and evaluate G(Y) with same interpolation scheme.
2.	Numerical inversion: For a given Y, find segment index i such that X_i ≤ Y < X_{i+1} then evaluate cubic polynomial F(y) (which is monotonic in segment). Solve cubic or use Newton iteration starting from linear inverse y0 = x_i + (Y - X_i) * (Δx/ΔX). Because we want O(log n) mapping, building reverse segments (option 1) is simpler and symmetric.

Recommendation: build both forward (segsF) and reverse (segsG) segment tables upon anchor rebuild. Keep them in sync.

⸻

5. Numerical Stability & Clamping
   •	Use double-precision floats for all internal mapping math.
   •	Define ε = 1e-6 * lh to avoid division by zero.
   •	Clamp slopes to avoid extreme tangents:
   •	|dX/dx| <= SlopeMax where SlopeMax = maxSlopeFactor * (rightDocHeight / leftDocHeight) — pick safe maxSlopeFactor, e.g., 10.
   •	Avoid tangents that introduce local extrema when anchors are monotone:
   •	If X_{i+1} - X_i and x_{i+1} - x_i have same sign, tangent must preserve sign. Otherwise set tangent to 0.

⸻

6. Continuous-Time Smoothing of Mapping Changes

When anchors are rebuilt (diff recalculation, edits), F changes. We must avoid jumps for the follower pane.

Strategy:
•	Keep previous mapping F_old and new mapping F_new.
•	For any follower position mapping event, compute target yF_new = F_new(yM) and yF_old = F_old(yM). If |yF_new - yF_old| > JumpThreshold (e.g., 3 * lh), animate follower over AnimDuration (120–200 ms) from yF_old to yF_new using ease-out cubic:

u(t) = 1 - (1 - t)^3 , t ∈ [0,1]
yF(t) = yF_old + u(t) * (yF_new - yF_old)


	•	Provide option snap mode for reduced-motion users.

Note: If master is actively scrolling (user continues to scroll during animation), abort animation and compute new live mapping.

⸻

7. Anchor Persistence & Re-Matching Heuristics

When content changes slightly (user edits), want anchors to persist to avoid anchor reordering that causes jumps.

Heuristic: Anchor Affinity Matching
•	Maintain stable blockId where possible from previous diff run:
•	For each new block, attempt to find an old block with high line-overlap ratio.
•	If overlap ratio ≥ AffThreshold (e.g., 0.6), reuse old anchor coordinates (with small correction) and preserve anchor ID.
•	For entirely new/removed blocks, insert/remove anchors accordingly.

This reduces churn in anchor set and mapping differences across minor edits.

⸻

8. Handling Collapsed Contexts (Folded Ranges)

When unchanged regions are collapsed into a single collapsed header occupying H_collapsed pixels:
•	Two mappings need to be consistent:
•	F_unfolded — mapping in unfolded coordinate system.
•	F_folded — mapping with collapsed regions having height H_collapsed.

Implementation:
•	Maintain a transformation function T_fold(y_unfolded) → y_folded that maps unfolded doc Y to folded doc Y (piecewise linear: collapsed ranges map to constant size).
•	Build anchors in unfolded doc-space then map anchor positions via T_fold to folded doc-space for rendering and follower mapping.
•	When expanding/collapsing:
•	Recompute T_fold, map anchors to folded coords, animate mapping changes as in Section 6.

Key point: The mapping F used at runtime always uses the visible coordinates (folded or unfolded) to avoid mismatch between rendered positions and computed anchors.

⸻

9. Algorithmic Complexity & Data Structures
   •	anchors[] stored as flat array sorted by x_i. Lookup via binary search O(log m).
   •	segments[] derived from anchors, each stores:

{
L0, L1, R0, R1, Δx, ΔX, slope, mL, mR
}


	•	For queries:
	•	mapLeftToRight(y): binary search segment index → evaluate cubic Hermite → O(log m).
	•	mapRightToLeft(y): similar using reverse segments.
	•	For range queries (e.g., drawing connectors visible in viewport):
	•	Maintain interval index keyed by x_i (e.g., b-tree or segment tree) to query anchors overlapping viewport range; typical implementation: binary search first anchor index and linear scan forward until anchor x > viewportBottom + buffer — cost proportional to number of visible anchors (usually small).
	•	Anchor rebuild:
	•	Diff algorithm O(N + D^2) worst-case for Myers’ algorithm; anchor computation O(h) where h = hunks. Anchor rebuild performed off-main thread. UI receives immutable anchors and segments snapshots.

⸻

10. Complete Pseudocode

10.1 BuildAnchors (full)

function BuildAnchors(leftLines, rightLines, lh):
blocks = computeLineDiffBlocks(leftLines, rightLines)  // returns list of blocks with left & right ranges
anchors = []

// sentinel top
anchors.push({x: 0.0, X: 0.0, w: 1.0})

for block in blocks:
L0 = block.left.startLine
L1 = block.left.endLine
R0 = block.right.startLine
R1 = block.right.endLine

    yL0 = lineToY_left(L0)
    yL1 = lineToY_left(L1)
    yR0 = lineToY_right(R0)
    yR1 = lineToY_right(R1)

    cyL = (yL0 + yL1) * 0.5
    cyR = (yR0 + yR1) * 0.5
    anchors.push({ x: cyL, X: cyR, w: (yL1 - yL0) + (yR1 - yR0) })

    tall = max(yL1 - yL0, yR1 - yR0)
    if tall > 12 * lh:
      for q in [0.25, 0.5, 0.75]:
        ax = yL0 + q * (yL1 - yL0)
        AX = yR0 + q * (yR1 - yR0)
        anchors.push({ x: ax, X: AX, w: anchors[-1].w * 0.6 })

// sentinel bottom
leftHeight = lineToY_left(leftLines.length + 0.5)  // total doc height
rightHeight = lineToY_right(rightLines.length + 0.5)
anchors.push({ x: leftHeight, X: rightHeight, w: 1.0 })

// sort & merge small gaps
anchors.sortBy(a => a.x)
anchors = mergeNearbyAnchors(anchors, threshold = 2 * lh)

return anchors

10.2 BuildSegmentsFromAnchors (Hermite prep)

function BuildSegments(anchors):
n = anchors.length
// compute slopes s[i] for each interval
slopes = []
for i in 0 .. n-2:
Δx = anchors[i+1].x - anchors[i].x
ΔX = anchors[i+1].X - anchors[i].X
slopes.push( ΔX / max(Δx, ε) )

// compute tangents using Catmull-Rom style then apply monotone clamp
tangents = array of length n
for i in 0 .. n-1:
if i == 0:
tangents[i] = slopes[0]
else if i == n-1:
tangents[i] = slopes[n-2]
else:
tangents[i] = 0.5 * (slopes[i-1] + slopes[i])

// monotone clamp (Fritsch-Carlson)
for i in 0 .. n-2:
s = slopes[i]
if abs(s) < ε:
tangents[i] = 0
tangents[i+1] = 0
else:
α = tangents[i] / s
β = tangents[i+1] / s
// ensure no overshoot
if α < 0: tangents[i] = 0
if β < 0: tangents[i+1] = 0
// cap extreme multiples
tangents[i] = clamp(tangents[i], -3*abs(s), 3*abs(s))
tangents[i+1] = clamp(tangents[i+1], -3*abs(s), 3*abs(s))

// build segments
segments = []
for i in 0 .. n-2:
seg = {
L0: anchors[i].x, L1: anchors[i+1].x,
R0: anchors[i].X, R1: anchors[i+1].X,
Δx: anchors[i+1].x - anchors[i].x,
ΔX: anchors[i+1].X - anchors[i].X,
mL: tangents[i], mR: tangents[i+1],
slope: slopes[i]
}
segments.push(seg)
return segments

10.3 Evaluate F (Hermite)

function mapLeftToRight(y, segments):
// binary search for segment index i where L0 <= y < L1
i = binarySearch(segments, y, key=seg => seg.L0)
if i == -1:
// handle extrapolate
if y < segments[0].L0: return clampToRange(y, segments[0]) // linear extrapolate
else return clampToRange(y, segments[-1])

s = segments[i]
t = (y - s.L0) / max(s.Δx, ε)
h00 = 2*t*t*t - 3*t*t + 1
h10 = t*t*t - 2*t*t + t
h01 = -2*t*t*t + 3*t*t
h11 = t*t*t - t*t
return h00*s.R0 + h10*(s.mL * s.Δx) + h01*s.R1 + h11*(s.mR * s.Δx)

10.4 Scroll Sync Loop (integration)

state = {
master: LEFT or RIGHT,
lastMasterTime: 0,
hysteresisMs: 500,
segmentsF: buildSegments(anchors),
segmentsG: buildSegments(reverseAnchors) // reversed
}

onScroll(pane, scrollTop_doc):
now = clock.now()
if pane != state.master and (now - state.lastMasterTime) > state.hysteresisMs:
state.master = pane

if state.master == LEFT:
yCenterLeft = scrollTop_doc + viewportHeight / 2
yCenterRightTarget = mapLeftToRight(yCenterLeft, state.segmentsF)
// convert to top coordinate
rightTopTarget = yCenterRightTarget - viewportHeight / 2
followerEaseTo(RIGHT, rightTopTarget)
else:
yCenterRight = scrollTop_doc + viewportHeight / 2
yCenterLeftTarget = mapRightToLeft(yCenterRight, state.segmentsG)
leftTopTarget = yCenterLeftTarget - viewportHeight / 2
followerEaseTo(LEFT, leftTopTarget)

state.lastMasterTime = now

10.5 followerEaseTo (smoothing)

function followerEaseTo(pane, targetTop):
yCurr = pane.scrollTop_doc
distance = targetTop - yCurr
// if distance small, snap
if abs(distance) < 0.5 * lh:
pane.setScrollTop(targetTop)
return
// dynamic α based on dt and user-config
α = 0.35  // tuned parameter
yNext = yCurr + α * distance
pane.setScrollTop(yNext)


⸻

11. Edge Cases & Robust Handling

11.1 Large Zero-Length Mapping (Deletion)
•	A left block mapping to zero height on right yields ΔX ≈ 0.
•	This makes slope ≈ 0 for that segment; tangents near zero will be clamped.
•	Ensure Δx is non-zero (anchors separated by at least ε). If two anchors collapse to the same x, merge them.

11.2 Highly Skewed Heights (Huge Insertions)
•	Cap local slope s = ΔX/Δx to SlopeMax to avoid extreme stretching.
•	For segments where cap is applied, insert intermediate anchor(s) by splitting the large block proportionally into N sub-anchors, where N = ceil(abs(s)/SlopeMax). This reduces slope per segment.

11.3 Repeated Rapid Updates (throttling)
•	If anchors rebuild very frequently (typing), throttle anchor rebuilds and mapping updates to 30 Hz; queue additional rebuilds and apply only latest snapshot.

11.4 Fractional Line Heights & Wrap
•	If soft wrap enabled, use visual-line units where each wrapped visual line contributes lh. Precompute mapping from logical line -> visual-lines and compute anchors in visual-lines domain.

11.5 Device Pixel Rounding
•	Keep F outputs as floats. When setting scrollTop, allow fractional pixels if platform supports subpixel scroll; otherwise round to nearest device pixel but smooth with easing.

⸻

12. Testing, Metrics, & Acceptance

12.1 Quantitative Tests
•	For n = 10k lines, random diff with 50 hunks, measure:
•	mapLeftToRight query latency ≤ 50 µs average.
•	Anchor rebuild latency (off-main-thread) ≤ 30 ms.
•	Follower convergence time ≤ 180 ms for 95% of scroll events.

12.2 Visual Tolerance Tests
•	Under slow single-line scroll, ensure vertical center alignment error ≤ 1 px on retina-scale devices; under fast scroll ≤ 2–3 px allowed.
•	Under anchor rebuild after small edits, mapping shift at arbitrary y ≤ 2 * lh (animated to new position if larger).

12.3 Stability Tests
•	Repeated minor edits (small insert/delete near top) should not cause wholesale remapping of anchors far down the file — check anchor affinity matching works.

⸻

13. Implementation Notes & Optimizations
    •	Precompute and cache:
    •	segments with Δx inverses to avoid divisions per evaluation.
    •	Bounding boxes for each segment for quick viewport culling.
    •	Use typed arrays for segments for faster binary search.
    •	Offload heavy diff calculation and anchor generation to a worker thread; send immutable snapshots to main thread.
    •	Use incremental diff recomputation around edit area to avoid full diff when possible (Myers’ diff variants support limited-range edits).
    •	For very large files, implement a sparse anchor map (only hunks + sentinels); for extremely large insertions, insert synthetic anchors to constrain slopes.

⸻

14. API Exposed for Mapping & Debugging
    •	getAnchors() → returns anchors array [ {x, X, id} ]
    •	getSegments() → returns immutable segments snapshot
    •	mapLeftToRight(y) → returns X
    •	mapRightToLeft(Y) → returns x
    •	explainMappingAt(y) → returns { segmentIndex, localT, X, Xdot } useful for debugging and telemetry
    •	rebuildAnchors(snapshot) → triggers rebuild (worker) and returns Promise resolving to new segments

⸻

15. Debugging & Visualization Tools (recommended)
    •	Debug overlay mode:
    •	Draw anchor points as small dots with label i and numeric x_i, X_i.
    •	Draw segment slope vectors at quarter points.
    •	Show live mapping error scalar: compute err(y) = |rendered_center_left(y) - mapLeftToRight(y)| across sampled Y values.
    •	Logging:
    •	When anchor rebuild occurs, log #anchors, maxSlope, avgSlope, timeTaken.
    •	If maxSlope > SlopeWarnThreshold, produce warning and optionally insert micro-anchors.

⸻

16. Final Notes (Design Rationale Summary)
    •	Use Hermite (monotone cubic) mapping for balance between smoothness and lack of overshoot.
    •	Preserve anchor stability across edits using affinity matching; prefer a small number of anchors over overly dense micro-anchors (performance tradeoff).
    •	Always operate mapping on visible coordinate system (folded/unfolded) to avoid mismatch between rendering and math.
    •	Keep follower smoothing configurable for different UX: aggressive (snappy), gentle (slow easing), none (instant snap).
    •	Provide debugging tools to detect pathological diffs (too many large insertions) and strategies to mitigate (split anchors, cap slopes).

⸻
Nice — here comes Part 6: Virtualization, Rendering Implementation, and Memory/CPU Optimizations.
This part is focused on turning the previous mapping, highlighting, and interaction specs into a high-performance implementation that feels instantaneous even on huge files. It covers rendering layers, virtualization strategies, precise DOM vs. canvas trade-offs, GPU acceleration, memory budgets, incremental updates, worker-thread architectures, and measurable acceptance criteria. Treat this as the engineering playbook your LLM agent or dev team should implement.

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 6 of N)

Topic: Virtualization, Rendering Implementation, Memory/CPU Optimizations, and Practical Engineering Patterns

⸻

0. Goals (brief)
   •	Make editor UI render only what’s necessary: visible text + small buffers.
   •	Keep connector drawing cheap and GPU-friendly.
   •	Recompute diffs and anchors off the UI thread; stream incremental results.
   •	Bound memory — support files with millions of visual lines in a stable way.
   •	Provide smooth 60fps interactions (or graceful degradation) on mid-range hardware.

⸻

1. Rendering Layer Architecture (z-order)

Design a layered rendering stack. Each layer has a single responsibility and can be turned on/off independently.

Layer order (back → front):
1.	Background & gutter base (DOM or CSS).
2.	Left code layer — virtualized DOM/text or glyph cache (text area left).
3.	Right code layer — same as left.
4.	Connector canvas — single canvas (or WebGL) spanning connector column.
5.	Overlay highlights — token-level overlays (DOM spans, absolutely positioned divs), tooltips, focus rings.
6.	Interaction layer — hitboxes, invisible inputs for keyboard, touch handling.
7.	Minimap & UI chrome.

Rationale: Separating connectors into canvas/WebGL keeps heavy vector drawing off the DOM. Code text remains accessible (selectable, copyable) as DOM where possible.

⸻

2. Virtualization Strategy (Text)

2.1 Virtualization unit
•	Unit: visual line (wrapped or unwrapped). Each visual line has a fixed lh. Virtualization operates on ranges of visual lines.
•	Viewport window: visible range computed from scrollTop and viewportHeight.

2.2 Render window
•	Render range = [firstVisibleLine - pad, lastVisibleLine + pad] where pad = viewportLineCount * 0.5 (or tuned value). pad prevents blank regions during fast scroll.
•	For extremely large files, cap pad to a configurable maximum (e.g., 1000 lines).

2.3 DOM vs. Canvas text rendering tradeoffs
•	DOM (preferred on web):
•	Advantages: selectable text, accessibility (ARIA), native fonts, copy/paste, IME support.
•	Use a recycled pool of line elements (virtual rows). Keep them minimal: precomputed innerHTML for syntax-highlighting tokens.
•	Use position: absolute for each line inside a large-scrolling container. Container has full height equal to document height to allow native scroll.
•	Canvas text rendering:
•	Advantages: extreme performance for thousands of lines; cheaper to render highlights/overlays with the text embedded on the same surface.
•	Disadvantages: not selectable, more complex IME and accessibility.
•	Hybrid approach: Use DOM for selectable text in visible window, use canvas for connectors/minimap. For very high performance or custom fonts, consider canvas with an overlay for selection emulation.

2.4 Recycling & Pooling
•	Keep a fixed-size pool of DOM nodes equal to maximum rendered lines (renderWindowSize). Reuse nodes by updating text and style when scroll changes.
•	Avoid creating/destroying nodes on each frame.

2.5 Text diff highlight rendering
•	Prefer tokenized HTML spans inside each line element for inline highlights. Keep token spans minimal (wrap only changed tokens).
•	For unchanged lines, render single text node (no spans) to reduce DOM nodes.
•	Use will-change: transform sparingly — only for animating elements.

⸻

3. Connector Rendering (Canvas / WebGL)

3.1 Technology choice
•	Canvas2D is sufficient for most cases and easier to implement; use GPU accelerated context where available.
•	WebGL / Skia for extreme cases (many connectors, large visual complexity).

3.2 Single-canvas pattern
•	Use a single canvas element sized to connector column width × viewport height.
•	On scroll, update canvas transform or redraw only visible connector segments.
•	Keep canvas device-pixel scaled (canvas.width = cssWidth * devicePixelRatio) to maintain crisp connectors.

3.3 Draw-on-demand + caching
•	Cache computed Bézier control points and path geometries per block id keyed by current Wc, lh, and anchor snapshot id.
•	On each frame:
•	Query visible anchor indices (binary search) — O(log m + k) where k = visible anchors count.
•	For each visible anchor pair, fetch cached path and draw with ctx.stroke() or ctx.fill() when needed.
•	Use requestAnimationFrame to batch draws (throttle scroll event handlers).

3.4 GPU hints
•	For repeated strokes, draw connectors into an offscreen canvas and blit onto the visible canvas when static. Update offscreen only when anchors or viewport changes.
•	On WebGL, upload path as triangle strips for filled ribbons and render with fragment shader to apply softness/gradient.

3.5 Hit testing
•	Maintain per-connector AABB. Broad-phase using AABB test. For narrow-phase, approximate distance to curve with precomputed polyline samples (e.g., 8–12 segments) — accurate and fast.

⸻

4. Incremental Diffing & Anchors (Worker-based)

4.1 Pipeline
•	Main thread: UI rendering, scrolling, input handling.
•	Worker thread: diff calculation, tokenization, anchor generation, heavy language parsing.
•	Message contract: Worker returns immutable snapshots:

{
versionId,
hunks: [...],
anchors: [...],
tokenDiffs: { lineIndex: [ {start, end, type} ] },
stats: { timeMs, linesProcessed }
}


	•	Main thread applies snapshot atomically; any in-progress rendering uses the last snapshot until new one arrives.

4.2 Incremental updates
•	For edits localized in a small region, compute diff only around that zone (Myers’ diff supports limited-range recompute). Worker returns a patch-like snapshot with changed hunks only — apply incremental anchor merges to maintain stability.

4.3 Throttling & Debounce
•	Debounce heavy recompute after continuous typing (e.g., 200–400 ms idle before full recompute).
•	For live preview, run a fast lightweight diff algorithm (line-level) immediately, then refine with token-level diff when available.

⸻

5. Memory Budgets & Data Structures

5.1 Budgets (recommendations)
•	Reserve ~100 MB for UI data structures on desktop; for web target mid-range machines, aim < 60 MB.
•	Capped caches:
•	Connector path cache: limit to visible + N prefetch (N=200). Evict LRU.
•	Token diffs cache: per-line limited to changed lines only.
•	Recycled DOM nodes: keep pool size = renderWindowSize (e.g., viewport lines + pad). For 1200 px viewport and 18px lh, renderWindowSize ≈ 100 lines -> pool 150 nodes.

5.2 Compact representations
•	Store anchors and segments in typed arrays (Float64Array or Float32Array) for fast binary search and lower GC churn.
•	Represent hunks with small structs:

struct Hunk {
id: int32,
leftStart: int32, leftEnd: int32,
rightStart: int32, rightEnd: int32,
type: int8, // enum
tokenDiffIndex: int32 // index into tokenDiff pool or -1
}


	•	Token diffs: pooled flat arrays of [lineIndex, start, end, type].

⸻

6. Rendering Optimizations & Heuristics

6.1 Avoid layout thrash
•	Batch DOM reads/writes. Always read scrollTop once per frame; compute positions in JS then write to DOM.
•	Use transforms (translateY) for moving pools rather than changing top/height where applicable.

6.2 Paint cost reduction
•	Minimize CSS box-shadow, expensive filters, and large paint area elements.
•	Use contain: layout paint on the code pane container to isolate paints.

6.3 Adaptive LOD
•	If frame budget missed (frame time > 16ms), progressively degrade:
1.	Skip token-level inline highlights (keep line-level).
2.	Render connectors as simple lines instead of filled ribbons.
3.	Increase virtualization pad to keep node churn low.
•	Monitor FPS and CPU usage; auto-toggle LOD for smooth UX.

6.4 Prefetching & Predictive rendering
•	Predict scroll direction from velocity; pre-render additional lines ahead in that direction.
•	For connectors, precompute paths for anchors slightly outside viewport based on scroll velocity.

⸻

7. Accessibility & Selection Considerations in Virtualized DOM
   •	Ensure selection across recycled nodes maps to underlying document coordinates. Implement a mapping layer: DOM node index → document line index.
   •	When user selects text, export selection as actual text using lineIndex mapping rather than DOM content that’s being re-used.
   •	For screen readers, keep aria-live updates minimal; announce only meaningful changes (hunk applied, conflict detected).

⸻

8. Threading, Concurrency & Consistency

8.1 Immutable snapshots
•	Worker returns immutable snapshots with version id. UI only switches to snapshot if it matches expected sequence to avoid stale updates.
•	Use compare-and-swap semantics:

if(snapshot.version > current.version) {
current = snapshot;
applySnapshot(current);
}



8.2 Cancellation
•	Send cancel tokens for previous heavy tasks when new requests come in (e.g., new diffs while typing).

8.3 Main-thread safety
•	Keep heavy computation off main thread.
•	For small immediate UI changes (scrolling), adjust mapping and visuals using last snapshot; do not wait for worker.

⸻

9. Performance Telemetry & Tuning

Collect metrics to drive heuristics:
•	anchorRebuildTimeMs
•	diffComputeTimeMs
•	visibleHunksCount
•	connectorDrawTimeMs (per frame)
•	DOMUpdatesPerSecond
•	frameRate (30/60)
Log anomalies and provide telemetry toggles for opt-in.

⸻

10. Testing & Benchmarks

10.1 Microbenchmarks
•	mapLeftToRight executed 1M times; average latency < 100µs on target hardware.
•	Connector draw: drawing 200 visible connectors should complete < 4ms on canvas.

10.2 Scenario tests
•	100k-line file with 200 hunks (random) — interactive scroll should maintain > 45fps on mid-range machine.
•	50 MB file with 500 hunks — initial diff offloaded to worker, worker time < 800 ms; UI remains responsive.

10.3 Regression tests
•	Memory leak tests: repeated open/close diff views should not increase retained memory beyond small steady-state.
•	DOM node churn tests: ensure recycled pool keeps node creation low.

⸻

11. Acceptance Criteria (visible & measurable)
    1.	Scrolling the master pane and seeing follower converge: follower center aligns within ±2 px within ≤ 200 ms in 95% of cases.
    2.	Startup: loading a 50 MB file (text) and computing line-level diff should not block UI; worker returns initial snapshot in ≤ 1.2s.
    3.	Peak memory usage for 100k-line file < 300MB on desktop; for web medium machines aim < 150MB.
    4.	CPU budget: steady-state UI thread usage < 15% on a 4-core CPU during typical interaction.
    5.	Connector rendering: 200 visible connectors draw under 8 ms total on a typical machine.

⸻

12. Implementation Checklist (deliverable items)
    •	Virtualized DOM line pool implementation (left & right).
    •	Worker diff & anchor generator with incremental updates.
    •	Typed-array segments + binary search mapping functions mapLeftToRight & mapRightToLeft.
    •	Single-canvas connector renderer with caching & LOD.
    •	Token-level inline highlight overlays (sparse).
    •	Hit-testing layer for connectors (AABB + polyline sample).
    •	Throttled requestAnimationFrame draw loop for connectors & overlays.
    •	Adaptive LOD fallback and FPS monitoring.
    •	Accessibility bindings and selection mapping layer.
    •	Telemetry & debug overlay tools.

⸻

13. Example Web Implementation Notes (practical tips)
    •	Use virtual-scroller technique: a container with full height, but only small set of positioned lines in DOM.
    •	Use CSS transform: translateY() to position pooled line elements with hardware acceleration.
    •	For worker comms, use postMessage with Transferable objects for large buffers (ArrayBuffers).
    •	Use OffscreenCanvas for canvas rendering if supported (moves rendering to worker / dedicated thread).
    •	Avoid heavy CSS fonts/weights per token; precompute classnames like .tok-add, .tok-del rather than inline styles.

⸻

14. Closing rationale & next steps

Perfect — let’s move into Part 7: Theming, Colors, and Iconography.
This part defines the exact visual design system: how colors are assigned to diffs, how themes adapt between light/dark/high-contrast, how icons are standardized, and how CSS tokens/variables make the system extensible. This is the bridge between the raw rendering engine and the JetBrains-like polished UX feel.

⸻

📑 Prompt Spec — JetBrains Git Diff Viewer (Part 7 of N)

Topic: Theming, Colors, Typography, Icons, and Visual Consistency

⸻

1. Theming System Overview
   •	Diff viewer supports multiple themes:
   •	Light theme
   •	Dark theme
   •	High contrast mode (accessibility)
   •	Custom themes (user-defined via CSS variables or config API)
   •	Theming is token-based: every visual property maps to a CSS variable or programmatic token.
   •	Tokens cascade from base → semantic → component-level.

⸻

2. Base Tokens

2.1 Color primitives

Define raw palette values:
•	--color-green-500 = #4CAF50
•	--color-red-500 = #F44336
•	--color-yellow-500 = #FFC107
•	--color-blue-500 = #2196F3
•	Neutral grays: --gray-50, --gray-100, …, --gray-900
•	Accessible shades: ensure WCAG 2.1 contrast ratios

2.2 Typography primitives
•	Font family: --font-mono = "JetBrains Mono", Menlo, Consolas, monospace
•	Font size: --font-size-sm = 12px, --font-size-md = 14px, --font-size-lg = 16px
•	Line height: --line-height-code = 1.4em

2.3 Border radius & spacing
•	Radius tokens: --radius-xs = 2px, --radius-sm = 4px, --radius-md = 6px
•	Spacing tokens: --space-xs = 4px, --space-sm = 8px, --space-md = 16px

⸻

3. Semantic Tokens

3.1 Diff-specific
•	Addition background: --diff-add-bg = var(--color-green-500, #4CAF50, 0.15 alpha)
•	Addition text: --diff-add-fg = var(--color-green-500)
•	Deletion background: --diff-del-bg = var(--color-red-500, 0.15 alpha)
•	Deletion text: --diff-del-fg = var(--color-red-500)
•	Modification background: --diff-mod-bg = var(--color-yellow-500, 0.15 alpha)
•	Modification text: --diff-mod-fg = var(--color-yellow-500)
•	Connector column bg: --diff-connector-bg = var(--gray-200) (light), --gray-800 (dark)

3.2 Code tokens
•	Default text color: --code-fg = var(--gray-900) (light) / --gray-50 (dark)
•	Comment color: --code-comment-fg = var(--gray-500)
•	Keyword color: --code-keyword-fg = var(--color-blue-500)
•	String color: --code-string-fg = var(--color-green-500)

3.3 UI tokens
•	Toolbar background: --ui-toolbar-bg = var(--gray-100) / --gray-900
•	Toolbar icon color: --ui-icon-fg = var(--gray-600) / --gray-300
•	Tooltip background: --ui-tooltip-bg = var(--gray-900, 0.9 alpha)

⸻

4. Light Theme Definition

:root {
--code-fg: #1E1E1E;
--diff-add-bg: rgba(76, 175, 80, 0.15);
--diff-add-fg: #2E7D32;
--diff-del-bg: rgba(244, 67, 54, 0.15);
--diff-del-fg: #C62828;
--diff-mod-bg: rgba(255, 193, 7, 0.15);
--diff-mod-fg: #FF8F00;
--diff-connector-bg: #E0E0E0;
}


⸻

5. Dark Theme Definition

:root[data-theme="dark"] {
--code-fg: #EDEDED;
--diff-add-bg: rgba(76, 175, 80, 0.25);
--diff-add-fg: #81C784;
--diff-del-bg: rgba(244, 67, 54, 0.25);
--diff-del-fg: #EF9A9A;
--diff-mod-bg: rgba(255, 193, 7, 0.25);
--diff-mod-fg: #FFD54F;
--diff-connector-bg: #424242;
}


⸻

6. High Contrast Theme
   •	Uses bold outlines, no transparency.
   •	Additions: solid bright green background, black text.
   •	Deletions: solid bright red background, white text.
   •	Connectors: thick 3px lines with dashed pattern for clarity.
   •	Token colors chosen for maximum WCAG AAA compliance.

⸻

7. Typography Rules
   •	Code font = JetBrains Mono at 14px default.
   •	Adjust per user setting:
   •	Zoom scaling factor: user can bump by ±25%.
   •	Line height = 1.4em, ensures vertical rhythm matches connector mapping.

⸻

8. Icons & Symbolography
   •	Icon system is monotone, vector-based, designed to adapt color via currentColor.
   •	Use SVG paths with stroke="currentColor", fill="none".
   •	Icons needed:
   •	Expand/collapse block → chevron (▸/▾)
   •	Copy to clipboard → overlapping rectangles
   •	Jump to next/prev diff → up/down arrows
   •	Merge accept left/right → arrow pointing left/right into block
   •	Conflict marker → exclamation inside triangle
   •	All icons sized to 16×16 grid, aligned pixel-perfect.

⸻

9. Connector Styling per Theme
   •	Light theme:
   •	Connectors use semi-transparent stroke (0.4 opacity).
   •	On hover → 0.8 opacity.
   •	Dark theme:
   •	Connectors brighter: 0.6 opacity baseline.
   •	Hover → full color (1.0 opacity).
   •	High contrast:
   •	Connectors are bold, 3px width, dashed.

⸻

10. Tooltip Styling
    •	Default tooltip style:
    •	Background: --ui-tooltip-bg
    •	Border radius: --radius-sm
    •	Text: white or black depending on background
    •	Padding: --space-sm
    •	Tooltip content:
    •	Line numbers → monospace bold
    •	Diff type → color-coded label
    •	Example:

Lines 45–48 → Lines 52–55
Type: Modified



⸻

11. CSS Variable Namespaces

All tokens prefixed with --diffviewer- for isolation:
•	--diffviewer-add-bg
•	--diffviewer-del-fg
•	--diffviewer-connector-bg

Developers extending theme can override at root or in [data-theme] context.

⸻

12. Transitions & Animations
    •	Color transitions are smooth (transition: color 0.2s ease, background 0.2s ease).
    •	Hover on connector → animate stroke width and opacity.
    •	Expanding/collapsing unchanged blocks → slide-down animation, eased cubic timing.

⸻

13. Accessibility & Colorblind Support
    •	Provide alternative patterns (hatching, dotted backgrounds) for diff types:
    •	Additions → diagonal green hatching
    •	Deletions → crosshatch red
    •	Modifications → dotted amber
    •	Allow users to switch between color mode and pattern mode.
    •	Ensure ARIA labels describe:
    •	“Block addition: 3 lines added.”
    •	“Connector: maps lines 20–25 to lines 30–35.”

⸻

14. Acceptance Criteria
    1.	Theme switch (light ↔ dark ↔ high-contrast) must apply instantly, no layout shift.
    2.	All connector colors must be distinguishable in WCAG AAA tests.
    3.	All icons must remain pixel-aligned and visible at 125% zoom.
    4.	CSS token overrides must propagate across entire viewer without requiring rebuild.

