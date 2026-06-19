//! Prêt amortissable à **mensualités constantes** (système d'amortissement
//! classique français), avec assurance optionnelle sur le capital initial.
//!
//! Moteur **pur** : aucune I/O, aucun état global, déterministe et testable
//! contre des valeurs de référence calculées à la main.

use serde::{Deserialize, Serialize};

/// Paramètres du prêt.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanParams {
    /// Capital emprunté.
    pub principal: f64,
    /// Taux nominal annuel du crédit, décimal (3 % => `0.03`).
    pub annual_rate: f64,
    /// Durée du prêt, en **mois**.
    pub term_months: u32,
    /// Taux annuel d'assurance appliqué au **capital initial** (0 si aucune).
    /// Convention française usuelle : prime mensuelle constante
    /// `= principal · insurance_annual_rate / 12`.
    pub insurance_annual_rate: f64,
}

/// Une ligne du tableau d'amortissement (un mois).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmortRow {
    /// Numéro du mois, de 1 à `term_months`.
    pub month: u32,
    /// Mensualité hors assurance (constante).
    pub payment: f64,
    /// Part de capital remboursée ce mois.
    pub principal_part: f64,
    /// Part d'intérêts payée ce mois.
    pub interest_part: f64,
    /// Prime d'assurance du mois.
    pub insurance: f64,
    /// Capital restant dû après ce mois.
    pub balance: f64,
}

/// Résultat du prêt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanResult {
    /// Mensualité hors assurance.
    pub monthly_payment: f64,
    /// Prime d'assurance mensuelle.
    pub monthly_insurance: f64,
    /// Mensualité totale (`monthly_payment + monthly_insurance`).
    pub monthly_total: f64,
    /// Total des intérêts sur la durée.
    pub total_interest: f64,
    /// Total de l'assurance sur la durée.
    pub total_insurance: f64,
    /// Coût total payé (`principal + total_interest + total_insurance`).
    pub total_cost: f64,
    /// Tableau d'amortissement complet (une ligne par mois).
    pub schedule: Vec<AmortRow>,
}

/// Calcule la mensualité hors assurance.
///
/// `M = P · i / (1 − (1+i)^(−N))`, avec le cas limite `i = 0` donnant `M = P/N`.
///
/// Renvoie `0.0` si la durée `n` est nulle (prêt vide) afin d'éviter toute
/// division par zéro ; ce cas dégénéré est géré explicitement par [`loan`].
fn monthly_payment(principal: f64, monthly_rate: f64, n: u32) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if monthly_rate == 0.0 {
        return principal / f64::from(n);
    }
    let factor = 1.0 - (1.0 + monthly_rate).powi(-(n as i32));
    principal * monthly_rate / factor
}

/// Calcule un prêt amortissable à mensualités constantes.
///
/// Taux mensuel `i = annual_rate / 12`, `N = term_months`.
/// Mensualité (hors assurance) : `M = P · i / (1 − (1+i)^(−N))`.
/// Cas limite `i = 0` : `M = P / N`.
/// Assurance : prime mensuelle constante `= principal · insurance_annual_rate / 12`.
///
/// Le tableau d'amortissement applique chaque mois :
/// `interest_part = balance · i`, `principal_part = M − interest_part`,
/// `balance -= principal_part`. La dernière ligne solde exactement le capital
/// (l'arrondi résiduel est reporté sur le `principal_part` et le `payment` du
/// dernier mois).
pub fn loan(params: &LoanParams) -> LoanResult {
    let n = params.term_months;
    let monthly_rate = params.annual_rate / 12.0;
    let base_payment = monthly_payment(params.principal, monthly_rate, n);

    // Prime d'assurance mensuelle constante, assise sur le capital INITIAL.
    let monthly_insurance = params.principal * params.insurance_annual_rate / 12.0;

    // Cas dégénéré : durée nulle => aucun flux, aucun coût.
    if n == 0 {
        return LoanResult {
            monthly_payment: 0.0,
            monthly_insurance,
            monthly_total: monthly_insurance,
            total_interest: 0.0,
            total_insurance: 0.0,
            total_cost: params.principal,
            schedule: Vec::new(),
        };
    }

    let mut schedule = Vec::with_capacity(n as usize);
    let mut balance = params.principal;
    let mut total_interest = 0.0;

    for month in 1..=n {
        let interest_part = balance * monthly_rate;

        let (payment, principal_part) = if month == n {
            // Dernier mois : solder exactement le capital restant. On rembourse
            // tout le `balance` résiduel ; le paiement s'ajuste à l'arrondi près
            // (intérêts inclus) pour absorber l'accumulation des erreurs f64.
            let principal_part = balance;
            (principal_part + interest_part, principal_part)
        } else {
            (base_payment, base_payment - interest_part)
        };

        balance -= principal_part;
        total_interest += interest_part;

        // Forcer le solde final à zéro exact (et non un -1e-9 résiduel).
        if month == n {
            balance = 0.0;
        }

        schedule.push(AmortRow {
            month,
            payment,
            principal_part,
            interest_part,
            insurance: monthly_insurance,
            balance,
        });
    }

    let total_insurance = monthly_insurance * f64::from(n);
    let total_cost = params.principal + total_interest + total_insurance;

    LoanResult {
        monthly_payment: base_payment,
        monthly_insurance,
        monthly_total: base_payment + monthly_insurance,
        total_interest,
        total_insurance,
        total_cost,
        schedule,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tolérance relative serrée pour les ratios / mensualités (sans dimension).
    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    /// Tolérance absolue pour les montants exprimés en euros.
    fn approx_eur(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn nominal_loan() {
        // Cas nominal réaliste : P = 200 000 €, r = 3 %/an, N = 240 mois (20 ans).
        // i = 0.03 / 12 = 0.0025.
        // M = P·i / (1 − (1+i)^(−N))
        //   = 200000·0.0025 / (1 − 1.0025^(−240))
        //   = 500 / (1 − 0.5487754…) = 1109.195196… € (cf. repère ≈ 1109.20).
        let result = loan(&LoanParams {
            principal: 200_000.0,
            annual_rate: 0.03,
            term_months: 240,
            insurance_annual_rate: 0.0,
        });

        assert!(
            approx_eur(result.monthly_payment, 1109.195196),
            "M attendu ≈ 1109.195196, obtenu {}",
            result.monthly_payment
        );
        // Sans assurance, le total mensuel égale la mensualité.
        assert!(approx_eur(result.monthly_insurance, 0.0));
        assert!(approx_eur(result.monthly_total, result.monthly_payment));

        // Le tableau a exactement N lignes, numérotées 1..=240.
        assert_eq!(result.schedule.len(), 240);
        assert_eq!(result.schedule[0].month, 1);
        assert_eq!(result.schedule[239].month, 240);

        // Premier mois : interest = balance·i = 200000·0.0025 = 500.00 €,
        // principal_part = M − 500 = 1109.195196 − 500 = 609.195196 €.
        let first = &result.schedule[0];
        assert!(approx_eur(first.interest_part, 500.0), "intérêts m1");
        assert!(approx_eur(first.principal_part, 609.195196), "capital m1");

        // Le capital est soldé EXACTEMENT au dernier mois.
        assert_eq!(result.schedule[239].balance, 0.0);

        // total_interest = (somme des mensualités) − principal
        //   ≈ 1109.195196·240 − 200000 = 266206.847 − 200000 = 66206.847 €
        //   (calculé à la main par sommation du tableau : 66206.84697 €).
        assert!(
            approx_eur(result.total_interest, 66206.84697),
            "total_interest attendu ≈ 66206.85, obtenu {}",
            result.total_interest
        );

        // Pas d'assurance => total_insurance = 0.
        assert!(approx_eur(result.total_insurance, 0.0));

        // total_cost = principal + total_interest + total_insurance
        //   = 200000 + 66206.84697 + 0 = 266206.84697 €.
        assert!(approx_eur(result.total_cost, 266206.84697));

        // Invariant : la somme des parts de capital rembourse tout le principal.
        let sum_principal: f64 = result.schedule.iter().map(|r| r.principal_part).sum();
        assert!(approx_eur(sum_principal, 200_000.0));
    }

    #[test]
    fn zero_rate_loan() {
        // Cas limite r = 0 : aucun intérêt, M = P / N.
        // P = 12 000 €, N = 12 => M = 12000 / 12 = 1000.00 €.
        let result = loan(&LoanParams {
            principal: 12_000.0,
            annual_rate: 0.0,
            term_months: 12,
            insurance_annual_rate: 0.0,
        });

        assert!(approx(result.monthly_payment, 1000.0), "M = P/N = 1000");
        // total_interest = 0 (aucun intérêt quand le taux est nul).
        assert!(approx_eur(result.total_interest, 0.0));
        assert!(approx_eur(result.total_insurance, 0.0));
        // total_cost = principal exactement (pas d'intérêts ni d'assurance).
        assert!(approx_eur(result.total_cost, 12_000.0));

        assert_eq!(result.schedule.len(), 12);
        // Chaque mois rembourse 1000 € de capital, 0 € d'intérêts.
        for (idx, row) in result.schedule.iter().enumerate() {
            assert!(approx_eur(row.interest_part, 0.0), "intérêts nuls mois {idx}");
            assert!(approx_eur(row.principal_part, 1000.0), "capital 1000 mois {idx}");
            assert!(approx_eur(row.insurance, 0.0));
        }
        // Solde final exactement nul.
        assert_eq!(result.schedule[11].balance, 0.0);
        // Solde après le 1er mois : 12000 − 1000 = 11000 €.
        assert!(approx_eur(result.schedule[0].balance, 11_000.0));
    }

    #[test]
    fn small_exact_schedule_with_insurance() {
        // Cas limite court entièrement vérifiable à la main, AVEC assurance.
        // P = 1000 €, r = 12 %/an => i = 0.12/12 = 0.01, N = 2 mois.
        // M = P·i / (1 − (1+i)^(−N))
        //   = 1000·0.01 / (1 − 1.01^(−2))
        //   = 10 / (1 − 0.98029605…) = 10 / 0.01970395… = 507.512438… €.
        //
        // Assurance : insurance_annual_rate = 0.0036 (0,36 %/an).
        // prime mensuelle = 1000·0.0036/12 = 0.30 € (constante, capital INITIAL).
        let result = loan(&LoanParams {
            principal: 1000.0,
            annual_rate: 0.12,
            term_months: 2,
            insurance_annual_rate: 0.0036,
        });

        assert!(
            approx_eur(result.monthly_payment, 507.512438),
            "M attendu ≈ 507.512438, obtenu {}",
            result.monthly_payment
        );
        assert!(approx_eur(result.monthly_insurance, 0.30), "prime mensuelle");
        // total mensuel = M + prime = 507.512438 + 0.30 = 507.812438 €.
        assert!(approx_eur(result.monthly_total, 507.812438));

        assert_eq!(result.schedule.len(), 2);

        // Mois 1 : interest = 1000·0.01 = 10.00 €,
        //          principal_part = M − 10 = 497.512438 €,
        //          balance = 1000 − 497.512438 = 502.487562 €,
        //          insurance = 0.30 €.
        let m1 = &result.schedule[0];
        assert_eq!(m1.month, 1);
        assert!(approx_eur(m1.interest_part, 10.0), "intérêts m1");
        assert!(approx_eur(m1.principal_part, 497.512438), "capital m1");
        assert!(approx_eur(m1.balance, 502.487562), "solde m1");
        assert!(approx_eur(m1.insurance, 0.30));

        // Mois 2 (dernier) : interest = 502.487562·0.01 = 5.024876 €,
        //          principal_part = balance résiduel = 502.487562 € (solde le prêt),
        //          balance = 0 exact.
        let m2 = &result.schedule[1];
        assert_eq!(m2.month, 2);
        assert!(approx_eur(m2.interest_part, 5.024876), "intérêts m2");
        assert!(approx_eur(m2.principal_part, 502.487562), "capital m2");
        assert_eq!(m2.balance, 0.0, "le dernier mois solde exactement le capital");

        // total_interest = 10.00 + 5.024876 = 15.024876 €.
        assert!(
            approx_eur(result.total_interest, 15.024876),
            "total_interest attendu ≈ 15.024876, obtenu {}",
            result.total_interest
        );
        // total_insurance = prime·N = 0.30·2 = 0.60 €.
        assert!(approx_eur(result.total_insurance, 0.60));
        // total_cost = 1000 + 15.024876 + 0.60 = 1015.624876 €.
        assert!(approx_eur(result.total_cost, 1015.624876));

        // Invariant : la somme des parts de capital = principal.
        let sum_principal: f64 = result.schedule.iter().map(|r| r.principal_part).sum();
        assert!(approx_eur(sum_principal, 1000.0));
    }

    #[test]
    fn zero_term_is_degenerate() {
        // Cas limite durée nulle : aucun flux. Évite toute division par zéro.
        let result = loan(&LoanParams {
            principal: 5000.0,
            annual_rate: 0.05,
            term_months: 0,
            insurance_annual_rate: 0.01,
        });
        assert!(approx_eur(result.monthly_payment, 0.0));
        assert!(approx_eur(result.total_interest, 0.0));
        assert!(approx_eur(result.total_insurance, 0.0));
        // total_cost = principal (rien n'a été remboursé ni facturé).
        assert!(approx_eur(result.total_cost, 5000.0));
        assert!(result.schedule.is_empty());
    }
}
