<script lang="ts">
  import type { SeriesPoint } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR0 } from "$lib/format";
  import AnimatedNumber from "./AnimatedNumber.svelte";

  let {
    series,
    cursorYear = $bindable(),
    height = 260,
    showContributed = true,
    showInterest = true,
    scrub = true,
    format = fmtEUR0,
    valueLabel = "",
  }: {
    series: SeriesPoint[];
    cursorYear?: number;
    height?: number;
    showContributed?: boolean;
    showInterest?: boolean;
    scrub?: boolean;
    format?: (n: number) => string;
    valueLabel?: string;
  } = $props();

  const uid = $props.id();
  let cw = $state(640);

  const padL = 6;
  const padR = 10;
  const padT = 18;
  const padB = 24;

  const data = $derived(
    series.length ? series : [{ year: 0, value: 0, contributed: 0, interest: 0 }],
  );
  const xMax = $derived(Math.max(data[data.length - 1].year, 1e-9));
  const yMax = $derived(Math.max(...data.map((p) => p.value), 1));

  const iw = $derived(Math.max(cw - padL - padR, 10));
  const ih = $derived(Math.max(height - padT - padB, 10));

  const X = (year: number) => padL + (year / xMax) * iw;
  const Y = (v: number) => padT + (1 - v / yMax) * ih;

  function line(key: "value" | "contributed"): string {
    return data
      .map((p, k) => `${k ? "L" : "M"}${X(p.year).toFixed(2)},${Y(p[key]).toFixed(2)}`)
      .join("");
  }
  function area(key: "value" | "contributed"): string {
    const yb = Y(0).toFixed(2);
    return `${line(key)}L${X(xMax).toFixed(2)},${yb}L${X(0).toFixed(2)},${yb}Z`;
  }

  const valueLine = $derived(line("value"));
  const valueArea = $derived(area("value"));
  const contribLine = $derived(showContributed ? line("contributed") : "");

  const grid = $derived([yMax / 2, yMax]);

  // — Curseur : on borne dans [0, xMax] et on initialise à l'horizon. —
  $effect(() => {
    const end = Math.round(xMax);
    if (cursorYear === undefined || cursorYear > xMax) cursorYear = end;
  });
  const effCursor = $derived(Math.min(Math.max(cursorYear ?? xMax, 0), xMax));

  function nearest(year: number): SeriesPoint {
    let best = data[0];
    let bd = Infinity;
    for (const p of data) {
      const d = Math.abs(p.year - year);
      if (d < bd) {
        bd = d;
        best = p;
      }
    }
    return best;
  }
  const cur = $derived(nearest(effCursor));
  const cx = $derived(X(cur.year));
  const cyVal = $derived(Y(cur.value));

  const sliderStep = $derived(xMax >= 8 ? 1 : xMax >= 2 ? 0.5 : 0.25);
  const horizonLabel = $derived(
    cur.year < 0.5 ? t("common.today") : t("common.inYears", { n: Math.round(cur.year) }),
  );
</script>

<div class="curve">
  <div class="readout">
    <div class="ry">
      <span class="small muted">{valueLabel || t("dashboard.projectedValue")}</span>
      <span class="rv display mono"><AnimatedNumber value={cur.value} {format} /></span>
      {#if showInterest}
        <span class="small muted"
          >{t("dashboard.ofWhichInterest", { v: format(cur.interest) })}</span
        >
      {/if}
    </div>
    <div class="chip mono small">{horizonLabel}</div>
  </div>

  <div class="chart" bind:clientWidth={cw} style="height:{height}px">
    <svg
      width={cw}
      {height}
      role="img"
      aria-label={`${valueLabel || t("dashboard.projectedValue")} : ${format(cur.value)}, ${horizonLabel}`}
    >
      <defs>
        <linearGradient id="grad-{uid}" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="var(--accent)" stop-opacity="0.34" />
          <stop offset="100%" stop-color="var(--accent)" stop-opacity="0" />
        </linearGradient>
      </defs>

      {#each grid as g (g)}
        <line class="grid" x1={padL} x2={cw - padR} y1={Y(g)} y2={Y(g)} />
        <text class="axis" x={padL} y={Y(g) - 5}>{format(g)}</text>
      {/each}

      <path class="area" d={valueArea} fill="url(#grad-{uid})" />
      {#if showContributed}
        <path class="line-contrib" d={contribLine} />
      {/if}
      <path class="line" d={valueLine} />

      <line class="cursor" x1={cx} x2={cx} y1={padT} y2={padT + ih} />
      <circle class="dot-halo" cx={cx} cy={cyVal} r="8" />
      <circle class="dot" cx={cx} cy={cyVal} r="4.5" />

      <text class="axis" x={padL} y={height - 6}>{t("common.today")}</text>
      <text class="axis end" x={cw - padR} y={height - 6}
        >{t("common.inYears", { n: Math.round(xMax) })}</text
      >
    </svg>
  </div>

  {#if scrub}
    <div class="scrub-row">
      <span class="small muted">{t("dashboard.projectionOver")}</span>
      <input
        class="range"
        type="range"
        min="0"
        max={xMax}
        step={sliderStep}
        value={effCursor}
        oninput={(e) => (cursorYear = +e.currentTarget.value)}
        aria-label={t("dashboard.projectionOver")}
        aria-valuetext={horizonLabel}
      />
    </div>
  {/if}
</div>

<style>
  .curve {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .readout {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }
  .ry {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .rv {
    color: var(--ink);
  }
  .chip {
    background: var(--accent-soft);
    color: var(--accent);
    padding: 4px 10px;
    border-radius: var(--radius-pill);
    font-weight: 500;
    white-space: nowrap;
  }
  .chart {
    width: 100%;
  }
  svg {
    display: block;
    overflow: visible;
  }
  .grid {
    stroke: var(--line);
    stroke-width: 1;
    stroke-dasharray: 2 5;
    opacity: 0.7;
  }
  .axis {
    fill: var(--muted);
    font-family: var(--font-mono);
    font-size: 10px;
  }
  .axis.end {
    text-anchor: end;
  }
  .line {
    fill: none;
    stroke: var(--accent);
    stroke-width: 2.5;
    stroke-linejoin: round;
    stroke-linecap: round;
  }
  .line-contrib {
    fill: none;
    stroke: var(--muted);
    stroke-width: 1.5;
    stroke-dasharray: 4 4;
    opacity: 0.7;
  }
  .cursor {
    stroke: var(--accent);
    stroke-width: 1;
    stroke-dasharray: 3 3;
    opacity: 0.6;
  }
  .dot {
    fill: var(--accent);
    stroke: var(--surface);
    stroke-width: 2.5;
  }
  .dot-halo {
    fill: var(--accent);
    opacity: 0.18;
  }
  .scrub-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }
  .scrub-row .range {
    flex: 1;
  }
</style>
