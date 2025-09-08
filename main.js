/*
 * main.js - the frontend glue code for counterjs
 * Copyright (C) 2025 pastaya
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import init, {
  increment_counter,
  init_counter,
  reset_counter,
  set_counter
} from "./counterrs/pkg/counterrs.js";

async function main() {
  await init();

  const button = document.getElementById("nyabtn");
  const counter = document.getElementById("counter");

  counter.textContent = init_counter();

  // we export functions so it can be run in console
  window.set_counter = set_counter;
  window.reset_counter = reset_counter;
  window.increment_counter = increment_counter;
  window.init_counter = init_counter;

  let lastClick = 0;

  // increment on click (hopefully)
  button.addEventListener("click", () => {
    const now = Date.now();
    if (now - lastClick < 100) return; // HACK: very hacky way of "anticheat" and "debounce prevention"
    lastClick = now;

    counter.textContent = increment_counter();
  });
}

main();
