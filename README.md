# Pécule

**Votre argent, au clair et au calme. Local, privé, sans compte.**

Pécule est une application de bureau **native** (Tauri + Rust + SvelteKit) de
finance personnelle **local-first**. Elle réunit trois piliers autour d'une même
« courbe » interactive :

1. **Portefeuille** — suivre ce que l'on possède *(v0.2)*
2. **Simulateurs** — projeter ce que l'on pourrait avoir *(disponible)*
3. **Abonnements** — maîtriser ses dépenses récurrentes *(v0.3)*

> ⚠️ **Avertissement** : Pécule est un outil **pédagogique** et de suivi. Il ne
> fournit **aucun conseil en investissement** et ne remplace pas un professionnel.

---

## Où en est-on ? (v0.1)

La v0.1 livre les **simulateurs** et « **la courbe** » — déjà utiles sans aucune
donnée personnelle :

- **Intérêts composés** — capital placé une fois, capitalisation au choix.
- **Épargne régulière (DCA)** — capital de départ + versements mensuels.
- **Prêt immobilier** — mensualité, assurance, coût total, tableau d'amortissement.
- **« La courbe »** — projection temporelle interactive avec curseur « et dans X
  années ? » en temps réel.
- Thème **sombre** (défaut) / clair, interface **française**, chiffres tabulaires.

Le **portefeuille** et les **abonnements** sont esquissés dans l'interface et
arriveront aux versions suivantes (voir la feuille de route).

## Vie privée d'abord

- **Local-first** : tout est stocké sur votre machine (SQLite local), jamais sur
  un serveur. Aucun compte, aucune banque connectée, aucune télémétrie, aucune pub.
- **Aucune origine réseau externe** n'est autorisée (CSP stricte) : l'application
  est inutilisable comme canal d'exfiltration.
- La **seule** entorse possible, prévue plus tard et **désactivée par défaut**, est
  le rafraîchissement de **cours publics** (prix d'un ETF, d'une action…). Même
  alors, **vos positions ne quittent jamais l'appareil** — seuls des prix publics
  entrent.

## Calculs exacts et vérifiés

Les moteurs financiers sont écrits en **Rust pur** (sans I/O, déterministes) et
**testés contre des valeurs de référence** calculées via les formules fermées
standard (`cargo test`). Le frontend appelle ces moteurs via l'IPC Tauri et reçoit
la série complète, ce qui rend le curseur de projection fluide sans appel réseau.

---

## Développement

**Prérequis** : [Rust](https://rustup.rs) (stable) et [Node.js](https://nodejs.org)
18+. Sur macOS : Xcode Command Line Tools. Voir les
[prérequis Tauri](https://v2.tauri.app/start/prerequisites/) par OS.

```bash
npm install          # dépendances frontend
npm run tauri dev    # lance l'application native (Vite + Rust)
npm run tauri build  # construit les binaires de l'OS courant

# Frontend seul (prévisualisation navigateur, moteur TS de repli) :
npm run dev

# Qualité :
npm run check                       # svelte-check (types)
cd src-tauri && cargo test          # tests des moteurs financiers
cd src-tauri && cargo clippy        # lint Rust
```

## Structure

```
pecule/
├─ src/                      # FRONTEND (SvelteKit + TS)
│  ├─ routes/                # vues (tableau de bord, simulateurs, …)
│  └─ lib/
│     ├─ components/         # Courbe, cartes, contrôles, sidebar, en-tête
│     ├─ i18n/               # traductions (FR prioritaire, EN partiel)
│     ├─ styles/             # jetons de design + styles globaux
│     ├─ ipc.ts              # pont typé vers les commandes Rust
│     └─ sim-fallback.ts     # moteur TS de repli (prévisualisation navigateur)
└─ src-tauri/                # BACKEND (Rust + config Tauri)
   └─ src/
      ├─ finance/            # moteurs purs : compound, savings, loan (+ tests)
      ├─ commands/           # commandes IPC (simulate, settings)
      └─ db.rs               # SQLite local (rusqlite)
```

## Feuille de route

| Version | Contenu |
|---------|---------|
| **v0.1** | Simulateurs (intérêts composés, épargne, prêt) + « la courbe » ✅ |
| v0.2 | Portefeuille : positions, valorisation manuelle, +/- value, répartition |
| v0.3 | Abonnements : coût annuel réel, alertes de renouvellement |
| v1.0 | Trois piliers, import/export, binaires signés (macOS/Windows/Linux) |
| v2.0 | Cours publics optionnels, enveloppes, multi-devises, base chiffrable |

## Licence

[MIT](LICENSE). Outil pédagogique — **aucun conseil financier**.
