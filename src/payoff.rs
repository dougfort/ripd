use crate::ipd::Action;

/// For payoff https://en.wikipedia.org/wiki/Prisoner's_dilemma
///
/// If both players cooperate, they both receive the reward R for cooperating.
/// If both players defect, they both receive the punishment payoff P.
/// If Blue defects while Red cooperates, then Blue receives the temptation payoff T, while Red receives the "sucker's" payoff, S.
/// Similarly, if Blue cooperates while Red defects, then Blue receives the sucker's payoff S, while Red receives the temptation payoff T.
///
/// T > R > P > S
/// We want 2R > T + S for the iterative game
///

pub fn compute_payoff(red: Action, blue: Action) -> (u32, u32) {
    const R: u32 = 3;
    const T: u32 = 4;
    const P: u32 = 2;
    const S: u32 = 1;

    match (red, blue) {
        (Action::Cooperate, Action::Cooperate) => (R, R),
        (Action::Defect, Action::Defect) => (P, P),
        (Action::Defect, Action::Cooperate) => (S, T),
        (Action::Cooperate, Action::Defect) => (T, S),
        (_, _) => (0, 0),
    }
}
