//! Batched phonetic dict: two single-pass batches (proven identical to the
//! historical sequential application on the 30M wiki dump and the full 2GB
//! dump); the sequential tail is empty. The cut between 3 and 4 preserves
//! the `[жш]ц…` → `здж` order-vs-position cascade (`Нозджцы `).
crate::dict_batches! {
    pub PHONETIC:
    batch [ // [0..4) (4)
        (r"сш", r"шш"),
        (r"[зс]ч", r"шч"),
        (r"чц([^ьіеюя])", r"цц$1"),
        (r"[жш]ц([ауы] |а[хўйм] |амі )", r"сц$1"),
    ],
    batch [ // [4..8) (4)
        (r"шся ", r"сься "),
        (r"здж", r"ждж"),
        (r" ([бд]|кнд|нот)р ", " $1р "),
        (r"(\S\S[дт])р ", r"$1ар "),
    ],
    sequential [ // SEQ [8..8) (empty)
    ],
}
