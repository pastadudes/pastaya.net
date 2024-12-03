let count = 0;
const button = document.getElementById('nyabtn');
const clickCountText = document.getElementById('counterNum');

button.addEventListener('click', () => {
    count++;
    clickCountText.textContent = `${count}`;
});
