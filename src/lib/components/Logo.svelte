<script lang="ts">
  let { size = 24 }: { size?: number } = $props();
  // id unique par instance pour éviter les collisions de gradient.
  const uid = $props.id();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden="true"
  class="logo"
>
  <defs>
    <linearGradient id="lg-bg-{uid}" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#121821" />
      <stop offset="1" stop-color="#0B0F14" />
    </linearGradient>
  </defs>
  <rect x="0.5" y="0.5" width="23" height="23" rx="6" fill="url(#lg-bg-{uid})" stroke="#1E2730" />
  <!-- barres ascendantes discrètes -->
  <rect x="8.6" y="13.4" width="2.3" height="5.2" rx="1" fill="#16BFA6" opacity="0.3" />
  <rect x="12.1" y="11" width="2.3" height="7.6" rx="1" fill="#16BFA6" opacity="0.3" />
  <rect x="15.6" y="8.4" width="2.3" height="10.2" rx="1" fill="#16BFA6" opacity="0.3" />
  <!-- courbe signature -->
  <path
    d="M4.4 16.4 C 7.4 15.8, 9.4 13.6, 12 10.8 C 14 8.6, 15.6 7.6, 18.4 6.9"
    stroke="#16BFA6"
    stroke-width="2.1"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
  <circle cx="18.4" cy="6.9" r="2" fill="#2FBF71" />
  <circle cx="18.4" cy="6.9" r="2" fill="none" stroke="#0B0F14" stroke-width="0.8" />
</svg>
