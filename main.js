import init, {
  increment_counter,
  init_counter,
} from "./counterrs/pkg/counterrs.js";

async function main() {
  await init();

  const button = document.getElementById("nyabtn");
  const counter = document.getElementById("counter");

  counter.textContent = init_counter();

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
