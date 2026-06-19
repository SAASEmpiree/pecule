# Pécule — Plan directeur

> « **Une infinité de fonctionnalités, mais une app qui reste simple.** »
> Document directeur **vivant**. Pécule : application de bureau native (Tauri 2 + Rust + SvelteKit), finance personnelle **local‑first**, marché FR/UE (RGPD), open source (MIT).
> État actuel : **v0.2** — simulateurs (intérêts composés, épargne/DCA, prêt), « la courbe », portefeuille (positions, valorisation manuelle, +/− value, répartition, export).

## Le pari : riche ET simple

Richesse et simplicité ne s'opposent que si l'on empile des écrans. Pécule fait l'inverse — **de la profondeur cachée derrière une surface simple** :

1. **Simple par défaut, puissance à la demande.** Chaque fonction avancée a un défaut sensé et reste masquée tant qu'on ne la cherche pas (divulgation progressive, mode simple/expert).
2. **Tout au clavier — palette ⌘K.** Une app « infinie » reste navigable en tapant deux lettres : naviguer, ajouter, simuler, régler.
3. **Bons défauts, zéro configuration pour démarrer.**
4. **« La courbe » comme fil conducteur** unique entre tous les piliers (patrimoine, projections, dépenses).
5. **Local‑first non négociable.** Vos chiffres restent chez vous : aucune banque connectée, aucune télémétrie, aucune pub.
6. **Pédagogique, jamais prescripteur.** Hypothèses et repères explicites, pas de conseil financier.

Le plan couvre : le **catalogue exhaustif** des fonctionnalités, le **système d'expérience** qui les garde faciles, l'**architecture** qui les rend possibles, la **rigueur** des calculs, et la **roadmap** pour y arriver.

---


## 1. Catalogue exhaustif de fonctionnalités

Ce catalogue dresse, domaine par domaine, **tout ce qu'une app de finance perso de classe mondiale peut offrir**, adapté à Pécule (Tauri/Rust/SvelteKit, local-first, FR/RGPD, MIT). Pour chaque domaine : une liste riche marquée **`[E]` essentiel** (le défaut, le cœur de cible) vs **`[A]` avancé** (la puissance à la demande), et un encadré **« Comment ça reste simple »** précisant le défaut, l'endroit où la fonction se cache, et le raccourci/palette pour l'invoquer.

Convention de lecture : `[E]` = visible par défaut, zéro réglage requis ; `[A]` = masqué derrière un « Afficher plus », un mode expert, un onglet secondaire ou la palette ⌘K. Tout `[A]` doit avoir un défaut sensé pour ne jamais bloquer un débutant.

> Ancrage technique : le schéma SQLite actuel (`accounts`, `holdings`, `transactions`, `dividends`, `subscriptions`, `simulations`, `settings`) couvre déjà les piliers ; ce catalogue indique aussi les **tables/colonnes à ajouter** (en *italique*) pour chaque grand bloc, afin que le « quoi » reste réaliste vis-à-vis du code.

---

### 1. Patrimoine net (net worth) — la vue qui réconcilie tout

*Tables à ajouter : `account_balances` (snapshots datés par compte), `assets` (immo, véhicules, objets), `liabilities` (dettes hors prêts amortis), `net_worth_snapshots`.*

| | Fonctionnalité |
|---|---|
| `[E]` | **Agrégation de tous les comptes** : courant, livret A/LDDS/LEP, PEL/CEL, assurance-vie, PEA, CTO, PER, crypto wallet, espèces, immobilier, dettes — un seul nombre « patrimoine net ». |
| `[E]` | **Saisie manuelle d'un solde** par compte (cohérent avec le local-first : aucune banque connectée). Un clic = « mettre à jour le solde aujourd'hui ». |
| `[E]` | **Courbe du patrimoine net dans le temps** (la signature de Pécule appliquée au réel, pas seulement aux projections). |
| `[E]` | **Répartition actifs / passifs** (actifs liquides, placements, immobilier, dettes) en barre ou anneau. |
| `[E]` | **Distinction liquide vs illiquide** et **brut vs net** (patrimoine net = actifs − dettes). |
| `[A]` | **Snapshots historiques** : photo mensuelle automatique du patrimoine (table `net_worth_snapshots`) pour reconstituer une courbe même sans saisie quotidienne. |
| `[A]` | **Comptes « groupés »** (ex. « Épargne de précaution » regroupe Livret A + LDDS) et comptes joints / par personne (foyer). |
| `[A]` | **Actifs non financiers** : immobilier (avec valeur estimée révisable), véhicule, objets de valeur, parts de SCI/SCPI, stock-options/BSPCE. |
| `[A]` | **Évolution annotée** : marqueurs d'événements (« achat appart », « prime », « krach ») posés sur la courbe. |
| `[A]` | **Décomposition de la variation** (waterfall) : « +X versements, +Y plus-values, −Z retraits » entre deux dates. |
| `[A]` | **Patrimoine cible / barre de progression** vers un jalon (premier 100 k€, indépendance financière). |
| `[A]` | **Comparaison anonyme par tranche d'âge** (repères statistiques INSEE intégrés en local, jamais de données envoyées). |

> **Comment ça reste simple.** Au premier lancement, l'utilisateur ne voit qu'**un grand nombre + une courbe**. Ajouter un compte = un bouton « + Compte », trois champs (nom, type, solde). Tous les `[A]` (snapshots, actifs immo, waterfall) vivent derrière le détail d'un compte ou un « Afficher l'évolution détaillée ». Raccourci : **⌘K → « Patrimoine »** saute à la vue ; **⌘K → « Mettre à jour un solde »** ouvre la saisie rapide.

---

### 2. Portefeuille investissement — le pilier déjà amorcé (v0.2)

*Existe : `accounts`, `holdings (nom, symbole, type, quantité, pru, prix_actuel, devise)`, `transactions (sens, qté, prix, frais, date)`, `dividends`. À ajouter : `corporate_actions` (splits, regroupements), `price_history`, `benchmarks`.*

#### 2.1 Positions & comptes-enveloppes
| | Fonctionnalité |
|---|---|
| `[E]` | **Multi-classes d'actifs** : actions, ETF, obligations, fonds (OPCVM/SICAV), crypto, fonds euros, liquidités. |
| `[E]` | **Multi-comptes / enveloppes FR** : PEA, PEA-PME, CTO, assurance-vie, PER, livrets, wallet crypto — chaque position rattachée à un compte (déjà `account_id`). |
| `[E]` | **Valorisation manuelle** (prix actuel saisi) — le défaut local-first actuel. |
| `[E]` | **PRU (prix de revient unitaire)** déjà stocké ; affichage prix d'achat vs prix actuel. |
| `[A]` | **Rafraîchissement de cours publics** *optionnel et désactivé par défaut* (déjà prévu : seuls des prix publics entrent, les positions ne sortent jamais ; cf. `quotes.rs`). |
| `[A]` | **Multi-lignes par titre** (plusieurs lots d'achat) avec consolidation. |
| `[A]` | **Comptes en lecture seule / archivés** (anciens PEA clôturés conservés pour l'historique). |

#### 2.2 Coût de revient & opérations
| | Fonctionnalité |
|---|---|
| `[E]` | **Méthode de coût CUMP** (coût unitaire moyen pondéré) — défaut conforme à la fiscalité FR sur titres. |
| `[A]` | **Méthode FIFO** (et WAC/LIFO en option) pour comparer, avec note pédagogique « la méthode fiscale FR de référence est le PRU moyen pondéré ». |
| `[E]` | **Journal de transactions** : achat/vente, quantité, prix, **frais** (déjà en base), date. |
| `[A]` | **Opérations sur titres (corporate actions)** : splits, regroupements, distribution gratuite, fusion, changement de symbole — recalcul automatique du PRU et des quantités. |
| `[A]` | **Versements/retraits d'espèces** par enveloppe (pour TWR/MWR exacts). |
| `[A]` | **Import relevé** (CSV courtier, format générique) avec mapping de colonnes mémorisé. |

#### 2.3 Revenus du capital
| | Fonctionnalité |
|---|---|
| `[E]` | **Dividendes** (déjà table `dividends`) : montant, date, par position. |
| `[A]` | **Coupons d'obligations**, **distributions de SCPI/fonds**, **staking/rewards crypto**. |
| `[A]` | **Rendement sur dividende (yield)** courant et sur coût (yield-on-cost). |
| `[A]` | **Calendrier prévisionnel des dividendes** (dates de détachement saisies ou estimées). |
| `[A]` | **Réinvestissement (DRIP)** : marquer un dividende comme réinvesti → crée la transaction d'achat. |

#### 2.4 Plus-values & performance
| | Fonctionnalité |
|---|---|
| `[E]` | **+/− value latente** (position non vendue) et **réalisée** (après vente). |
| `[E]` | **Performance simple** (% gain) et en montant, par position et global. |
| `[A]` | **TWR (Time-Weighted Return)** : performance « pure » indépendante des dépôts. |
| `[A]` | **MWR / XIRR (taux de rendement interne)** : performance vécue par l'investisseur, tient compte du timing des versements. |
| `[A]` | **Performance annualisée (CAGR)** et **par période** (YTD, 1 an, 3 ans, depuis origine). |
| `[A]` | **Contribution à la performance** par position/classe (qui tire le portefeuille). |
| `[A]` | **Volatilité, max drawdown, ratio de Sharpe** (repères pédagogiques, masqués par défaut). |

#### 2.5 Allocation & benchmarks
| | Fonctionnalité |
|---|---|
| `[E]` | **Répartition par classe d'actifs** (actions/obligations/crypto/liquidités) — déjà esquissée. |
| `[A]` | **Allocation par secteur, zone géographique, devise, enveloppe**. |
| `[A]` | **Transparence des ETF** (look-through) : un ETF World ventilé en zones/secteurs si la composition est saisie. |
| `[A]` | **Benchmark** : comparer sa courbe à un indice de référence (CAC 40, MSCI World) saisi manuellement ou via cours optionnels. |
| `[A]` | **Cible d'allocation & dérive** : « vous visez 80/20, vous êtes à 86/14 » + suggestion de **rééquilibrage** (montant à acheter/vendre, sans conseil). |
| `[A]` | **Concentration / risque** : alerte « une position pèse > 25 % du portefeuille ». |

> **Comment ça reste simple.** Le portefeuille s'ouvre sur **3 chiffres** (valeur totale, +/− value, répartition) + la liste des positions. Ajouter une position = nom, symbole, quantité, prix d'achat (PRU), prix actuel — rien d'autre. TWR/XIRR, FIFO, corporate actions, secteurs : tout est sous un onglet **« Analyse »** et un menu **« Méthode de calcul »** (défaut CUMP). Les cours en ligne restent **OFF** jusqu'à activation explicite. Raccourcis : **⌘K → « Ajouter une transaction »**, **⌘K → « Marquer un dividende »**.

---

### 3. Budget & dépenses — le quotidien, sans friction

*Tables à ajouter : `categories`, `transactions_cash` (flux courants, distincts des transactions titres), `recurring`, `envelopes`, `rules` (catégorisation), `reconciliation`.*

| | Fonctionnalité |
|---|---|
| `[E]` | **Catégories de dépenses/revenus** prêtes à l'emploi (logement, courses, transport, loisirs, santé, salaire…). |
| `[E]` | **Saisie rapide d'une dépense** (montant, catégorie, date) — clavier d'abord. |
| `[E]` | **Flux de trésorerie mensuel** : entrées − sorties = reste à vivre / épargne du mois. |
| `[E]` | **Revenus & dépenses récurrents** (salaire, loyer, assurances) saisis une fois, projetés automatiquement. |
| `[E]` | **Top des postes du mois** (« où part l'argent »). |
| `[A]` | **Enveloppes budgétaires (méthode zéro-based / YNAB-like)** : allouer chaque euro, suivre le « disponible » par enveloppe. |
| `[A]` | **Règles de catégorisation** : « libellé contient *SNCF* → Transport » (mémorisées, table `rules`), appliquées à l'import. |
| `[A]` | **Import de relevés bancaires** (CSV/OFX/QIF) — l'utilisateur exporte de sa banque, importe en local ; aucun connecteur bancaire par défaut (RGPD/local-first). |
| `[A]` | **Rapprochement** : pointer les transactions importées contre la saisie, repérer les écarts. |
| `[A]` | **Budget glissant / moyenne 3 mois** comme base d'enveloppe. |
| `[A]` | **Dépenses partagées / remboursements** (qui me doit quoi — colocation, couple). |
| `[A]` | **Prévisionnel de trésorerie** : projeter le solde du compte courant sur 30/90 jours à partir des récurrences (relié à « la courbe »). |
| `[A]` | **Détection d'anomalies** : « ce poste a doublé vs la moyenne ». |

> **Comment ça reste simple.** Pécule **ne force pas à budgéter** : par défaut, on ne fait que **suivre** (catégories auto-suggérées, récurrences saisies une fois). Les enveloppes/zéro-based sont un **mode optionnel** (« Activer la budgétisation par enveloppe » dans Réglages → Budget). L'import est un **assistant** en 3 écrans (fichier → mapping → confirmation) avec mapping mémorisé. Raccourci : **⌘K → « Nouvelle dépense »** ouvre une mini-fiche en une ligne.

---

### 4. Abonnements — pilier prévu (v0.3), déjà en base

*Existe : `subscriptions (nom, catégorie, montant, fréquence, prochain_prélèvement, actif)`.*

| | Fonctionnalité |
|---|---|
| `[E]` | **Liste des abonnements** avec coût et fréquence (déjà modélisé). |
| `[E]` | **Coût mensuel ET annuel réel** : « 9,99 €/mois = **119,88 €/an** » (le déclic). |
| `[E]` | **Total « ce que vos abonnements vous coûtent par an »** en un nombre. |
| `[E]` | **Catégories** (streaming, logiciels, sport, presse, télécoms, assurances). |
| `[E]` | **Prochain prélèvement & calendrier** (déjà `prochain_prélèvement`). |
| `[A]` | **Alertes de renouvellement** (notification native Tauri J-3 avant prélèvement) — `alerts.rs` existe déjà. |
| `[A]` | **Détection d'abonnements peu/pas utilisés** : « dernière utilisation déclarée il y a 4 mois » → suggestion de résiliation. |
| `[A]` | **« Et si j'investissais ? »** : brancher le coût annuel d'un abonnement sur les **simulateurs** (« 120 €/an placés 20 ans à 5 % = … »). Pont direct vers la courbe. |
| `[A]` | **Hausses de prix dans le temps** (historique du montant par abonnement). |
| `[A]` | **Essais gratuits** : date de fin d'essai → alerte avant facturation. |
| `[A]` | **Périodicité atypique** (trimestriel, semestriel, hebdo) et abonnements partagés (coût par personne). |

> **Comment ça reste simple.** Vue par défaut = **un total annuel + la liste triée par coût décroissant**. Ajouter = nom + montant + fréquence (la conversion annuel/mensuel est automatique). Alertes, taux d'utilisation et « et si j'investissais » sont des actions au clic sur une ligne. Raccourci : **⌘K → « Ajouter un abonnement »**, **⌘K → « Coût annuel des abonnements »**.

---

### 5. Dettes & crédits — le miroir du patrimoine

*Tables à ajouter : `loans` (principal, taux, durée, assurance, date début), `loan_extra_payments`, lien vers le moteur d'amortissement Rust existant.*

| | Fonctionnalité |
|---|---|
| `[E]` | **Prêt immobilier** (déjà couvert par le simulateur de prêt : mensualité, assurance, coût total, **tableau d'amortissement**). |
| `[E]` | **Suivi du capital restant dû** dans le temps (intégré au patrimoine net comme passif). |
| `[E]` | **Prêts conso / auto / étudiant** (même moteur, durées courtes). |
| `[A]` | **Cartes de crédit / crédit renouvelable** (taux élevés mis en évidence). |
| `[A]` | **Remboursement anticipé** : simuler un versement supplémentaire → intérêts économisés + nouvelle durée (le moteur `loan` est déjà là). |
| `[A]` | **Modulation d'échéances** (hausse/baisse de mensualité, pause). |
| `[A]` | **Stratégies de désendettement** : **avalanche** (taux le plus haut d'abord) vs **boule de neige** (plus petit solde d'abord) — comparateur côte à côte. |
| `[A]` | **Renégociation / rachat de crédit** : comparer prêt actuel vs nouveau (taux, frais de dossier, IRA). |
| `[A]` | **Taux d'endettement** (mensualités / revenus) avec repère 35 %. |
| `[A]` | **Multi-prêts agrégés** (immo + travaux + conso) : échéancier consolidé. |

> **Comment ça reste simple.** Un prêt se crée avec les **4 champs du simulateur** (capital, taux, durée, assurance) ; le tableau d'amortissement et le capital restant sont calculés. Remboursement anticipé et stratégies avalanche/boule de neige sont des **boutons « Et si je… »** sur la fiche du prêt, branchés sur la courbe. Raccourci : **⌘K → « Simuler un remboursement anticipé »**.

---

### 6. Objectifs & épargne — donner un sens aux chiffres

*Tables à ajouter : `goals` (nom, montant cible, date cible, compte(s) lié(s), versement mensuel).*

| | Fonctionnalité |
|---|---|
| `[E]` | **Objectifs datés** : « 10 000 € pour août 2027 », barre de progression. |
| `[E]` | **Fonds d'urgence** : objectif spécial calculé en **mois de dépenses** (relié au budget). |
| `[E]` | **Projection vers l'objectif** : « à ce rythme, atteint en mai 2027 » (la courbe appliquée à l'objectif). |
| `[E]` | **Versement nécessaire** : « pour y être à la date, épargnez X €/mois ». |
| `[A]` | **Objectifs liés à des comptes réels** : progression alimentée par les soldes saisis, pas seulement déclarative. |
| `[A]` | **Plusieurs objectifs simultanés** avec priorités et répartition d'un versement entre eux. |
| `[A]` | **Objectifs avec rendement** (épargne placée, pas seulement thésaurisée) → projection avec taux. |
| `[A]` | **Jalons & encouragements** (25/50/75/100 %) avec micro-célébration discrète. |
| `[A]` | **Objectif inversé** : « combien à la retraite avec X €/mois ? » (pont vers simulateur FIRE/retraite). |

> **Comment ça reste simple.** Créer un objectif = **nom + montant + date** → Pécule calcule le versement mensuel et trace la courbe. Le fonds d'urgence propose un défaut **« 3 à 6 mois de dépenses »** pré-rempli. Tout le reste (multi-objectifs, rendement, comptes liés) est en « Options de l'objectif ». Raccourci : **⌘K → « Nouvel objectif »**.

---

### 7. Simulateurs — le cœur historique, à étendre

*Existe : moteurs Rust purs `compound`, `savings (DCA)`, `loan` + `simulations (type, paramètres_json, créé_le)` pour sauvegarder. « La courbe » avec curseur « et dans X années ? ».*

| | Fonctionnalité |
|---|---|
| `[E]` | **Intérêts composés** (capital placé une fois, capitalisation au choix) — livré. |
| `[E]` | **Épargne régulière / DCA** (capital initial + versements mensuels) — livré. |
| `[E]` | **Prêt** (mensualité, assurance, coût, amortissement) — livré. |
| `[E]` | **Curseur temporel « et dans X années ? »** sur chaque simulation — livré. |
| `[A]` | **FIRE / indépendance financière** : capital cible (règle des 4 % / 25× dépenses), date d'atteinte, taux de retrait sûr. |
| `[A]` | **Retraite** : projection capital + rente, intégrant PER et estimations de pension. |
| `[A]` | **Effet de l'inflation** : afficher les montants en euros constants (pouvoir d'achat) vs courants — un simple **toggle** réutilisable partout. |
| `[A]` | **Louer vs acheter** : comparer coût total locataire vs propriétaire (apport, intérêts, taxe foncière, entretien, plus-value, coût d'opportunité de l'apport placé). |
| `[A]` | **Simulateur fiscal** (voir §8) : PFU vs barème, AV après 8 ans, plus-value PEA. |
| `[A]` | **DCA vs investissement unique (lump sum)** : comparer les deux stratégies sur historique ou taux supposé. |
| `[A]` | **Comparateur de scénarios** : superposer 2-3 courbes (« prudent 3 % / médian 5 % / optimiste 7 % ») sur le même graphe. |
| `[A]` | **Scénarios stochastiques (Monte-Carlo)** : fourchette probable (P10/P50/P90) plutôt qu'une courbe unique — masqué, pour avertir contre la fausse précision. |
| `[A]` | **Sauvegarde & comparaison de simulations** (table `simulations` déjà prévue) : nommer, retrouver, dupliquer, comparer. |
| `[A]` | **Variables avancées** : frais annuels (TER ETF), fiscalité sur les gains, versements croissants (indexés inflation/salaire), versements ponctuels (primes). |

> **Comment ça reste simple.** Chaque simulateur s'ouvre avec **des défauts plausibles déjà remplis** (ex. 5 %/an, 20 ans) → la courbe s'affiche **immédiatement**, l'utilisateur ajuste ensuite. Les variables avancées (frais, fiscalité, inflation, versements croissants) sont sous **« Réglages avancés »** repliés ; le toggle inflation est un seul bouton. Le comparateur de scénarios = bouton **« + Comparer »**. Raccourci : **⌘K → « Simuler … »** (composés, DCA, prêt, FIRE, retraite, louer/acheter).

---

### 8. Fiscalité FR (pédagogique) — le différenciateur français

*Aucune donnée n'est transmise ; calculs locaux, repères datés et révisables. **Toujours estampillé « pédagogique, pas un conseil ».***

| | Fonctionnalité |
|---|---|
| `[E]` | **Repères des enveloppes** : fiches courtes PEA / PEA-PME / CTO / Assurance-vie / PER / Livrets (plafonds, fiscalité, blocage). |
| `[E]` | **PFU / « flat tax » (30 %)** : calcul de l'impôt sur une plus-value ou un dividende CTO. |
| `[A]` | **PFU vs barème progressif** : lequel est plus avantageux selon la TMI saisie. |
| `[A]` | **Assurance-vie** : fiscalité avant/après 8 ans, abattement annuel (4 600 / 9 200 €), prélèvements sociaux (17,2 %). |
| `[A]` | **PEA** : exonération d'IR après 5 ans (hors prélèvements sociaux), conséquences d'un retrait avant 5 ans. |
| `[A]` | **PER** : déduction des versements selon TMI (économie d'impôt à l'entrée vs fiscalité à la sortie). |
| `[A]` | **Plus-values mobilières** : calcul global, abattements pour durée de détention (titres ante-2018), report de moins-values. |
| `[A]` | **Plus-value immobilière** : abattements par durée de détention (résidence principale exonérée). |
| `[A]` | **Prélèvements sociaux** (17,2 %) appliqués automatiquement où ils s'imposent. |
| `[A]` | **Estimation d'IFI** (patrimoine immobilier net > seuil) — indicatif. |
| `[A]` | **Optimisation d'enveloppe** : « pour cet objectif/horizon, l'AV/PEA serait pertinent parce que… » (explication, jamais recommandation d'actif). |
| `[A]` | **Paramètres fiscaux versionnés** : taux/plafonds dans un fichier local daté (« barème 2026 »), modifiables, avec avertissement de péremption. |

> **Comment ça reste simple.** La fiscalité est d'abord de la **pédagogie contextuelle** : une petite icône « ⓘ impôt » à côté d'une plus-value ouvre l'explication et le calcul, sans page dédiée obligatoire. Les comparateurs (PFU vs barème, AV avant/après 8 ans) sont des **mini-simulateurs** atteignables via ⌘K. Un bandeau permanent rappelle **« pédagogique — pas un conseil fiscal »** (cohérent avec `Disclaimer.svelte`). Raccourci : **⌘K → « Impôt sur une plus-value »**.

---

### 9. Rapports & insights — transformer les données en compréhension

| | Fonctionnalité |
|---|---|
| `[E]` | **Bilan mensuel** : patrimoine, épargne du mois, top dépenses, en un écran. |
| `[E]` | **Tendances** : évolution sur 6/12 mois (patrimoine, dépenses, épargne). |
| `[E]` | **Tableau de bord d'accueil** synthétique (patrimoine, reste à vivre, prochain événement). |
| `[A]` | **Bilan annuel** (« votre année financière ») : entrées/sorties, performance investissements, impôts estimés, faits marquants. |
| `[A]` | **Alertes intelligentes** (locales) : dérive d'allocation, abonnement inutilisé, fonds d'urgence sous le seuil, mensualité > 35 % des revenus, dépense anormale. |
| `[A]` | **Calendrier d'événements financiers** : prélèvements d'abonnements, échéances de prêt, dividendes attendus, dates fiscales (déclaration, IFU). |
| `[A]` | **Insights « langage clair »** : phrases générées localement (« vous épargnez en moyenne 420 €/mois », « vos loisirs ont augmenté de 18 % »). |
| `[A]` | **Rapports exportables** : PDF/CSV du bilan, du portefeuille, de l'amortissement (pour soi, son comptable, sa banque). |
| `[A]` | **Comparaison période vs période** (ce mois vs le même mois l'an dernier). |
| `[A]` | **Widget / résumé au lancement** (« depuis hier : patrimoine +0,4 % »). |

> **Comment ça reste simple.** L'accueil montre **3 cartes** max (patrimoine, mois en cours, prochain événement) ; le reste des rapports est dans un onglet **« Bilans »**. Les insights sont des **phrases**, pas des graphiques à déchiffrer. Les alertes sont **discrètes et désactivables** par type. Raccourci : **⌘K → « Bilan du mois »**, **⌘K → « Exporter un rapport »**.

---

### 10. Transversal — multi-devises, pièces jointes, notes, tags, données

*Tables à ajouter : `attachments`, `tags`, `entity_tags`, `fx_rates` ; `settings` déjà présent pour les préférences.*

| | Fonctionnalité |
|---|---|
| `[E]` | **Devise principale** (EUR par défaut) et formatage FR (espaces, virgule décimale, chiffres tabulaires — déjà en place). |
| `[E]` | **Notes libres** sur n'importe quel élément (position, dépense, objectif). |
| `[E]` | **Tags transverses** (ex. « voyage Japon », « travaux ») reliant dépenses, objectifs, transactions. |
| `[A]` | **Multi-devises** : positions et comptes en USD/CHF/GBP/crypto, conversion vers devise principale via **taux saisis manuellement** (ou taux publics optionnels, OFF par défaut). Champ `devise` déjà présent sur `holdings`. |
| `[A]` | **Pièces jointes** : factures, contrats, RIB, relevés — **fichiers chiffrés stockés en local** (jamais d'upload). |
| `[A]` | **Recherche globale** sur toutes les entités (palette ⌘K cherche aussi les données, pas que les actions). |
| `[A]` | **Champs personnalisés** sur les entités (pour power users). |

> **Comment ça reste simple.** Une seule devise et EUR par défaut : le multi-devises n'apparaît que si on saisit une position dans une autre monnaie. Notes et tags sont des **petits champs optionnels** en bas de chaque fiche. Les pièces jointes = glisser-déposer sur une fiche. Raccourci : **⌘K** est la recherche universelle.

---

### 11. Confiance, données & vie privée — fonctionnalités à part entière

Chez Pécule, la confidentialité **est** une fonctionnalité (pas une mention légale). À traiter comme un domaine produit.

| | Fonctionnalité |
|---|---|
| `[E]` | **Local-first non négociable** : SQLite local (déjà en place), aucun compte, aucune banque connectée, **zéro télémétrie, zéro pub**. |
| `[E]` | **Export complet** : JSON (déjà pour le portefeuille) + sauvegarde de toute la base, à tout moment, sans friction (anti-lock-in). |
| `[E]` | **Import / restauration** depuis un export (changer de machine sans rien perdre). |
| `[A]` | **Chiffrement au repos / mot de passe maître** (SQLCipher) — `crypto.rs` déjà esquissé, prévu v2. |
| `[A]` | **Verrouillage de l'app** (mot de passe / Touch ID) au lancement ou après inactivité. |
| `[A]` | **Sauvegardes automatiques locales** (rotation, dossier choisi par l'utilisateur). |
| `[A]` | **Mode confidentialité** : masquer les montants d'un raccourci (« blur ») pour consulter en public. |
| `[A]` | **Cours publics optionnels** : panneau clair de ce qui sort (rien) et entre (prix publics), domaines en liste blanche (CSP stricte déjà en place). |
| `[A]` | **Profils / coffres multiples** (perso, foyer, asso) en bases séparées. |
| `[A]` | **Effacement sûr** (supprimer définitivement une base/un profil). |

> **Comment ça reste simple.** Tout marche **sans aucun réglage de confidentialité** car le défaut est déjà le plus sûr. Export/import = un bouton chacun dans Réglages → Données. Le chiffrement et le verrouillage sont **opt-in** en une bascule. Mode confidentialité = raccourci **⌘⇧M (masquer les montants)**. Raccourci : **⌘K → « Exporter mes données »**, **⌘K → « Sauvegarder maintenant »**.

---

### Synthèse — où vit chaque domaine (carte mentale de l'UI)

| Domaine | Surface par défaut (essentiel) | Puissance cachée (avancé) | Entrée ⌘K type |
|---|---|---|---|
| Patrimoine net | 1 nombre + 1 courbe | snapshots, immo, waterfall, foyer | « Patrimoine », « Mettre à jour un solde » |
| Portefeuille | valeur, +/− value, répartition, positions | TWR/XIRR, FIFO, corporate actions, secteurs, benchmark | « Ajouter une transaction » |
| Budget | catégories + flux mensuel | enveloppes, règles, import, rapprochement | « Nouvelle dépense » |
| Abonnements | total annuel + liste triée | alertes, taux d'usage, « et si j'investissais » | « Ajouter un abonnement » |
| Dettes | prêt + capital restant dû | anticipé, avalanche/boule de neige, renégociation | « Remboursement anticipé » |
| Objectifs | nom + montant + date → courbe | multi-objectifs, rendement, comptes liés | « Nouvel objectif » |
| Simulateurs | 3 moteurs + courbe (défauts pré-remplis) | FIRE, retraite, louer/acheter, inflation, Monte-Carlo | « Simuler … » |
| Fiscalité FR | fiches + PFU 30 % | PFU vs barème, AV 8 ans, PER, plus-values | « Impôt sur une plus-value » |
| Rapports | accueil 3 cartes | bilan annuel, alertes, calendrier, export PDF | « Bilan du mois » |
| Transversal | EUR, notes, tags | multi-devises, pièces jointes, champs perso | « Rechercher » |
| Vie privée | local, export/import | chiffrement, verrouillage, profils, masquage | « Exporter mes données » |

**Principe directeur du catalogue** : chaque domaine offre une **porte d'entrée à un seul écran** (un nombre, une courbe, une liste) et **toute la profondeur derrière un repli, un onglet « Analyse/Options », ou la palette ⌘K**. La richesse est infinie ; la première impression reste celle de trois ou quatre chiffres clairs et d'une courbe.

*Fichiers de référence pour l'ancrage technique : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/db.rs` (schéma SQLite), `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src/lib/ipc.ts` (types & commandes IPC), `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/{quotes,crypto,alerts}.rs` (cours publics, chiffrement, alertes déjà esquissés), `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src/lib/components/{Curve,Disclaimer}.svelte`.*

---

## 2. Système d’expérience — riche mais simple

### Principe directeur : « simple par défaut, puissance à la demande »

Pécule peut tout faire, mais ne montre presque rien au démarrage. Trois règles non négociables encadrent toute fonctionnalité ajoutée :

1. **Défaut intelligent** — chaque réglage a une valeur juste pour 90 % des gens (taux 5 %/an, devise EUR, capitalisation mensuelle, thème sombre). On peut tout utiliser sans rien régler.
2. **Surface cachée mais accessible** — une fonction avancée n'apparaît jamais d'office : elle se révèle (bouton « Plus d'options ») ou se lance (⌘K). La densité visuelle reste constante quand la puissance augmente.
3. **Tout au clavier** — chaque action atteignable à la souris est atteignable au clavier, et toute action ayant un nom est joignable par ⌘K. Le pouvoir n'est jamais piégé dans un menu profond.

Aujourd'hui, le socle existe déjà et porte ces règles : `Field.svelte` (champ + curseur + bornage `clampMax`/`clamp`), `Curve.svelte` (curseur épinglé `pinned`, scrub temps réel), `Sidebar.svelte` (groupes primaire / simulateurs / secondaire + badges « Bientôt »), `theme.svelte.ts` (clair/sombre persistant) et l'i18n FR-prioritaire avec repli. Les mécanismes ci-dessous étendent ce socle sans l'alourdir.

---

### 1. Palette de commandes ⌘K — le cœur du système

C'est l'outil qui permet d'ajouter une infinité de fonctions sans jamais surcharger l'écran : tout vit dans la palette, l'interface visible ne montre que l'essentiel.

**Déclenchement** : `⌘K` (macOS) / `Ctrl K` (Win/Linux). Une seule entrée mentale pour « faire quelque chose ». Esc ferme, `↑/↓` navigue, `Entrée` exécute, `Tab` accepte la complétion.

**Recherche unifiée** : un seul champ cherche simultanément dans 4 registres, fusionnés et classés par récence + correspondance floue :
- **Navigation** — « Portefeuille », « Prêt immobilier », « Réglages ».
- **Actions** — « Ajouter une position », « Nouvel abonnement », « Exporter en JSON », « Basculer le thème », « Changer de devise ».
- **Données** — vos propres positions, catégories, abonnements (« Air Liquide », « Netflix »).
- **Aide / définitions** — « C'est quoi le TAEG ? », « Qu'est-ce que le DCA ? » (ouvre l'aide inline, jamais le web).

**Exemples concrets de commandes Pécule** :

| Frappe | Effet |
|---|---|
| `aj pos` | « Ajouter une position » → quick-add (voir §4) |
| `200 /mois 5% 30 ans` | lance l'**épargne (DCA)** pré-remplie, la courbe s'affiche aussitôt |
| `pret 250000 3.5 25` | **prêt** : capital, taux, durée → mensualité immédiate |
| `>thème` | actions système (le préfixe `>` force le mode action) |
| `@netflix` | saute droit à l'abonnement (le préfixe `@` force la recherche de données) |
| `export` | propose « Exporter portefeuille (JSON) », « Exporter la courbe (PNG) » |
| `fire` | ouvre le simulateur d'indépendance financière (badge « Bientôt ») |

**Comment ça reste simple** : la palette est facultative — débutant à la souris, expert au clavier, même app. Les **commandes paramétrées** (`200 /mois 5% 30 ans`) acceptent un langage naturel tolérant : ordre libre, unités optionnelles (`%`, `/mois`, `ans`), valeurs manquantes complétées par les défauts. La palette **enseigne** : chaque commande affiche son raccourci à droite, donc l'usage répété fait apprendre le raccourci sans effort.

---

### 2. Divulgation progressive (progressive disclosure)

**Mode Simple / Avancé par vue.** Chaque simulateur et le portefeuille ont un interrupteur discret en haut à droite. Le mode **Simple** (défaut) n'affiche que 2-3 champs essentiels ; le mode **Avancé** déplie le reste.
- *Prêt — Simple* : capital, taux, durée → mensualité. *Avancé* : assurance (taux ou €/mois), frais de dossier, frais de garantie, modulation d'échéance, remboursement anticipé, périodicité.
- *Épargne — Simple* : versement mensuel, taux, durée. *Avancé* : capital de départ, versement en début/fin de période, indexation annuelle des versements, prélèvements (PFU 30 %), inflation (courbe en euros constants).

**Sections repliées « Plus d'options ».** Le champ rare n'est jamais supprimé : il est sous un dépliant fermé par défaut, avec un compteur (« Plus d'options · 4 »). Replié = invisible mais pas perdu.

**Réglages à 3 profondeurs.** *Réglages* (déjà présents) → bloc « Avancé » → bloc « Expert » (gestion du fichier de données, dossier de stockage, base chiffrée, format des nombres). Le débutant ne voit jamais l'expert.

**Comment ça reste simple** : le choix Simple/Avancé est **mémorisé par vue** (réutilise le pattern de persistance de `theme.svelte.ts`). Un avertissement « Outil pédagogique » (`Disclaimer.svelte`) reste visible quel que soit le mode : la rigueur ne se cache pas.

---

### 3. Architecture de l'information

**Navigation latérale stable** (existante dans `Sidebar.svelte`), structurée en trois zones qui ne bougent jamais :
- **Primaire** : Tableau de bord, Portefeuille.
- **Simulateurs** (groupe nommé) : Intérêts composés, Épargne, Prêt, FIRE (« Bientôt »).
- **Bas / secondaire** : Abonnements, Réglages.

La barre latérale a une vocation : « où suis-je ». La palette ⌘K a l'autre : « que faire ». On ne mélange jamais les deux — c'est ce qui permet d'ajouter 30 fonctions sans rallonger la sidebar.

**Tableau de bord modulaire (widgets).** Le dashboard devient une grille de tuiles réorganisables (glisser-déposer, masquer/afficher). Widgets : « La courbe » (patrimoine projeté), Répartition du portefeuille, +/- value du mois, Coût annuel des abonnements, Prochain renouvellement, Raccourci vers le dernier simulateur utilisé.
- *Simple par défaut* : 3 widgets pré-arrangés, pertinents même sans données (la courbe d'exemple tourne déjà, cf. `+page.svelte`). Le bouton « Personnaliser » est discret ; un menu « ⋯ » par tuile suffit pour masquer.
- *Disposition mémorisée localement* (jamais de serveur), réinitialisable d'un clic.

**Vues personnalisables (portefeuille).** Colonnes activables, regroupement par enveloppe / classe d'actifs / devise, tri persistant, **vues enregistrées** (« PEA seul », « Crypto », « Ce qui baisse ce mois-ci ») accessibles via ⌘K. La vue par défaut reste un tableau simple : positions, valeur, +/- value.

**Hiérarchie typographique** déjà cadrée par `tokens.css` (`--fs-display` 40 → `--fs-small` 13, chiffres `--font-mono` tabulaires) : le montant projeté domine, les libellés s'effacent en `--muted`. La couleur **encode du sens** et rien d'autre — sarcelle = action/marque, vert `--gain` = plus-value, rouge `--loss` = moins-value, ambre `--soon` = échéance proche. Cette grammaire couleur rend chaque écran lisible d'un coup d'œil sans légende.

---

### 4. Saisie ultra-rapide

C'est le point qui décide si « beaucoup de fonctions » devient « beaucoup de corvées » ou non.

**Quick-add universel** (`A` ou ⌘K → « Ajouter »). Un seul champ devine le type :
- `AAPL 10 @150` → 10 actions Apple à 150.
- `Netflix 13,99 /mois` → abonnement mensuel.
- `Livret A 8000` → position cash.

Une barre de confirmation (style « toast ») montre l'interprétation avec **annulation** immédiate (cf. §6). On valide à l'aveugle sans ouvrir de formulaire.

**Formulaires intelligents** (extension du `Field.svelte` existant) :
- **Bornage vivant** déjà en place (`clampMax` à la frappe, `clamp` au blur) : impossible d'entrer une valeur absurde qui casserait la courbe.
- **Curseur + champ jumelés** : régler à la souris ou taper la valeur, synchronisés (pattern actuel du `Field`). Le débutant pousse le curseur, l'expert tape.
- **Unités et format FR** : séparateur de milliers, virgule décimale, `€` et `%` affichés en suffixe `--muted`, jamais à saisir.
- **Tab-flow** : ordre de tabulation logique, `Entrée` recalcule, focus initial sur le premier champ utile.

**Valeurs par défaut et mémoire** : chaque champ se souvient de la dernière saisie (devise, taux, durée). Ouvrir un simulateur, c'est déjà avoir un résultat à l'écran — on ajuste, on ne crée pas de zéro.

**Autocomplétion** :
- **Symboles** (actions/ETF/crypto) depuis un **catalogue local embarqué** (ISIN/ticker → nom), zéro réseau, conforme à la promesse local-first. La saisie `air liq` propose « Air Liquide (AI.PA) ».
- **Catégories d'abonnements** apprises de l'historique de l'utilisateur (Streaming, Logiciel, Assurance…), proposées avant qu'il tape.

**Collage CSV & glisser-déposer** :
- **Coller** un tableau (Ctrl V depuis Excel/Numbers) ouvre un **mapping de colonnes** à 1 écran : Pécule devine les colonnes (symbole, quantité, prix, date) ; on corrige, on importe.
- **Déposer** un fichier `.csv` / `.json` sur la fenêtre déclenche le même flux (Tauri gère le drag-and-drop OS natif).
- L'**export** (JSON déjà livré) devient symétrique : import du même format = sauvegarde/restauration sans compte.

**Comment ça reste simple** : aucune de ces voies n'est obligatoire. Le débutant tape un montant dans un champ ; l'utilisateur pressé colle un CSV ; l'expert fait `A` puis du texte. Trois portes, une seule destination.

---

### 5. Onboarding, écrans vides utiles, aide contextuelle

**Onboarding en 30 secondes, sautable.** 3 cartes, pas un tunnel : (1) « Vos chiffres restent chez vous » (la promesse, avec lien vérifiable vers Réglages → confidentialité), (2) choix devise + thème (déjà réglés par défaut), (3) « Essayez la courbe » — un curseur à bouger en direct. Aucun compte, aucune donnée demandée. L'onboarding **fait toucher** la valeur signature (« la courbe ») avant toute saisie.

**Écrans vides qui travaillent** (`EmptyState.svelte` existe déjà). Un portefeuille vide n'affiche pas « 0 € » triste mais : une icône, une phrase, et **2-3 actions amorces** — « Ajouter une position », « Coller depuis un tableur », « Voir un exemple ». Le vide enseigne le premier geste.

**Aide contextuelle inline, jamais de manuel externe.** Petit `(i)` à côté des termes (TAEG, DCA, PFU, capitalisation) → infobulle d'une phrase claire, sans jargon, ouverte au clavier (focusable). L'avertissement pédagogique (`Disclaimer.svelte`) reste l'ancre de confiance. La définition vit **dans** l'app : cohérent avec « zéro réseau ».

---

### 6. Recherche, filtres, tri, undo/redo, multi-sélection, raccourcis

**Recherche globale** : intégrée à ⌘K (§1) ; pas de second champ à apprendre.

**Filtres & tri** (portefeuille, abonnements) : filtres en « chips » empilables (classe d'actif, enveloppe, devise, « en moins-value »), tri par en-tête de colonne, état **persistant et nommable** (vues enregistrées, §3). Vue par défaut = aucun filtre.

**Undo / redo global** (`⌘Z` / `⌘⇧Z`) sur **toute mutation de données** : ajout, suppression, édition, import CSV. Filet de sécurité indispensable quand l'app est puissante — on ose cliquer parce qu'on peut revenir. Chaque action destructive émet aussi un toast « Annuler » de quelques secondes.

**Multi-sélection** : `clic`, `Maj+clic` (plage), `⌘+clic` (discontinu), `⌘A` (tout). Actions groupées sur la sélection : supprimer, changer d'enveloppe, exporter le sous-ensemble. Une barre d'actions n'apparaît que quand ≥ 1 ligne est sélectionnée (divulgation progressive appliquée aux actions).

**Liste de raccourcis** (affichable via `?`) :

| Raccourci | Action |
|---|---|
| `⌘K` / `Ctrl K` | Palette de commandes |
| `A` | Ajout rapide |
| `?` | Aide / liste des raccourcis |
| `⌘Z` / `⌘⇧Z` | Annuler / Rétablir |
| `⌘F` | Rechercher dans la vue |
| `⌘,` | Réglages |
| `G` puis `P` / `S` / `B` | Aller à Portefeuille / Simulateurs / Abonnements (chord « go to ») |
| `1` `2` `3` | Bascule entre simulateurs (intérêts / épargne / prêt) |
| `←` `→` | Déplacer le curseur de « la courbe » d'un an |
| `T` | Basculer le thème |
| `Esc` | Fermer panneau / palette |

**Comment ça reste simple** : tous ces raccourcis sont **optionnels et découvrables** — `?` révèle la carte, la palette montre chaque raccourci en regard de sa commande. On peut vivre une vie entière à la souris.

---

### 7. « La courbe » comme fil conducteur unifié

`Curve.svelte` est déjà l'élément signature ; il devient le **langage commun** des trois piliers, ce qui réduit la charge d'apprentissage : une seule métaphore à comprendre pour toute l'app.

- **Même composant, mêmes gestes partout** : curseur épinglé à l'horizon (`pinned`), scrub temps réel, lecture « et dans X années ? », libellé « aujourd'hui → +N ans ». Quiconque a compris la courbe d'un simulateur a compris celle du patrimoine.
- **Empilement de scénarios** : superposer 2-3 courbes (« avec / sans remboursement anticipé », « 200 € vs 300 €/mois ») dans le même cadre, avec la grammaire couleur de `tokens.css`. La comparaison, fonction avancée, ne coûte qu'un bouton « Comparer ».
- **Pont entre piliers** : depuis le portefeuille, « Projeter cette position » envoie ses paramètres réels au simulateur d'épargne et affiche la courbe ; depuis un abonnement, « Coût sur 10 ans » trace ce que représente un abonnement capitalisé. Le fil narratif relie « ce que j'ai » et « ce que je pourrais avoir » sans nouvelle interface.
- **Curseur au clavier** (`←/→`) et lecture vocale via `aria-label` déjà présent sur le `<svg>` : la courbe est utilisable sans souris ni vue.

---

### 8. Accessibilité, thèmes, densité, i18n

**WCAG AA, clavier complet.** Tout focusable, ordre logique, anneau de focus visible (`--focus` existe déjà). Les éléments « informatifs mais inertes » (badge « Bientôt » dans la sidebar) sont déjà neutralisés (`pointer-events: none`). Cibles ≥ 24 px. Aucune action réservée au survol.

**Lecteurs d'écran.** Pattern déjà amorcé : `aria-current="page"` sur l'item actif, `aria-label`/`aria-valuetext` sur le curseur de la courbe, `role="img"` + description chiffrée sur le graphe. À généraliser : annonces `aria-live` polies pour « Position ajoutée », « Annulé », et résultats de recalcul.

**Reduced-motion.** `--dur-fast/--dur-mid` centralisés dans `tokens.css` : un seul `@media (prefers-reduced-motion: reduce)` les neutralise globalement (animations de la courbe, `AnimatedNumber.svelte` figé sur la valeur finale). Le mouvement est un plus, jamais un prérequis.

**Thème clair / sombre.** Déjà livré (`[data-theme="light"]`, `toggleTheme`), sombre par défaut, choix persistant, raccourci `T`. Ajouter « suivre le système » comme troisième option.

**Densité.** Réglage « Confort / Compact » qui ne touche qu'aux variables d'espacement (`--space-*`) : l'expert avec un grand portefeuille gagne en lignes visibles, le débutant garde de l'air. Une variable, deux mondes.

**Contraste.** Les couleurs sémantiques ont déjà une variante claire mieux contrastée (`--gain` `#2FBF71` → `#128A50` en clair) ; viser AA sur texte et éléments d'UI dans les deux thèmes, avec une option « contraste renforcé » dans Réglages → Avancé.

**i18n.** Système en place (`messages.ts`, FR prioritaire, repli FR si clé EN manquante, bascule live dans `Header.svelte`). Formats nombres/dates/devises localisés. Le **français sans jargon** est la langue de référence ; chaque libellé EN est une traduction, jamais un compromis sur la clarté FR.

---

### 9. Micro-interactions, feedback, états de chargement et d'erreur

**Motion sobre et fonctionnel.** Easing unique (`--ease`), durées courtes (`--dur-fast` 120 ms), centralisés. `AnimatedNumber.svelte` fait « rouler » les montants quand la courbe bouge — assez pour montrer le lien cause→effet, jamais assez pour distraire. Le mouvement sert toujours à expliquer un changement, jamais à décorer.

**Feedback immédiat.** Le recalcul est anti-course (jeton `runId` déjà utilisé dans `+page.svelte`) et **débouncé** (~40 ms) : la courbe suit le doigt sur le curseur sans clignoter ni recalculer à contretemps. Chaque action (ajout, export, suppression) produit un toast bref avec « Annuler ».

**États de chargement.** Local-first = quasi-instantané ; les rares attentes (import d'un gros CSV, futur rafraîchissement de cours publics **opt-in**) montrent un **squelette** de la vue cible, pas un spinner anonyme. La promesse « tout est sur votre machine » se ressent dans la vitesse.

**États d'erreur utiles, jamais culpabilisants.** Message en français clair + cause + action de sortie : « Cette colonne CSV n'a pas pu être lue comme un prix. → Choisir une autre colonne / Ignorer cette ligne. » Le futur rafraîchissement de cours, désactivé par défaut, échoue **silencieusement et localement** (« cours non mis à jour, dernière valeur conservée le JJ/MM ») — jamais une erreur réseau anxiogène, conformément au positionnement « calme » de Pécule.

---

### Synthèse : le contrat d'expérience

Chaque nouvelle fonctionnalité de Pécule doit passer ce test avant d'exister :
1. **A-t-elle un défaut qui la rend invisible quand on n'en a pas besoin ?**
2. **Est-elle joignable par ⌘K et par raccourci ?**
3. **Son ajout laisse-t-il l'écran de départ aussi calme qu'avant ?**
4. **Parle-t-elle français clair, sans jargon, avec une aide inline ?**
5. **Reste-t-elle 100 % locale et réversible (undo) ?**

Tant que la réponse est « oui » cinq fois, on peut empiler les fonctions à l'infini : l'app gagne en puissance sans jamais perdre en simplicité.

---

## 3. Architecture technique, données, sécurité, extensibilité

L'état du dépôt aujourd'hui : backend Rust avec `finance/` (moteurs purs `compound`/`savings`/`loan`), `commands/` (enveloppes IPC `simulate`/`portfolio`/`settings`), trois placeholders déjà créés (`crypto.rs`, `quotes.rs`, `alerts.rs`), une base SQLite via `rusqlite` *bundled* avec migrations `PRAGMA user_version` (v1 → v2), un pont IPC TS typé (`src/lib/ipc.ts`) qui retombe sur un moteur TS hors Tauri. Le grand plan ci-dessous **étend** cette architecture sans la renier : chaque ajout suit les conventions déjà posées (moteurs purs, fonctions `q_*` testables, sérialisation `camelCase`, frontière IPC qui assainit).

Principe directeur transverse : **la complexité vit dans des modules isolés et optionnels ; le chemin par défaut reste celui d'aujourd'hui**. Un utilisateur qui n'active ni chiffrement, ni cours, ni sync, ni plugins, a exactement l'app v0.2 — ces sous-systèmes sont *dormants* (feature flags, settings, fichiers absents) tant qu'on ne les réveille pas.

---

### 1. Structure modulaire Rust (`src-tauri/src/`)

On garde la racine légère (`lib.rs` = setup + `invoke_handler`) et on découpe par **domaine métier**, pas par couche technique. Chaque domaine expose : un module de calcul/logique pur (testable sans Tauri), un module `commands` (fines enveloppes IPC qui verrouillent la connexion et assainissent), et éventuellement un module de persistance (`q_*` libres prenant `&Connection`).

```
src-tauri/src/
├── lib.rs                 # setup Tauri, manage(state), invoke_handler (registre central)
├── main.rs
├── error.rs              # NOUVEAU — enum AppError + impl Serialize, code/message stables (cf. §2)
├── db/
│   ├── mod.rs            # Db(Mutex<Connection>), open(), pool de prepared stmts optionnel
│   ├── schema.rs         # const SCHEMA (extrait de db.rs actuel)
│   └── migrations.rs     # migrate() piloté par user_version, une fn par palier (cf. §3)
├── finance/             # EXISTANT — moteurs PURS, aucune I/O, déterministes
│   ├── mod.rs           # SeriesPoint, months_count, MAX_PROJECTION_MONTHS
│   ├── compound.rs · savings.rs · loan.rs        # existants
│   ├── fire.rs          # taux de retrait sûr, FIRE/Coast/Barista (la route existe déjà)
│   ├── retirement.rs    # PER, rente, capitalisation fiscale FR
│   ├── tax.rs           # PFU 30 %, abattement PEA > 5 ans, IR/IFU — fonctions pures
│   ├── xirr.rs          # TRI pondéré par flux (Newton + bissection de secours)
│   └── networth.rs      # agrégation patrimoine, allocation, drawdown
├── ledger/              # NOUVEAU — comptes, transactions, catégorisation
│   ├── mod.rs · accounts.rs · transactions.rs
│   └── rules.rs         # moteur de règles de catégorisation (cf. §9)
├── budgets/             # enveloppes mensuelles, report (rollover), objectifs
├── subscriptions/       # abonnements (v0.3) — détection récurrences, prochain prélèvement
├── debts/               # prêts/dettes suivis, stratégies boule de neige / avalanche
├── goals/               # objectifs (apport, matelas) reliés à « la courbe »
├── import/              # NOUVEAU — cf. §6
│   ├── mod.rs           # trait Importer + ImportReport (doublons, mappés, ignorés)
│   ├── csv.rs · ofx.rs · qif.rs · json.rs
│   ├── mapping.rs       # ColumnMapping, profils de courtiers FR (sérialisés)
│   └── dedup.rs         # empreinte de transaction → détection de doublons
├── export/              # JSON complet (sauvegarde), CSV, PDF/HTML (rapports)
├── quotes/              # EXISTANT (placeholder) — cours OPTIONNELS (cf. §10)
│   ├── mod.rs           # trait QuoteProvider, cache, kill-switch
│   └── providers/       # stooq.rs, yahoo_unofficial.rs, ecb_fx.rs, coingecko.rs
├── crypto/              # EXISTANT (placeholder) — chiffrement base (cf. §4)
│   ├── mod.rs · kdf.rs (argon2) · cipher.rs (aes-gcm) · keyring.rs
├── alerts/              # EXISTANT (placeholder) — renouvellements, seuils (v0.3)
├── reports/             # NOUVEAU — rapports patrimoniaux, IFU-aide, bilan annuel
├── backup/              # NOUVEAU — snapshot atomique, restauration, vérif intégrité
├── sync/                # NOUVEAU — synchro E2E optionnelle (cf. §8)
├── plugins/             # NOUVEAU — registre d'extensions (cf. §9)
└── commands/            # EXISTANT — frontière IPC, un sous-module par domaine
    ├── mod.rs · simulate.rs · portfolio.rs · settings.rs
    └── ledger.rs · budgets.rs · import.rs · quotes.rs · backup.rs · sync.rs …
```

Conventions imposées (déjà respectées par le code existant, on les formalise) :
- **Moteurs purs** dans `finance/` : aucune dépendance Tauri/I/O, testés contre valeurs de référence. C'est ce qui permet le **fallback TS** (`sim-fallback.ts`) — on garde ce miroir pour les seuls calculs *purs* (simulateurs), pas pour la persistance.
- **Logique SQL en fonctions libres `q_*(conn, …)`** (comme `q_list`/`q_insert` actuels) → testables avec une base temporaire, sans `State<Db>`.
- **Commandes = enveloppes minces** : verrou (`db.0.lock().unwrap_or_else(|p| p.into_inner())` — pattern anti-poison déjà en place), assainissement des entrées (comme `finite`/`amount`/`years`/`rate` dans `simulate.rs`), `map_err(|e| e.to_string())` → migré vers `AppError` (cf. §2).
- **Sérialisation `#[serde(rename_all = "camelCase")]`** systématique (déjà la norme) pour coller aux interfaces TS.

> Comment ça reste simple : l'utilisateur ne voit jamais ces modules. Le **registre central** (`invoke_handler` dans `lib.rs`) est la seule liste à maintenir ; chaque domaine ajoute ses commandes là et n'a aucun couplage avec les autres. Un contributeur peut implémenter un importeur sans toucher au reste.

---

### 2. Gestion d'erreurs typée (`error.rs`)

Le code actuel fait `.map_err(|e| e.to_string())` — ça marche mais perd la structure. On introduit un `AppError` unique, sérialisable, pour des messages stables côté front (et de la traduction propre).

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("base de données: {0}")] Db(#[from] rusqlite::Error),
    #[error("entrée invalide: {0}")] Validation(String),
    #[error("import: {0}")] Import(String),
    #[error("verrouillé")] Locked,            // base chiffrée non déverrouillée
    #[error("réseau désactivé")] NetworkDisabled,
    #[error("E/S: {0}")] Io(#[from] std::io::Error),
}
impl serde::Serialize for AppError {            // {code, message} → i18n côté front
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let code = match self { Self::Db(_) => "db", Self::Validation(_) => "validation",
            Self::Import(_) => "import", Self::Locked => "locked",
            Self::NetworkDisabled => "network_disabled", Self::Io(_) => "io" };
        let mut m = s.serialize_map(Some(2))?;
        m.serialize_entry("code", code)?; m.serialize_entry("message", &self.to_string())?;
        m.end()
    }
}
pub type AppResult<T> = Result<T, AppError>;
```

> Reste simple : les commandes passent de `Result<_, String>` à `AppResult<_>` sans changer leur corps (le `?` propage via `#[from]`). Le front mappe `code` → message traduit ; pas de chaîne Rust brute affichée.

---

### 3. Modèle de données étendu + migrations

On **conserve** le schéma existant (`settings`, `accounts`, `holdings`, `transactions`, `dividends`, `subscriptions`, `simulations`) et on l'enrichit par **migrations idempotentes pilotées par `user_version`** — exactement le mécanisme déjà en place (`migrate()`, v1→v2). Règle d'or : **on n'édite jamais une migration passée**, on ajoute un palier.

Schéma cible (colonnes en français, cohérent avec l'existant) :

| Table | Rôle | Points clés |
|---|---|---|
| `accounts` (existe) | comptes : PEA, CTO, AV, PER, livret, compte courant, wallet | + `devise`, `institution`, `archive`, `position` (ordre d'affichage) |
| `transactions` (existe) | mouvements de cash & titres | + `categorie_id`, `note`, `pointee` (rapprochement), `import_id`, `empreinte` (dedup), `contrepartie_id` (virements internes) |
| `holdings` (existe) | positions valorisées | déjà `nom`/`prix_actuel` (v2) ; + `isin`, `marche`, `prix_maj_le` |
| `dividends` (existe) | coupons/dividendes | + `devise`, `retenue_source`, `reinvesti` |
| `categories` (NOUVEAU) | arbre de catégories (parent_id) | `type` (dépense/revenu/transfert), `couleur`, `icone` |
| `tags` + `transaction_tags` (NOUVEAU) | étiquettes libres N-N | recherche/filtres puissants sans rigidifier |
| `budgets` (NOUVEAU) | enveloppe par catégorie/mois | `montant`, `rollover` (report), `periode` |
| `goals` (NOUVEAU) | objectifs | `cible`, `echeance`, `compte_id`, lié à « la courbe » |
| `debts` (NOUVEAU) | dettes suivies | `principal`, `taux`, `mensualite`, `strategie` |
| `subscriptions` (existe) | abonnements | + `mode_paiement`, `compte_id`, `rappel_jours` |
| `attachments` (NOUVEAU) | pièces jointes | **BLOB chiffré** ou chemin ; `sha256`, `entity_type`/`entity_id` polymorphe |
| `net_worth_snapshots` (NOUVEAU) | photos de patrimoine | `date`, `total`, `detail_json` → historise « la courbe » du réel |
| `import_runs` (NOUVEAU) | journal d'imports | rejouable, annulable (rollback d'un import) |
| `rules` (NOUVEAU) | règles de catégorisation | `condition_json`, `action_json`, `priorite` |
| `prices_cache` (NOUVEAU) | cache de cours publics | `symbole`, `date`, `prix`, `source`, `expire_le` |
| `simulations` (existe) | sims sauvegardées | déjà `parametres_json` — extensible aux plugins |
| `settings` (existe) | clé/valeur | flags de fonctionnalités, préférences, version chiffrement |

`migrations.rs`, une fonction par palier, dans l'esprit du `migrate()` actuel :

```rust
pub fn migrate(conn: &Connection) -> AppResult<()> {
    let v: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if v < 3 { m3_categories_tags(conn)?; conn.pragma_update(None,"user_version",3)?; }
    if v < 4 { m4_budgets_goals(conn)?;   conn.pragma_update(None,"user_version",4)?; }
    if v < 5 { m5_snapshots_rules(conn)?; conn.pragma_update(None,"user_version",5)?; }
    // … chaque palier idempotent, testé sur une base au palier précédent
    Ok(())
}
```

Garde-fous de migration : **sauvegarde automatique du fichier `.db` avant tout palier** (copie horodatée dans `backups/`), exécution dans une transaction (`BEGIN`/`COMMIT`), test dédié « ouvrir une base v(n) → migrer → vérifier v(n+1) » (le test `open_creates_schema_and_settings_roundtrip` existant sert de modèle). `PRAGMA foreign_keys=ON` + `journal_mode=WAL` restent appliqués à l'ouverture (déjà le cas). Index ajoutés en même temps que les colonnes filtrées (ex. `transactions(date)`, `transactions(categorie_id)`, `transactions(empreinte)` unique pour le dedup).

> Comment ça reste simple : l'utilisateur lambda n'utilise que `accounts`/`holdings`/`transactions`. Les catégories/tags/budgets/objectifs sont **proposés mais facultatifs** ; une transaction sans catégorie est parfaitement valide. La richesse du schéma ne se traduit en UI que quand on l'active.

---

### 4. Sécurité & vie privée

**Local-first non négociable** : aujourd'hui la base est en clair dans `app_data_dir()`. On ajoute le **chiffrement optionnel** prévu dans `crypto.rs`, sans casser le mode par défaut.

Deux approches, je recommande **SQLCipher** (chiffrement transparent de toute la base, pages comprises) plutôt que du chiffrement applicatif champ par champ :
- `rusqlite` avec la feature `bundled-sqlcipher` (remplace `bundled`). La clé de page est dérivée du mot de passe maître.
- **KDF** : `argon2` (Argon2id, paramètres calibrés ~250–500 ms) sur le mot de passe maître → clé de 32 octets passée via `PRAGMA key`. Sel stocké en clair dans un fichier voisin `pecule.salt` (le sel n'est pas secret).
- **Keyring OS** (`keyring` crate → Keychain macOS / DPAPI Windows / Secret Service Linux) pour stocker **optionnellement** la clé dérivée afin d'éviter de retaper le mot de passe à chaque lancement (option « se souvenir sur cet appareil », désactivable).
- Si l'utilisateur ne veut PAS de keyring : la clé ne vit qu'en mémoire (zeroize) le temps de la session.

```rust
// crypto/mod.rs — esquisse
pub fn open_encrypted(path: &Path, master: &SecretString) -> AppResult<Connection> {
    let salt = load_or_create_salt(path)?;            // fichier .salt voisin
    let key = argon2id_derive(master, &salt, PARAMS)?; // [u8; 32], type Zeroizing
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "key", hex_key(&key))?;   // SQLCipher
    conn.pragma_update(None, "cipher_memory_security", "ON")?;
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))  // valide la clé
        .map_err(|_| AppError::Locked)?;
    Ok(conn)
}
```

Migration **clair → chiffré** (et inverse) : `sqlcipher_export()` vers une nouvelle base, swap atomique. L'utilisateur active le chiffrement depuis Réglages → l'app re-chiffre en place avec sauvegarde préalable.

**Verrouillage auto** : un état `AppLock { unlocked_until: Instant, idle_timeout }`. Au-delà du délai d'inactivité (réglable, défaut 15 min) ou à la mise en veille, on **drop la connexion** et on efface la clé (zeroize) → toute commande renvoie `AppError::Locked` jusqu'au redéverrouillage. Raccourci ⌘L pour verrouiller à la demande.

**CSP** : déjà stricte dans `tauri.conf.json` (`default-src 'self'`, pas de `connect-src` externe). Durcissements : retirer `'unsafe-inline'` de `script-src` (passer aux nonces SvelteKit) ; garder `connect-src 'self' ipc:` — **aucun domaine externe** par défaut. Quand les cours sont activés, on **n'élargit pas la CSP du webview** : les requêtes HTTP sortantes partent du **process Rust** (réseau côté natif), pas du webview, donc la CSP reste fermée et le front ne peut jamais initier un appel externe.

**Permissions Tauri minimales** (`capabilities/default.json`) : aujourd'hui `core:default` + `core:window:allow-start-dragging`. On reste sur le principe du moindre privilège — n'ajouter une permission que quand une fonctionnalité l'exige (ex. `dialog:allow-open` pour l'import, `fs` scopé au seul dossier de sauvegarde, `notification` pour les alertes v0.3). Pas de plugin `shell`, pas de `http` côté webview. Permissions documentées une par une.

**Vie privée par construction** : zéro télémétrie, zéro pub, zéro compte (déjà le cas). Le mode hors-ligne est l'état par défaut ; toute capacité réseau est un *opt-in* explicite, visible, réversible, avec un voyant d'état réseau dans l'UI.

> Reste simple : par défaut, **aucun mot de passe** (base en clair, comme aujourd'hui) — on n'impose pas la friction du chiffrement. L'utilisateur qui le veut l'active en un réglage ; ceux qui ne savent pas ce qu'est argon2 n'en entendent jamais parler.

---

### 5. Import / export

Architecture autour d'un **trait `Importer`** pour brancher des formats et des profils de courtiers (point d'extension communautaire majeur, marché FR).

```rust
// import/mod.rs
pub struct ParsedTx { pub date: NaiveDate, pub libelle: String, pub montant: f64,
    pub devise: String, pub categorie_suggeree: Option<String>, pub empreinte: String }
pub struct ImportReport { pub ajoutees: usize, pub doublons: usize,
    pub ignorees: usize, pub avertissements: Vec<String> }

pub trait Importer {
    fn detect(&self, sample: &str) -> bool;            // sniff d'en-tête
    fn parse(&self, raw: &str, map: &ColumnMapping) -> AppResult<Vec<ParsedTx>>;
}
```

- **Formats** : CSV (le plus courant en FR), **OFX** et **QIF** (banques), **JSON** (notre propre format = sauvegarde/restauration round-trip). Crates : `csv`, `quick-xml` (OFX), parseur QIF maison léger.
- **Mapping de colonnes** (`mapping.rs`) : l'utilisateur associe colonnes → champs (date, libellé, débit/crédit, montant). Auto-détection des en-têtes courants ; le mapping validé est **sauvegardable comme profil** (`settings`/fichier) et **partageable** (profils de courtiers FR : Boursorama, Crédit Agricole, Fortuneo, Trade Republic, Bourse Direct, Degiro… contribués par la communauté en JSON versionné dans le dépôt).
- **Détection de doublons** (`dedup.rs`) : `empreinte = sha256(date | montant | libellé normalisé | compte)`, contrainte `UNIQUE` sur `transactions.empreinte`. `INSERT … ON CONFLICT DO NOTHING` → un ré-import du même relevé n'ajoute rien.
- **Import rejouable/annulable** : chaque run logué dans `import_runs` ; un bouton « annuler cet import » supprime les transactions de ce `import_id` (transaction SQL).
- **Export** : JSON complet (toutes tables → **sauvegarde portable et lisible**, déjà l'esprit de l'export JSON v0.2 du portefeuille), CSV par table, **rapports PDF/HTML** générés côté Rust (gabarit HTML → impression headless ou crate `printpdf`). L'export ne sort **jamais** vers le réseau : c'est un fichier que l'utilisateur place où il veut.

> Reste simple : flux d'import en **3 écrans** — « choisis ton fichier » → « on a reconnu Boursorama, voici l'aperçu » (mapping pré-rempli) → « 142 ajoutées, 8 doublons ignorés ». Le mapping manuel ne s'affiche que si l'auto-détection échoue (divulgation progressive).

---

### 6. Cours publics OPTIONNELS (§11)

Le placeholder `quotes.rs` devient un sous-système strictement opt-in, avec une **abstraction de fournisseur**, un **cache**, et un principe absolu : **les positions de l'utilisateur ne sortent jamais**.

```rust
// quotes/mod.rs
pub trait QuoteProvider {
    fn name(&self) -> &'static str;
    async fn fetch(&self, symbole: &str) -> AppResult<Quote>;       // un symbole public à la fois
}
pub struct Quote { pub symbole: String, pub prix: f64, pub devise: String, pub date: NaiveDate }
```

- **Kill-switch global** : setting `quotes.enabled` (défaut `false`). Tant qu'il est faux, le module n'effectue **aucune** requête réseau — c'est l'état d'usine. Voyant réseau « hors-ligne » dans l'UI.
- **Garantie de fuite zéro** : on envoie un **symbole public** (ISIN/ticker) au fournisseur, **jamais** quantité, PRU, compte, ni valeur. Le code est structuré pour que la requête ne reçoive qu'un `&str` symbole — aucun chemin ne permet de joindre les données privées.
- **Cache** (`prices_cache`) : un cours est réutilisé tant que `expire_le` > maintenant (défaut 24 h). On regroupe les symboles distincts, on respecte un budget de requêtes (anti-rate-limit). Une seule actualisation manuelle par session par défaut.
- **Fournisseurs branchables** : `ecb_fx` (taux de change BCE, source officielle EU), `stooq`, Yahoo non-officiel, CoinGecko (crypto). L'utilisateur choisit la source ; chaque fournisseur déclare ses limites. Réseau **côté Rust uniquement** (CSP du webview inchangée).

> Reste simple : un bouton « Actualiser les cours » désactivé par défaut, avec une explication d'une ligne sur ce qui sort (le ticker, rien d'autre). Qui ne l'active pas garde la valorisation manuelle de la v0.2.

---

### 7. Sauvegarde / restauration & synchronisation E2E OPTIONNELLE

**Sauvegarde** (`backup/`) : snapshot atomique de la base via **`VACUUM INTO 'chemin'`** (cohérent même en WAL, sans verrouiller longuement), nom horodaté, dans un dossier choisi par l'utilisateur (ou cloud perso monté comme dossier local). Restauration = sélection d'un fichier + remplacement atomique après vérif d'intégrité (`PRAGMA integrity_check`). Sauvegarde **auto avant chaque migration** (cf. §3). Conservation glissante (garder N dernières).

**Synchronisation chiffrée de bout en bout, OPTIONNELLE, sans serveur Pécule** (`sync/`) :
- Modèle : **fichier chiffré** posé dans un dossier que l'utilisateur synchronise lui-même (iCloud Drive, Dropbox, Nextcloud, partage réseau local). Pécule n'a **aucun backend** — il lit/écrit un blob, point.
- **Chiffrement E2E** : le blob est chiffré avec une clé dérivée d'une passphrase de sync (argon2 + aes-gcm, même brique que `crypto/`). Le fournisseur cloud ne voit que du chiffré.
- **Détection de conflits** : journal d'événements append-only par appareil (CRDT léger ou last-write-wins par champ avec horloge logique). À défaut, stratégie sûre : verrou coopératif + « cette version vient de l'appareil X plus récent, garder/comparer ». Pas de fusion automatique destructive.
- **Réseau local** : option LAN (mDNS découverte + transfert chiffré direct entre deux instances Pécule), zéro cloud.

> Reste simple : « Sauvegarder » et « Restaurer » sont deux boutons. La sync est un onglet caché derrière « Avancé », avec un seul choix par défaut « un fichier que tu synchronises toi-même ». L'utilisateur qui ne veut rien sait que tout est déjà sur sa machine.

---

### 8. Système d'extensions / plugins & règles

Deux niveaux d'extensibilité, du plus sûr au plus puissant :

**(a) Extensions déclaratives (données, zéro code) — par défaut, sûres** :
- **Profils d'import** (mapping courtiers) : fichiers JSON versionnés, contribués par PR. Aucun code exécuté.
- **Règles de catégorisation** (`rules`, moteur dans `ledger/rules.rs`) : `si libellé contient "SNCF" alors catégorie=Transport` ; `condition_json`/`action_json`, appliquées par priorité après chaque import. Éditables dans l'UI, exportables/importables.
- **Modèles de simulateurs/rapports** : un simulateur = un schéma de paramètres + une formule. Les formules « plugin » s'expriment en déclaratif (DSL bornée) ou via les moteurs purs existants paramétrés — pas d'exécution de code arbitraire dans la version sûre.

**(b) Plugins « code » (avancé, opt-in, isolé)** : pour aller plus loin (importeur exotique, fournisseur de cours, rapport custom), un registre `plugins/` avec un **manifeste** (`plugin.json` : nom, version, capacités demandées, hash signé). Exécution **sandboxée WebAssembly** (`wasmtime`) : le plugin reçoit des entrées sérialisées, renvoie des sorties sérialisées, **sans accès** au système de fichiers ni au réseau ni à la base — seulement aux données qu'on lui passe explicitement (capability-based). Un importeur WASM voit le texte du fichier, pas le disque ; un fournisseur de cours WASM ne fait pas de réseau lui-même, il demande à l'hôte (qui applique le kill-switch §6).

```rust
// plugins/mod.rs
pub enum PluginKind { Importer, QuoteProvider, Report, Simulator }
pub struct PluginManifest { pub id: String, pub version: String,
    pub kind: PluginKind, pub capabilities: Vec<Capability>, pub wasm_sha256: String }
```

> Reste simple : 99 % des utilisateurs n'installent jamais un plugin code — les règles et profils déclaratifs couvrent l'essentiel et sont sans risque. Les plugins WASM sont derrière « Extensions (avancé) », signés, avec capacités affichées avant installation (« ce plugin peut : lire les fichiers que tu importes »).

---

### 9. Performance

- **Calcul lourd en Rust**, pur et déterministe (déjà le cas pour les simulateurs ; on y ajoute XIRR, agrégation patrimoine, génération de séries). Les commandes renvoient la **série complète** pour que le curseur de « la courbe » bouge sans nouvel IPC — pattern déjà documenté dans `simulate.rs`.
- **Bornes anti-explosion** déjà présentes (`MAX_PROJECTION_MONTHS = 200·12`, `months_count` plafonné) → généralisées à tout générateur de série.
- **Gros historiques** : index SQL ciblés (date, catégorie, compte), requêtes paginées et **agrégations en SQL** (`SUM`/`GROUP BY`) plutôt que de remonter 50 000 lignes au front. Vues matérialisées légères pour les soldes courants. Prepared statements réutilisés.
- **Virtualisation des listes** côté Svelte (rendu fenêtré : seules les lignes visibles existent dans le DOM) pour les longues listes de transactions — la liste de portefeuille actuelle reste triviale, mais le ledger en aura besoin.
- **IPC efficace** : éviter les allers-retours en boucle ; une commande = un lot. Les réglages et listes sont chargés une fois et tenus dans les stores Svelte 5 (`.svelte.ts`, comme `theme.svelte.ts` existant).
- **Démarrage** : ouverture base + migrations < quelques ms (déjà le cas) ; chiffrement ajoute le coût argon2 (volontaire, ~300 ms) **uniquement si activé**.

> Reste simple : la performance est invisible — l'app reste instantanée même à 100 k transactions, sans que l'utilisateur ait à archiver ou nettoyer quoi que ce soit.

---

### 10. Tests, CI multi-OS, mises à jour signées, distribution

**Tests** (la base est déjà bonne : tests `#[cfg(test)]` dans `db.rs`, `portfolio.rs`, `finance/mod.rs`) :
- **`cargo test`** : moteurs `finance/` contre **valeurs de référence** ; fonctions `q_*` contre base temporaire (pattern `crud_roundtrip` existant) ; **un test de migration par palier** (ouvrir base v(n) → migrer → vérifier structure & données). Tests des assainisseurs (NaN/∞/négatifs).
- **`vitest`** : pont IPC TS (`num()`, fallbacks `sim-fallback`), formatage (`format.ts`), i18n, stores.
- **`playwright`** : parcours bout-en-bout dans le webview (ajout d'une position, import d'un CSV, déplacement du curseur de la courbe). Tauri expose un mode test/driver pour piloter la fenêtre.
- **Property tests** (`proptest`) sur les invariants financiers (ex. `interest == value - contributed`, monotonie de la valorisation à taux positif).

**CI multi-OS** (GitHub Actions, matrice macOS/Windows/Linux) :
- `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- `npm run check` (svelte-check, déjà câblé), `vitest`, `playwright`.
- Build Tauri sur les 3 OS, artefacts de release.

**Mises à jour signées** : plugin `tauri-plugin-updater` avec **signature Ed25519** — l'app ne fait confiance qu'aux binaires signés par notre clé privée ; le manifeste de mise à jour est servi statiquement (GitHub Releases). Vérification cryptographique avant installation. **Désactivable** (les paquets gérés — brew/winget/flatpak — se mettent à jour par leur gestionnaire, et là on coupe l'updater intégré).

**Distribution** (`bundle.targets: "all"` déjà configuré) :
- **macOS** : `.dmg` signé + **notarisé** Apple ; canal **Homebrew Cask**.
- **Windows** : `.msi`/NSIS signé (certificat de signature de code) ; **winget**.
- **Linux** : **AppImage** (portable) + **Flatpak** (Flathub) ; éventuellement `.deb`/`.rpm`.
- Distribution **open source / MIT** : le dépôt et les recettes de build sont publics, builds reproductibles visées.

> Reste simple côté utilisateur : il installe par son moyen habituel (dmg, store, brew/winget/flatpak), les mises à jour arrivent par le canal attendu, signées. Aucun compte, aucune activation.

---

### 11. Frontend — vues, composants, stores (SvelteKit / Svelte 5)

L'arbo existant est sain et reste la fondation :

```
src/
├── routes/                       # une vue = un dossier (SvelteKit, adapter-static)
│   ├── +layout.svelte (Sidebar) · +page.svelte (accueil)
│   ├── simulateurs/{,interets-composes,epargne,pret,fire}
│   ├── portefeuille/ · abonnements/ · reglages/
│   ├── transactions/ · budgets/ · objectifs/ · dettes/   # NOUVEAU
│   └── patrimoine/ (snapshots + « la courbe » du réel)   # NOUVEAU
├── lib/
│   ├── ipc.ts                    # EXISTANT — pont typé + fallback (étendu par domaine)
│   ├── sim-fallback.ts           # moteur TS miroir (calculs PURS uniquement)
│   ├── format.ts · theme.svelte.ts
│   ├── i18n/ (state.svelte.ts, messages.ts)              # FR par défaut, extensible
│   ├── stores/  (NOUVEAU)        # holdings.svelte.ts, ledger.svelte.ts, lock.svelte.ts…
│   └── components/               # Curve, Card, Stat, Field, AnimatedNumber, EmptyState…
│       └── CommandPalette.svelte (NOUVEAU — ⌘K)
```

Principes front : 
- **Pont IPC unique** (`ipc.ts`) = seule porte vers Rust, typé, qui **assainit avant l'appel** (déjà fait : `num()`), avec **fallback navigateur** pour le mode preview. On ajoute un sous-bloc par domaine (ledger, budgets, import…) sur le même modèle que le bloc portefeuille actuel.
- **Stores Svelte 5 runes** (`.svelte.ts`, comme `theme.svelte.ts`/i18n existants) pour l'état partagé : positions, transactions, état de verrouillage, état réseau. Source de vérité = Rust ; le store est un cache rafraîchi après mutation.
- **« La courbe » comme composant réutilisable** (`Curve.svelte` existe) : alimentée par les `SeriesPoint[]` des moteurs ET par les `net_worth_snapshots` réels → même visuel pour projection et historique. Curseur « et dans X années ? » sans IPC (série complète déjà en main).
- **Palette ⌘K** : navigation, actions (ajouter une transaction, importer, verrouiller), recherche — c'est le vecteur de la « puissance à la demande » : tout est atteignable au clavier sans encombrer l'écran.
- **i18n** : FR par défaut (déjà en place), structure prête pour d'autres langues ; les `code` d'`AppError` se traduisent côté front.

> Reste simple : la barre latérale ne montre que les vues activées ; les fonctionnalités avancées vivent dans la palette ⌘K et derrière des sections « Avancé » des Réglages. Bons défauts partout (devise EUR, thème système, pas de chiffrement, pas de réseau). Divulgation progressive : on commence comme la v0.2, on déploie la puissance quand on la cherche.

---

### Fichiers de référence (chemins absolus)

- Registre IPC / setup : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/lib.rs`
- Base & migrations (à scinder en `db/`) : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/db.rs`
- Pattern persistance `q_*` testable : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/commands/portfolio.rs`
- Assainissement à la frontière IPC : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/commands/simulate.rs`
- Moteurs purs + bornes : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/finance/mod.rs`
- Placeholders à implémenter : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/{crypto.rs,quotes.rs,alerts.rs}`
- Pont IPC typé + fallback : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src/lib/ipc.ts`
- CSP & fenêtre : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/tauri.conf.json`
- Permissions minimales : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/capabilities/default.json`
- Dépendances Rust (à étendre : sqlcipher, argon2, aes-gcm, keyring, thiserror, csv, quick-xml, wasmtime, tauri-plugin-updater) : `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/Cargo.toml`

---

## 4. Rigueur du domaine financier & spécificités FR

L'argument de vente de Pécule, c'est la confiance : une app local-first qui ne ment pas sur les chiffres. La rigueur n'est pas un module de plus, c'est le socle. Cette lentille détaille comment garantir des calculs **justes, honnêtes et reproductibles**, comment les rendre **simples par défaut** (le bon chiffre s'affiche tout seul, la nuance est à la demande), et comment se brancher sur le code existant (`src-tauri/src/finance/`, table `transactions` déjà présente mais inexploitée, PRU plat actuel dans `holdings`).

### Principe transversal : le « contrat d'exactitude »

Trois règles non négociables, à graver dans `finance/mod.rs` comme conventions partagées :

1. **Calcul en pleine précision, arrondi à l'affichage uniquement.** Déjà respecté (cf. doc de `mod.rs` : « L'arrondi à deux décimales se fait à l'affichage, jamais dans les moteurs »). À étendre : ne jamais ressaisir un chiffre arrondi dans un calcul suivant (pas de PRU arrondi réinjecté dans une +/- value).
2. **Toute hypothèse est explicite et visible.** Chaque chiffre dérivé porte une méthode nommée (« TWR », « FIFO », « taux PFU 30 % »), affichable d'un survol. Zéro hypothèse cachée.
3. **Honnêteté > optimisme.** Quand une donnée manque (taux de change historique, frais d'une vieille transaction), on affiche « estimation » et on dégrade explicitement, jamais une fausse précision.

**Comment ça reste simple :** l'utilisateur voit un seul chiffre « propre » (ex. « +12,4 % cette année »). La méthode et les hypothèses sont dans une infobulle « (i) » et un panneau « Détail du calcul » repliable. Défaut intelligent, profondeur à la demande.

---

### 1. Mesures de performance : rendement simple vs TWR vs MWR/XIRR

C'est le piège n°1 des apps de finance perso : afficher un « rendement » sans dire lequel. Trois mesures, trois usages.

| Mesure | Question à laquelle elle répond | Quand l'utiliser | Sensible aux dépôts/retraits ? |
|---|---|---|---|
| **Rendement simple (ROI)** | « Combien ai-je gagné sur ce que j'ai mis ? » | Une position figée, sans flux intermédiaires | Oui (faux dès qu'on ajoute des versements) |
| **TWR** (Time-Weighted Return) | « Mes choix d'investissement étaient-ils bons ? » | Comparer à un indice, juger une stratégie | **Non** (neutralise le timing des versements) |
| **MWR / XIRR** (Money-Weighted) | « Quel rendement *moi* j'ai réellement obtenu ? » | Mesurer son expérience réelle, avec le DCA | **Oui** (récompense/pénalise le timing) |

**Formules clés.**

- **Rendement simple :** `ROI = (valeur_actuelle − coût_total) / coût_total`. C'est ce que calcule déjà `portefeuille/+page.svelte` (`gainPct = gain / invested`). **Correct uniquement sans flux intermédiaires** ; trompeur dès qu'il y a des versements échelonnés (un gros versement récent gonfle artificiellement le dénominateur).

- **TWR :** on découpe la période en sous-périodes bornées par chaque flux (dépôt/retrait), on calcule le rendement de chaque sous-période *avant* le flux, puis on **chaîne géométriquement** :
  `TWR = ∏ (1 + r_k) − 1`, avec `r_k = (V_fin,k − V_début,k − flux_k) / V_début,k` calculé de façon à isoler le flux. Le TWR neutralise l'effet du *montant* et du *moment* des versements → c'est la mesure comparable à un benchmark (CAC 40, MSCI World).

- **MWR / XIRR :** c'est le **TRI (taux de rendement interne)** des flux datés. On résout le taux `r` annualisé qui annule la valeur actuelle nette :
  `Σ_j CF_j / (1 + r)^((d_j − d_0)/365) = 0`,
  où `CF_j` sont les flux datés (versements négatifs = sorties de poche, valeur finale positive). Résolution numérique : **Newton-Raphson** avec repli sur **bissection** (Newton seul peut diverger si les flux changent de signe plusieurs fois). Toujours borner et détecter la non-convergence → afficher « non calculable » plutôt qu'un chiffre faux.

**Pièges à coder explicitement.**
- **XIRR multi-solutions / divergence :** flux alternés peuvent donner plusieurs racines. Garde-fou : intervalle de recherche [−99,9 % ; +1000 %], max 100 itérations, repli bissection, sinon `None`.
- **Annualisation d'une période < 1 an :** ne **jamais** extrapoler « +3 % en 2 mois → +18 %/an » par défaut. Afficher la perf brute de la période et proposer l'annualisation comme option explicitement étiquetée « annualisé ».
- **Sous-période à valeur de départ nulle** (premier versement) : exclure du chaînage TWR, pas de division par zéro.
- **TWR sans historique de valeurs intermédiaires :** Pécule valorise manuellement (pas de cours quotidiens). Le TWR exact exige une valeur à chaque flux → si l'utilisateur n'a pas saisi ces valeurs, **on ne propose pas le TWR**, on propose XIRR (qui ne nécessite que les flux datés + valeur actuelle). Honnêteté sur la limite.

**Comment ça reste simple :** par défaut, sur une position **sans flux multiples**, on montre le ROI simple (familier, « +15 % »). Dès qu'il y a plusieurs versements datés, l'app **bascule automatiquement** sur XIRR et affiche « rendement annualisé (méthode XIRR) ». Un sélecteur discret « Mesure de performance : Simple · TWR · XIRR » n'apparaît que dans le panneau avancé. Le bon défaut, pas de choix imposé au débutant.

> Prérequis technique : exploiter la table `transactions(sens, quantite, prix, frais, date)` déjà présente dans `db.rs` mais aujourd'hui non utilisée. C'est elle qui fournit les flux datés indispensables au XIRR. Migration : la valorisation actuelle (PRU plat dans `holdings`) reste le mode « simple » ; le mode « transactions » débloque TWR/XIRR.

---

### 2. Base de coût : FIFO, CUMP, frais, opérations sur titres

Le portefeuille actuel stocke un **PRU unique** par ligne (`holdings.pru`) : c'est implicitement du **CUMP** figé, saisi à la main. Pour des +/- values justes (et fiscalement crédibles), il faut dériver la base de coût des transactions.

**CUMP (coût unitaire moyen pondéré) — le défaut français.**
`PRU = (Σ quantités_achetées × prix + frais d'achat) / Σ quantités_achetées`. À chaque achat, le PRU se recalcule ; à chaque vente, le PRU **ne bouge pas**, seule la quantité diminue. La +/- value d'une vente = `quantité_vendue × (prix_vente − PRU) − frais_vente`. C'est la convention par défaut de la quasi-totalité des courtiers et de l'administration fiscale française (CTO).

**FIFO (premier entré, premier sorti).**
On épuise les lots dans l'ordre chronologique d'achat. Utile pour : compatibilité avec certaines juridictions, crypto (où le suivi par lot est répandu), et analyse fine. La +/- value dépend du lot consommé.

> **Règle fiscale française importante (garde-fou anti-erreur) :** pour les **plus-values mobilières sur CTO**, le fisc impose le **prix moyen pondéré d'acquisition** (≈ CUMP), **pas** le FIFO. Pour les **cryptos**, c'est une méthode spécifique (prix total d'acquisition × prix de cession / valeur globale du portefeuille — quasi-CUMP global). **Conséquence produit : le défaut DOIT être CUMP**, FIFO est une option d'analyse, jamais le calcul fiscal par défaut. Étiqueter clairement.

**Impact des frais.**
- Frais d'**achat** → **augmentent** la base de coût (donc réduisent la +value imposable). Le champ `transactions.frais` existe déjà.
- Frais de **vente** → **réduisent** le produit de cession.
- Ne jamais ignorer les frais « parce que c'est petit » : sur du DCA mensuel ils s'accumulent et faussent le PRU.

**Opérations sur titres (corporate actions).**
- **Split / regroupement (ex. 1 action → 4) :** multiplier la quantité par le ratio, **diviser le PRU** par le même ratio. Le coût total et la valeur sont invariants. Indispensable sinon le PRU devient absurde (PRU historique > prix actuel par 4).
- **Dividende en actions (stock dividend) :** ajoute des quantités à coût nul (ou répartit la base selon la règle applicable) → **baisse le PRU**. Le distinguer du dividende en cash.
- **Attribution gratuite / scrip :** traitement analogue au stock dividend.
- Ces opérations sont des **événements datés** : prévoir une table `corporate_actions(holding_id, type, ratio, date)` et les rejouer dans l'ordre lors du calcul de la base de coût.

**Comment ça reste simple :** l'utilisateur saisit ses achats/ventes normalement ; le PRU, la +/- value et la base fiscale se calculent **automatiquement en CUMP**. Le mode FIFO et le détail lot-par-lot sont dans un onglet « Avancé ». Les splits se déclarent via un assistant « Ajuster pour un split » (saisir le ratio, l'app fait le reste) plutôt qu'en éditant des chiffres à la main.

---

### 3. Dividendes/coupons, fiscalité à la source, FX, arrondis

**Dividendes & coupons.**
- Distinguer **dividende brut** et **net perçu** (après prélèvement). La table `dividends(montant, date)` existe ; lui ajouter `brut`, `retenue_source`, `devise`.
- Choix de méthode de rendement à exposer clairement : **price return** (hors dividendes) vs **total return** (dividendes réinvestis). Le total return est la mesure honnête de ce qu'a rapporté l'actif. Défaut : afficher les deux quand des dividendes existent.
- **Réinvestissement (DRIP) :** un dividende réinvesti devient un achat à coût = montant net → met à jour le PRU. Cohérence avec §2.

**Fiscalité à la source (withholding tax).**
- Dividendes étrangers : retenue prélevée dans le pays source (ex. 15 % US sous convention). Stocker le montant retenu ; il peut ouvrir droit à un **crédit d'impôt** en France. Pécule **n'optimise pas l'impôt** : il **enregistre et affiche** la retenue, avec mention « repère pédagogique ».

**FX — taux de change historiques.**
- Toute valeur en devise étrangère doit être convertie au **taux à la date de l'opération** pour la base de coût, et au **taux courant** pour la valorisation. Mélanger les deux est l'erreur classique.
- Conséquence : il faut une **table de taux historiques** `fx_rates(devise, date, taux)`. En local-first sans API par défaut : permettre la **saisie manuelle** du taux à l'achat, et un import optionnel de séries de taux (BCE, fichier CSV). Quand le taux historique manque → afficher « valorisé au taux du jour, base de coût approximée » (honnêteté §0).
- Le champ `holdings.devise` / `transactions` (à enrichir) existe déjà ; aujourd'hui aucune conversion n'est faite → c'est un bug de justesse latent à corriger dès qu'on dépasse l'euro.

**Arrondis.**
- Calcul en `f64` pleine précision (déjà le cas), arrondi **à l'affichage**. Pour les **montants monétaires affichés et agrégés fiscalement**, arrondir au **centime** par la règle « arrondi commercial » (half-up), et **sommer les valeurs non arrondies puis arrondir le total** (jamais sommer des valeurs déjà arrondies → écarts de 1 centime qui font douter).
- Cas connu et bien géré dans `loan.rs` : la dernière mensualité absorbe l'arrondi résiduel pour solder *exactement* le capital (`balance = 0.0`). Généraliser ce pattern « la dernière ligne solde » partout où on amortit/répartit.

**Comment ça reste simple :** saisie d'un dividende = un montant + une date, l'app propose le net/brut et la devise avec des défauts (EUR, pas de retenue). Le FX manuel n'apparaît que si la devise diffère de la devise de référence.

---

### 4. Prêts : assurance, remboursement anticipé, TAEG vs taux nominal

`loan.rs` est déjà solide (mensualité constante, amortissement français exact, assurance sur capital initial, dernière ligne qui solde). Points à compléter pour la rigueur :

**Assurance : capital initial vs capital restant dû.**
- `loan.rs` applique aujourd'hui une prime **constante sur le capital initial** : `principal × insurance_annual_rate / 12`. C'est la convention « **CI** » (capital initial), la plus répandue en France.
- Il manque la convention « **CRD** » (capital restant dû) : `prime_mois = balance × taux_assurance / 12`, prime **dégressive**. Sur un prêt long, CI et CRD diffèrent fortement (l'assurance CI coûte plus cher au global). Les rendre comparables côte à côte est un vrai service.
- **Action :** ajouter un champ `insurance_basis: { InitialCapital, OutstandingBalance }`. Recalculer la prime par ligne d'amortissement en mode CRD.

**Remboursement anticipé.**
- Modéliser : un remboursement partiel/total à une date donnée → réduit le capital restant dû → recalcul du tableau (soit mensualité réduite, soit durée réduite, au choix).
- **Indemnité de remboursement anticipé (IRA) :** plafond légal français = min(3 mois d'intérêts sur le capital remboursé ; 1 % du capital restant dû). À calculer et afficher. Repère pédagogique (des exceptions existent).

**TAEG vs taux nominal.**
- **Taux nominal :** sert à calculer les intérêts (`annual_rate` actuel).
- **TAEG (taux annuel effectif global) :** taux qui intègre **intérêts + assurance + frais de dossier + garantie**, c.-à-d. le coût réel. Il se calcule comme un **TRI des flux du prêt** (capital reçu vs ensemble des mensualités tout compris) → même solveur que le XIRR (§1, mutualiser le code).
- **Piège :** comparer deux offres sur le taux nominal seul est trompeur ; le TAEG est la seule comparaison honnête. Pécule doit **mettre le TAEG en avant**.
- Distinguer aussi **taux périodique** (mensuel = nominal/12 ici, convention proportionnelle française) vs **taux actuariel** ; documenter la convention retenue.

**Comment ça reste simple :** l'écran prêt montre par défaut **mensualité totale + TAEG + coût total**. CI/CRD, remboursement anticipé et IRA sont dans « Options avancées ». L'assistant « comparer deux prêts » fait le TAEG automatiquement.

---

### 5. Simulateurs : hypothèses explicites, inflation, taux effectif, prudence

Les trois moteurs (`compound`, `savings`, `loan`) sont purs, testés, et documentent déjà leurs hypothèses dans le code (ex. savings : « taux mensuel `i = annual_rate/12` nominal »). Il faut **remonter ces hypothèses à l'écran** et ajouter la dimension prudence.

**Taux nominal vs effectif (déjà au cœur du code).**
- `compound.rs` distingue bien `compounds_per_year` (n) : `FV = P·(1+r/n)^(n·t)`. Le **taux effectif annuel** correspondant est `TAEx = (1+r/n)^n − 1`. À afficher à côté du taux nominal saisi pour que l'utilisateur voie l'écart (5 % nominal mensuel → 5,12 % effectif).
- `savings.rs` utilise `i = annual_rate/12` (convention proportionnelle). C'est un **choix d'hypothèse** à étiqueter (« taux nominal, capitalisation mensuelle ») — défendable et courant, mais à rendre visible.

**Inflation : nominal vs réel.**
- Ajouter un paramètre « inflation annuelle » optionnel. Afficher le résultat en **euros d'aujourd'hui (réel)** en plus du nominal : `valeur_réelle = valeur_nominale / (1+infl)^t`, ou via le **taux réel de Fisher** : `(1+r_réel) = (1+r_nominal)/(1+inflation)` (≠ simple soustraction r − i, qui est une approximation). Sur 30 ans, l'écart nominal/réel est énorme → l'omettre serait malhonnête.

**Prudence & intervalles.**
- Un taux fixe de rendement est une **fiction pédagogique**. Proposer (en option) un mode **scénarios** : pessimiste / médian / optimiste (ex. 3 % / 5 % / 7 %) affichés comme une bande sur « la courbe ».
- Possibilité ultérieure : projection Monte-Carlo (distribution de résultats) — mais en restant lisible (« 80 % des scénarios entre X et Y »).
- Toujours rappeler : **les performances passées ne préjugent pas des performances futures** ; un rendement constant n'existe pas.

**Comment ça reste simple :** un seul champ « rendement attendu » avec un défaut raisonnable. L'inflation est un interrupteur « afficher en euros d'aujourd'hui ». Les scénarios sont un bouton « voir une fourchette » qui dessine la bande sur la courbe. Le débutant voit une courbe ; le curieux déplie les hypothèses (taux effectif, inflation, fourchette) sans jamais être noyé.

---

### 6. Fiscalité française pédagogique (repère, pas conseil)

Objectif : aider à **comprendre l'ordre de grandeur de l'impôt**, jamais se substituer à un conseiller. Chaque chiffre fiscal porte la mention « **repère pédagogique, pas un conseil fiscal** » (le composant `Disclaimer.svelte` existe déjà — le réutiliser systématiquement).

**Les enveloppes (repères 2025-2026, à dater et versionner dans le code).**

| Enveloppe | Règle clé (repère) | Calcul à proposer |
|---|---|---|
| **PEA** | Plafond **150 000 €** de versements. Après **5 ans** d'antériorité : gains exonérés d'IR (mais **prélèvements sociaux 17,2 %** dus). Avant 5 ans : retrait = clôture + imposition. | Suivre le total versé vs plafond ; afficher l'âge du plan et la date du seuil 5 ans. |
| **CTO** | **PFU / flat tax 30 %** = 12,8 % IR + 17,2 % PS sur la +value. Option barème progressif possible. | +value (base CUMP §2) × 30 % par défaut, avec note « option barème possible ». |
| **Assurance-vie** | Après **8 ans** : abattement annuel **4 600 € (seul) / 9 200 € (couple)** sur les gains. PS 17,2 %. Fiscalité des gains selon antériorité et date des versements (avant/après 27/09/2017). | Afficher antériorité du contrat, date du seuil 8 ans, abattement applicable. |
| **PER** | Versements **déductibles du revenu** (dans plafonds) ; imposition à la **sortie**. | Estimer l'économie d'impôt à l'entrée selon TMI (en repère). |
| **Plus-values mobilières** | Base = **prix moyen pondéré** (CUMP, cf. §2). | Calcul automatique depuis transactions. |
| **Prélèvements sociaux** | **17,2 %** (transversal : CTO, PEA après retrait, AV, intérêts). | Toujours afficher PS séparément de l'IR pour la pédagogie. |

**Garde-fous fiscaux indispensables.**
- **Versionner les taux et seuils** (`tax_rules` daté, ex. `pfu_2026.json`) : la fiscalité change chaque année. Afficher l'année de référence du calcul.
- Ne **jamais** affirmer « vous paierez X € » → toujours « à titre indicatif, selon les règles 2026, l'imposition serait de l'ordre de X € ».
- Cas non couverts (quotient familial, TMI réelle, plus-values de cession d'entreprise, etc.) explicitement signalés comme hors périmètre.
- Lien « pourquoi ce chiffre ? » → détail de la formule + source officielle.

**Comment ça reste simple :** quand l'utilisateur place une position dans une enveloppe (PEA/CTO/AV — le champ `accounts.type` existe déjà avec « PEA, CTO, AV, wallet »), Pécule applique automatiquement le bon régime et affiche un repère « si tu vendais aujourd'hui, impôt estimé ≈ … ». Pas de formulaire fiscal : la règle découle de l'enveloppe choisie.

---

### 7. Stratégie de tests contre valeurs de référence & garde-fous

**Tests (le standard est déjà excellent dans `finance/`, à généraliser).**
- Chaque moteur garde sa batterie de **valeurs de référence calculées à la main**, documentées en commentaire avec le détail du calcul (comme `loan.rs` : « M = 200000·0.0025/(1−1.0025^−240) = 1109.195196 »). C'est le modèle à imiter pour TWR, XIRR, CUMP/FIFO, TAEG, fiscalité.
- **Tolérances explicites** : `approx_money` (±0,01 € sur les montants) et `approx` (1e-6 sur les ratios) — pattern déjà en place, à réutiliser.
- **Tests croisés contre des références externes** : XIRR vs la fonction `XIRR` d'Excel/LibreOffice sur jeux connus ; TAEG vs exemples réglementaires ; amortissement vs simulateurs bancaires publics. Documenter la source de chaque référence.
- **Tests d'invariants** (déjà présents : « somme des parts de capital = principal », « interest == value − contributed ») → ajouter : « somme des +/- values FIFO sur lots = +value CUMP totale à liquidation complète », « TWR chaîné = ROI simple quand un seul flux », « valeur réelle ≤ valeur nominale si inflation > 0 ».
- **Cas limites systématiques** (déjà la norme : taux 0, durée 0, principal 0, n=0) → étendre : devise manquante, flux à somme nulle (XIRR non défini), split de ratio 0, dividende > capital.
- **Property-based testing** (proptest) sur les moteurs purs : ex. « FV croît avec le taux », « PRU FIFO après tout liquider = coût total / quantité totale », pour attraper les régressions sur les chemins non testés à la main.
- **Tests de non-régression d'arrondi** : vérifier que la somme des lignes affichées (arrondies) reconverge vers le total non arrondi affiché.

**Garde-fous & avertissements (non-conseil).**
- Composant `Disclaimer.svelte` obligatoire sur tout écran produisant un chiffre prospectif ou fiscal : « Pécule est un outil de simulation et de suivi. Ce n'est pas un conseil en investissement ni un conseil fiscal. »
- Distinction visuelle nette entre **données réelles saisies** (faits) et **projections/estimations** (hypothèses) — couleur/icône différente, jamais confondues.
- Quand une donnée manque (FX historique, frais, valeur intermédiaire pour TWR), **dégrader honnêtement** avec mention « estimation » plutôt que d'inventer.
- Annualisation, extrapolation, optimisme : interdits par défaut, toujours opt-in et étiquetés.

**Comment ça reste simple :** tout ce filet de sécurité est invisible pour l'utilisateur normal — il voit des chiffres justes et un petit « (i) ». Le disclaimer est discret mais toujours présent. La rigueur protège l'utilisateur sans l'alourdir : c'est exactement la promesse « simple par défaut, puissance (et honnêteté) à la demande ».

---

### Définitions clés (glossaire de la lentille)

- **PRU / CUMP** — Prix de revient unitaire = coût moyen pondéré : `(Σ qté×prix + frais) / Σ qté`. Base de coût par défaut en France.
- **FIFO** — Premier entré, premier sorti : on solde les lots dans l'ordre d'achat. Option d'analyse, **pas** la base fiscale française par défaut.
- **ROI (rendement simple)** — `(valeur − coût)/coût`. Juste seulement sans flux multiples.
- **TWR** — Rendement chaîné géométriquement, neutre aux versements : `∏(1+r_k) − 1`. Pour juger une stratégie / comparer à un indice.
- **MWR / XIRR** — TRI des flux datés : taux `r` tel que `Σ CF_j/(1+r)^(Δt_j) = 0`. Le rendement *réellement vécu*.
- **Taux nominal vs effectif** — `effectif = (1+nominal/n)^n − 1`. Effet de la fréquence de capitalisation.
- **Taux réel (Fisher)** — `(1+réel) = (1+nominal)/(1+inflation)`. Pour raisonner en euros constants.
- **TAEG** — Coût total réel d'un crédit (intérêts + assurance + frais) exprimé en taux ; un TRI des flux du prêt. La seule base de comparaison honnête entre offres.
- **CI vs CRD (assurance)** — prime sur capital initial (constante) vs capital restant dû (dégressive).
- **Price return vs total return** — performance hors dividendes vs dividendes inclus.
- **Withholding tax** — retenue à la source sur dividendes étrangers, potentiellement créditable.
- **PFU** — Prélèvement forfaitaire unique 30 % (12,8 % IR + 17,2 % PS) sur la plupart des revenus mobiliers du CTO.

> **Fichiers concernés (chemins absolus).** Moteurs à enrichir/ajouter sous `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/finance/` (`compound.rs`, `savings.rs`, `loan.rs` existants ; à créer : `performance.rs` pour TWR/XIRR, `cost_basis.rs` pour CUMP/FIFO/corporate actions, `tax.rs` pour les repères fiscaux versionnés). Schéma à étendre dans `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/db.rs` (exploiter `transactions`/`dividends` déjà créées, ajouter `corporate_actions`, `fx_rates`, `tax_rules`). Branchement IPC via `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src-tauri/src/commands/portfolio.rs` et affichage dans `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src/routes/portefeuille/+page.svelte`. Disclaimer via le composant existant `/Users/pierreherbert/Documents/SAAS/Finance/pecule/src/lib/components/Disclaimer.svelte`.

---

## 5. Roadmap phasée, priorisation, métriques & anti‑objectifs


> **Point de départ réel (vérifié dans le code)** : les moteurs financiers (`finance/` : compound, savings, loan) et « la courbe » sont livrés et testés ; le portefeuille a un CRUD de *holdings* câblé (`commands::portfolio`) **mais ni cours, ni transactions, ni dividendes en UI** (le schéma SQLite `transactions`/`dividends`/`subscriptions`/`accounts` existe déjà mais reste muet) ; `quotes.rs`, `alerts.rs`, `crypto.rs` sont des **stubs vides** ; **pas de palette ⌘K, pas d'onboarding**. Le séquencement ci-dessous exploite ce socle : beaucoup de valeur est déjà « pré-câblée » côté base, il s'agit surtout de la rendre visible et facile.

---

### Principe de priorisation (la boussole à chaque arbitrage)

À chaque étape, on choisit le plus grand **valeur ÷ effort** qui **préserve la simplicité** (jamais l'inverse). Filtre de décision en 4 questions, dans l'ordre :

1. **Est-ce activable par défaut sans rien régler ?** Sinon, ça se cache derrière la divulgation progressive (un repli « Options avancées », un panneau Réglages, une commande ⌘K) — ce n'est *pas* un motif de rejet, c'est un motif de **rangement**.
2. **Est-ce que ça rend le reste plus facile ?** (palette ⌘K, onboarding, import) → priorité maximale, on les met **tôt**.
3. **Est-ce local-first sans compromis ?** Si non → reporté ou refusé (cf. anti-objectifs).
4. **Effort solo réaliste ?** On préfère 3 features finies et polies à 8 features à moitié.

Règle d'or anti-complexité : **chaque nouvelle feature doit avoir un défaut, une cachette, et un raccourci.** Si on ne peut pas répondre aux trois, la feature n'est pas prête.

---

### Quick wins immédiats (les 2–4 prochaines semaines)

Avant même v0.3, des gains très élevés / effort faible, qui s'appuient sur l'existant :

| Quick win | Pourquoi (valeur ÷ effort) | Comment ça reste simple |
|---|---|---|
| **Sauvegarder / charger une simulation** | Le schéma `simulations` existe **déjà**, le code de persistance manque juste. Transforme la courbe d'un jouet en outil revisitable. | Bouton « Enregistrer ce scénario » discret ; liste rappelée via ⌘K plus tard. Aucun réglage. |
| **Comparer 2–3 scénarios sur la même courbe** | Réutilise le moteur déjà testé, zéro backend nouveau. C'est *le* moment « waouh » de l'élément signature. | Par défaut : 1 courbe. Bouton « + comparer » qui dédouble les contrôles. Caché tant qu'on ne clique pas. |
| **Export / import JSON de toutes les données** (pas juste le portefeuille) | Rassurance RGPD + filet de sécurité avant tout le reste. Faible effort (sérialiser les tables existantes). | Réglages → « Mes données » → Exporter / Importer. Un fichier, lisible. |
| **Raccourcis clavier de base + focus visible** | Prépare le terrain de la palette, améliore l'accessibilité tout de suite. | `?` ouvre l'aide des raccourcis. Rien à configurer. |
| **États vides soignés** sur Portefeuille / Abonnements | Aujourd'hui « esquissés » = vides moches. Un bon état vide *est* un mini-onboarding gratuit. | Texte + 1 bouton d'action + 1 exemple « essayer avec des données fictives ». |

Critère de sortie des quick wins : on peut **créer, enregistrer, comparer et exporter** un scénario sans toucher la souris plus que nécessaire, et aucun écran n'est « vide et mort ».

---

### Roadmap versionnée

#### v0.3 — « Tout au clavier + premier contact » *(socle de facilité — priorité absolue)*
**Thème** : rendre le reste de la roadmap *facile* en posant les deux fondations transversales.
- **Palette de commandes ⌘K** : recherche universelle (naviguer, lancer un simulateur, ajouter une position, basculer le thème, exporter…). C'est l'« infinité de fonctionnalités » sans menus infinis.
- **Onboarding 60 secondes** : 3 écrans max (Bienvenue → « vos données restent chez vous » → « voici la courbe, bougez le curseur »). Skippable, jamais bloquant.
- **Abonnements (vrai)** : la table `subscriptions` existe → CRUD + total mensuel/annuel + injection dans « la courbe » (impact d'un abonnement résilié sur 10 ans).
- **Tour de la courbe** : tooltip contextuel la première fois qu'on touche le curseur.

*Comment ça reste simple* : ⌘K = **une** touche, fonctionne partout, ferme à Échap ; l'onboarding se relance via ⌘K « Revoir l'introduction » et nulle part ailleurs. Abonnements : on tape juste un nom + un montant, le reste a des défauts (mensuel, actif).
**Critère de sortie** : un nouvel utilisateur installe, comprend le local-first, ajoute un abonnement et bouge la courbe **sans aide externe**, et tout est atteignable au clavier via ⌘K.

#### v0.4 — « Le portefeuille vivant »
**Thème** : passer du portefeuille statique (valorisation manuelle) au portefeuille **réel**.
- **Comptes** (`accounts` existe) : PEA / CTO / AV / wallet — regrouper les positions.
- **Transactions** (`transactions` existe) : achats/ventes → **PRU et +/- value calculés**, plus de saisie manuelle du prix de revient.
- **Dividendes** (`dividends` existe) : suivi + rendement, intégrables à la courbe.
- **Import CSV** (relevé courtier / Boursorama / Trade Republic) avec mapping de colonnes mémorisé.

*Comment ça reste simple* : par défaut, une seule « poche » ; les comptes apparaissent seulement quand on en crée un 2ᵉ. La saisie manuelle d'un PRU reste possible (mode simple) ; les transactions sont le mode « puissance à la demande ». L'import devine les colonnes, l'utilisateur corrige une fois.
**Critère de sortie** : on importe un CSV de courtier et on obtient un portefeuille juste (PRU, +/- value, dividendes) **sans ressaisir** position par position.

#### v0.5 — « Cours publics, opt-in »
**Thème** : activer `quotes.rs` (aujourd'hui stub) **sans trahir le local-first**.
- **Rafraîchissement de cours publics**, **désactivé par défaut**, opt-in explicite avec rappel « seuls des prix entrent, vos positions ne sortent jamais ».
- **Valorisation automatique** + historique de valeur du portefeuille (nouvelle table d'historique).
- **Alertes** (`alerts.rs`, stub) : seuils locaux (« préviens-moi si X passe sous Y »), notifications natives Tauri.

*Comment ça reste simple* : un seul interrupteur dans Réglages → « Cours publics : Désactivé ». Tant qu'il est off, **aucune requête réseau** (la CSP stricte le garantit déjà). Les alertes se créent depuis la ligne d'une position (clic droit / ⌘K), pas dans un écran à part.
**Critère de sortie** : interrupteur off = zéro octet réseau (vérifiable) ; interrupteur on = valorisation à jour ; une alerte se déclenche correctement hors-ligne sur la dernière valeur connue.

#### v0.6 — « Vue d'ensemble & patrimoine »
**Thème** : le **tableau de bord** qui réunit les trois piliers.
- **Patrimoine net** (positions + comptes + autres actifs/dettes saisis manuellement).
- **Courbe unifiée** : épargne projetée + portefeuille + impact abonnements sur **une** trajectoire.
- **Catégories de dépenses** légères (pour les abonnements et apports récurrents).

*Comment ça reste simple* : le tableau de bord montre **3 chiffres** par défaut (patrimoine, épargne mensuelle, dans 10 ans) ; tout le détail est en dessous, replié.
**Critère de sortie** : en ouvrant l'app, on voit immédiatement « où j'en suis » et « où je vais » sans cliquer.

#### v1.0 — « Solide, accessible, distribuable »
**Thème** : la version qu'on recommande sans réserve.
- **Multi-fichiers / coffres** (séparer perso/pro, ou plusieurs personnes sur une machine).
- **Sauvegardes automatiques locales** + chiffrement au repos optionnel (réactiver `crypto.rs`).
- **Accessibilité complète** (clavier, lecteurs d'écran, contraste), **i18n EN finalisé**.
- **Builds signés** macOS / Windows / Linux + auto-update **opt-in**.
- **Documentation utilisateur** intégrée (cherchable via ⌘K).

*Comment ça reste simple* : le chiffrement est **proposé**, pas imposé (mot de passe = perte de données possible, donc opt-in clair). Sauvegardes silencieuses, restaurables en 1 clic.
**Critère de sortie** : un débutant total installe une version signée, est productif en < 5 min, et toutes les actions sont au clavier, traduites, accessibles.

#### v2.0 — « Puissance à la demande »
**Thème** : la profondeur pour les utilisateurs avancés, **invisible** pour les autres.
- **Champs/calculs personnalisés**, **règles** (catégorisation auto), **objectifs** (épargne cible, FIRE, apport immo).
- **Multi-devises** propre (taux saisis manuellement, ou via cours opt-in).
- **Scénarios « et si » avancés** sur la courbe (inflation, fiscalité PEA/AV/flat tax FR).
- **Rapports exportables** (PDF/CSV) — patrimoine, plus-values pour la déclaration.

*Comment ça reste simple* : tout vit derrière ⌘K et des panneaux « avancé » repliés. Le calcul fiscal a des **défauts France** (flat tax 30 %, abattements PEA) qu'on n'est jamais obligé d'éditer.
**Critère de sortie** : un utilisateur avancé modélise un projet immo avec fiscalité FR sans quitter l'app ; un débutant ne croise *jamais* ces options par accident.

#### v3.0 — « Extensible & écosystème (local) »
**Thème** : l'« app infinie » au sens propre, sans serveur.
- **Modèles/templates de scénarios** partageables (fichiers, pas de cloud).
- **Plugins/extensions locaux** sandboxés (nouveaux calculs, importeurs de courtiers).
- **Connecteurs d'import** communautaires (formats de relevés), maintenus en open source.

*Comment ça reste simple* : un plugin = un fichier qu'on dépose ; il ajoute des entrées dans ⌘K, jamais des boutons imposés dans l'UI principale. Aucun plugin actif par défaut.
**Critère de sortie** : un tiers publie un importeur de courtier que l'utilisateur installe localement, sans que l'app de base devienne plus complexe pour qui ne l'installe pas.

#### vNext — « Bancaire connecté, strictement optionnel »
**Thème** : la seule grande feature à manier avec des pincettes.
- **Agrégation bancaire opt-in** via standard ouvert (ex. DSP2 / open banking européen), **jamais par défaut**, avec consentement granulaire et possibilité de tout supprimer localement.
- **Synchronisation chiffrée de bout en bout entre *ses propres* appareils** (jamais via un compte Pécule).

*Comment ça reste simple* : l'app reste **100 % utilisable et complète sans jamais connecter de banque**. Le connecteur est une porte, pas un couloir obligatoire.
**Critère de sortie** : un utilisateur qui refuse toute connexion bancaire a exactement la même app pleine et fonctionnelle ; celui qui l'active peut tout révoquer et purger en 1 action.

---

### Métriques de succès (sans télémétrie — qualitatives et vérifiables)

On ne mesure rien sur les machines des utilisateurs. On vérifie autrement :

- **Test des 5 minutes** (sur 5 personnes non-tech, en personne) : installer → comprendre le local-first → faire une projection → ajouter un abonnement, **sans poser de question**. Critère de sortie de chaque version.
- **« Tout au clavier »** : checklist interne — toute action de la version est atteignable via ⌘K + Échap ferme tout. Auditable.
- **Zéro réseau prouvable** : capture réseau (Wireshark / Little Snitch) montrant **0 connexion** tant que « cours publics » est off. C'est *la* promesse, on la prouve à chaque release.
- **Qualité visible** : `cargo test` vert (moteurs financiers vérifiés contre formules fermées), `svelte-check` sans erreur, lint propre.
- **Adoption open source** (signaux publics, pas privés) : étoiles GitHub, issues, paquets communautaires (Homebrew, AUR, Flathub), avis sur forums FI/RGPD. Indicatif, pas pilotant.
- **Régression de simplicité = bug** : si une feature ajoute une étape obligatoire à un parcours existant, c'est traité comme un défaut.

---

### Anti-objectifs (ce que Pécule ne fera JAMAIS)

- **Aucune publicité**, jamais, sous aucune forme (ni « partenaires », ni « offres »).
- **Aucune revente, partage ou collecte de données** ; **aucune télémétrie** même « anonyme ».
- **Aucun compte obligatoire** ni cloud propriétaire : pas de « connectez-vous pour commencer ».
- **Bancaire connecté jamais imposé** : l'app est complète sans aucune banque liée ; la connexion reste opt-in, révocable, purgeable localement.
- **Aucun dark pattern** : pas de urgence factice, pas de désabonnement caché, pas de paywall sur les données *de l'utilisateur*, pas de notifications manipulatrices.
- **Aucune complexité imposée** : pas d'écran de config obligatoire, pas de jargon non expliqué, pas de feature avancée qui s'impose à qui n'en veut pas.
- **Aucun conseil en investissement déguisé** : Pécule reste pédagogique et de suivi (cf. avertissement README).
- **Pas de réseau silencieux** : aucune requête sortante hors du seul interrupteur « cours publics » explicitement activé.

---

### Risques principaux & parades

| Risque | Pourquoi c'est sérieux | Parade |
|---|---|---|
| **Scope solo (épuisement, dérive)** | « App infinie » + fondateur seul = risque n°1. | Découpage en versions à **critère de sortie binaire** ; règle « 3 features finies > 8 ébauches » ; quick wins qui livrent de la valeur tous les 15 jours ; refuser activement (anti-objectifs comme bouclier anti-scope). |
| **Exactitude des calculs** (un chiffre faux ruine la confiance) | C'est un outil d'argent ; une erreur de PRU ou de fiscalité = perte de crédibilité immédiate. | Garder tous les moteurs en **Rust pur testé** contre valeurs de référence ; tests obligatoires avant merge sur tout calcul ; avertissement pédagogique maintenu ; afficher les hypothèses (taux, fiscalité) à côté des résultats. |
| **Vie privée vs cours publics** | La seule entorse réseau est aussi la plus grosse promesse à tenir. | Off par défaut ; CSP stricte ; **prix entrent, positions ne sortent jamais** (jamais le symbole+quantité dans une même requête identifiante) ; preuve réseau à chaque release. |
| **Import de courtiers fragile** (formats qui changent) | Mauvais import = mauvaises données = défiance. | Mapping de colonnes éditable + aperçu avant validation ; importeurs en open source maintenables par la communauté (v3.0) ; toujours garder la saisie manuelle en repli. |
| **Complexité qui s'accumule version après version** | Chaque feature ajoutée grignote la simplicité. | Discipline « défaut + cachette + raccourci » ; le tableau de bord n'expose que 3 chiffres ; ⌘K absorbe la découvrabilité au lieu de menus ; test des 5 minutes à chaque version. |
| **Perte de données locales** (pas de cloud = pas de filet) | Local-first transfère la responsabilité de sauvegarde à l'utilisateur. | Export/import JSON dès les quick wins ; sauvegardes auto locales (v1.0) ; chiffrement au repos opt-in avec avertissement clair sur la perte de mot de passe. |
| **Signature / distribution multi-OS** | Builds non signés = friction d'install, alertes OS. | Builds signés dès v1.0 ; canaux natifs (Homebrew, Flathub) ; auto-update opt-in. |

---

### Par quoi commencer, concrètement, dans l'ordre

1. **Quick wins** (semaines qui viennent) : enregistrer/charger un scénario (table `simulations` déjà là), comparer 2 courbes, export/import JSON global, états vides soignés, raccourcis de base. → valeur immédiate, zéro backend lourd.
2. **v0.3** : **⌘K d'abord** (il rend tout le reste découvrable), puis **onboarding**, puis les **abonnements réels** (table déjà présente).
3. **v0.4** : activer la chaîne **comptes → transactions → dividendes** (tables déjà présentes, c'est surtout de l'UI + commandes), avec **import CSV**.
4. Ensuite seulement **cours publics opt-in** (v0.5), car ils n'ont de sens qu'avec un portefeuille réel à valoriser.

Le fil conducteur : **on rend visible et facile ce que le schéma de base anticipe déjà**, dans l'ordre qui maximise valeur ÷ effort tout en posant tôt les deux multiplicateurs de simplicité (⌘K + onboarding).

---

> Ce document est vivant : il évolue à chaque version. Le cahier de conception d'origine reste la référence de marque/identité. Avertissement : Pécule est un outil pédagogique — **aucun conseil en investissement**.
