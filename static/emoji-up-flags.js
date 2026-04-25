function flagEmoji(cc) {
    if (cc === 'ZZ') return '🏳️';
    const codePoints = cc
        .split('')
        .map((char) => 127397 + char.charCodeAt(0))
    return String.fromCodePoint(...codePoints)
}

document.querySelectorAll('.cc').forEach(el => {
  el.textContent = flagEmoji(el.textContent.trim());
});
