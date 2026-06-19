<script lang="ts">
  let {
    label,
    value = $bindable(),
    min = 0,
    max = 100,
    step = 1,
    unit = "",
    slider = true,
    hint = "",
  }: {
    label: string;
    value: number;
    min?: number;
    max?: number;
    step?: number;
    unit?: string;
    slider?: boolean;
    hint?: string;
  } = $props();

  const uid = $props.id();

  // À la frappe : on plafonne au max et on neutralise les valeurs non finies,
  // sans imposer le min (pour laisser saisir librement vers le haut). Évite
  // qu'une valeur démesurée alimente les moteurs pendant la saisie.
  function clampMax() {
    if (Number.isFinite(value) && value > max) value = max;
  }
  // À la validation (blur/Entrée) : on borne dans [min, max] et on récupère un
  // champ vidé/non fini en le ramenant au min.
  function clamp() {
    if (!Number.isFinite(value)) value = min;
    else if (value < min) value = min;
    else if (value > max) value = max;
  }
</script>

<div class="field">
  <div class="top">
    <label for={uid} class="small muted">{label}</label>
    {#if hint}<span class="small muted hint">{hint}</span>{/if}
  </div>
  <div class="input-row">
    <input
      id={uid}
      class="mono num"
      type="number"
      {min}
      {max}
      {step}
      bind:value
      oninput={clampMax}
      onchange={clamp}
    />
    {#if unit}<span class="unit small muted">{unit}</span>{/if}
  </div>
  {#if slider}
    <input
      class="range"
      type="range"
      {min}
      {max}
      {step}
      bind:value
      aria-label={label}
    />
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .top {
    display: flex;
    justify-content: space-between;
    gap: var(--space-2);
  }
  .hint {
    opacity: 0.8;
  }
  .input-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    background: var(--surface-2);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
    padding: 0 var(--space-3);
  }
  .input-row:focus-within {
    border-color: var(--accent);
  }
  .num {
    flex: 1;
    width: 100%;
    border: 0;
    background: transparent;
    color: var(--ink);
    padding: 10px 0;
    font-size: var(--fs-body);
    font-weight: 500;
    outline: none;
  }
  /* masque les flèches natives, on garde l'input propre */
  .num::-webkit-outer-spin-button,
  .num::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  .num {
    -moz-appearance: textfield;
    appearance: textfield;
  }
  .unit {
    flex-shrink: 0;
  }
</style>
