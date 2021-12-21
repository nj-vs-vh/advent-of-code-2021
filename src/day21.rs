const MOVES: [u8; 7] = [3, 4, 5, 6, 7, 8, 9];
const MOVE_UNIVERSES: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

const END_SCORE: u8 = 21;

fn whowins(
    idx1: u8,
    idx2: u8,
    score1: u8,
    score2: u8,
    universes: u64,
    first_moves: bool,
) -> (u64, u64) {
    if score1 >= END_SCORE {
        return (universes, 0);
    } else if score2 >= END_SCORE {
        return (0, universes);
    }

    let (mut universes_win1, mut universes_win2) = (0, 0);

    let mut idx1_next = idx1;
    let mut idx2_next = idx2;
    let mut score1_next = score1;
    let mut score2_next = score2;
    for i_move in 0..7 {
        if first_moves {
            idx1_next = (idx1 + MOVES[i_move]) % 10;
            score1_next = score1 + idx1_next + 1 // idx is 0-based while numbering for scores is 1-based
        } else {
            idx2_next = (idx2 + MOVES[i_move]) % 10;
            score2_next = score2 + idx2_next + 1
        }
        let (uw1_mi, uw2_mi) = whowins(
            idx1_next,
            idx2_next,
            score1_next,
            score2_next,
            universes * MOVE_UNIVERSES[i_move],
            !first_moves,
        );
        universes_win1 += uw1_mi;
        universes_win2 += uw2_mi;
    }

    (universes_win1, universes_win2)
}

pub fn dirac_die() {
    let pos1 = 10;
    let pos2 = 2;

    let (un1, un2) = whowins(pos1 - 1, pos2 - 1, 0, 0, 1, true);
    println!(
        "1 wins in {}, 2 wins in {} (total of {})",
        un1,
        un2,
        un1 + un2
    );
}
