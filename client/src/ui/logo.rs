// SPDX-FileCopyrightText: 2026 SamosGames
// SPDX-License-Identifier: AGPL-3.0-or-later

use yew::{html, Html};

pub fn logo() -> Html {
    html! {
        <div style="display:grid; gap:.35rem; padding:1.2rem 1.6rem; border:1px solid rgba(255,255,255,.35); border-radius:.8rem; background:linear-gradient(135deg,rgba(6,27,53,.92),rgba(10,130,162,.78)); box-shadow:0 .8rem 3rem rgba(0,0,0,.35); color:white; font-family:system-ui,sans-serif; text-align:center; text-shadow:0 2px 12px #061b35;">
            <strong style="font-size:clamp(2rem,7vw,5rem); letter-spacing:.08em;">{"ФЛОТ ОНЛАЙН"}</strong>
            <span style="color:#ffd166; font-size:clamp(.75rem,2vw,1.1rem); letter-spacing:.3em;">{"МОРСКОЙ БОЙ ОТ SAMOSGAMES"}</span>
        </div>
    }
}
