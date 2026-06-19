// Parseur CSV tolérant pour l'import de positions de portefeuille.
// Accepte un séparateur `,` ou `;` (auto-détecté), un en-tête optionnel
// (colonnes reconnues par alias FR/EN), sinon l'ordre positionnel
// nom, symbole, type, quantité, PRU, cours. Décimales `.` ou `,`.
import type { NewHolding } from "./ipc";

const ALIASES: Record<keyof Omit<NewHolding, "devise">, string[]> = {
  nom: ["nom", "name", "libelle", "intitule", "designation"],
  symbole: ["symbole", "symbol", "ticker", "isin", "code", "mnemo"],
  type: ["type", "classe", "categorie", "asset", "nature"],
  quantite: ["quantite", "quantity", "qty", "nombre", "parts", "qte"],
  pru: ["pru", "cost", "prixderevient", "prixachat", "prixmoyen", "cout"],
  prixActuel: ["prixactuel", "prix", "price", "cours", "valeur", "last"],
};

function clean(s: string): string {
  return s.trim().replace(/^"(.*)"$/, "$1").trim();
}
function key(s: string): string {
  return s
    .toLowerCase()
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .replace(/[^a-z]/g, "");
}
function toNum(s: string): number {
  const n = parseFloat(clean(s).replace(/\s/g, "").replace(",", "."));
  return Number.isFinite(n) ? n : 0;
}
function normType(s: string): string {
  const t = key(s);
  if (t.includes("etf") || t.includes("tracker")) return "etf";
  if (t.includes("crypto") || t.includes("btc") || t.includes("eth")) return "crypto";
  if (t.includes("action") || t.includes("stock") || t.includes("titre")) return "action";
  if (["action", "etf", "crypto", "autre"].includes(t)) return t;
  return "action";
}

export function parseHoldingsCsv(text: string): NewHolding[] {
  const lines = text.split(/\r?\n/).map((l) => l.trim()).filter(Boolean);
  if (lines.length === 0) return [];

  const semis = (lines[0].match(/;/g) || []).length;
  const commas = (lines[0].match(/,/g) || []).length;
  const tabs = (lines[0].match(/\t/g) || []).length;
  const delim = tabs >= semis && tabs >= commas ? "\t" : semis >= commas ? ";" : ",";

  let rows = lines.map((l) => l.split(delim).map(clean));

  const firstKeys = rows[0].map(key);
  const allAliases = Object.values(ALIASES).flat();
  const isHeader = firstKeys.some((k) => allAliases.includes(k));

  let cols: Partial<Record<keyof NewHolding, number>>;
  if (isHeader) {
    cols = {};
    for (const [field, aliases] of Object.entries(ALIASES)) {
      const idx = firstKeys.findIndex((k) => aliases.includes(k));
      if (idx >= 0) cols[field as keyof NewHolding] = idx;
    }
    rows = rows.slice(1);
  } else {
    cols = { nom: 0, symbole: 1, type: 2, quantite: 3, pru: 4, prixActuel: 5 };
  }

  const at = (r: string[], f: keyof NewHolding): string => {
    const i = cols[f];
    return i != null && i < r.length ? r[i] : "";
  };

  const out: NewHolding[] = [];
  for (const r of rows) {
    const nom = at(r, "nom");
    const symbole = at(r, "symbole");
    if (!nom && !symbole) continue;
    const pru = toNum(at(r, "pru"));
    out.push({
      nom: nom || symbole,
      symbole: symbole || "",
      type: normType(at(r, "type")),
      quantite: toNum(at(r, "quantite")),
      pru,
      prixActuel: toNum(at(r, "prixActuel")) || pru,
      devise: "EUR",
    });
  }
  return out;
}
