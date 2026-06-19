// État partagé d'ouverture de la palette de commandes (⌘K), pour que l'en-tête
// et le composant CommandPalette parlent au même état.
let open = $state(false);

export function paletteIsOpen(): boolean {
  return open;
}
export function openPalette() {
  open = true;
}
export function closePalette() {
  open = false;
}
export function togglePalette() {
  open = !open;
}
