//! Commandes de simulation : fines enveloppes IPC autour des moteurs purs de
//! `finance`. Elles renvoient la **série complète** afin que le curseur « et dans
//! X années ? » de la courbe puisse être déplacé sans nouvel appel réseau/IPC.

use crate::finance::compound::{compound, CompoundParams, CompoundResult};
use crate::finance::loan::{loan, LoanParams, LoanResult};
use crate::finance::savings::{savings, SavingsParams, SavingsResult};

#[tauri::command]
pub fn simulate_compound(params: CompoundParams) -> CompoundResult {
    compound(&params)
}

#[tauri::command]
pub fn simulate_savings(params: SavingsParams) -> SavingsResult {
    savings(&params)
}

#[tauri::command]
pub fn simulate_loan(params: LoanParams) -> LoanResult {
    loan(&params)
}
